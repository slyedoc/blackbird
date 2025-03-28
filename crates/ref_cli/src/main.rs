use reqwest::{
    blocking::{multipart, Client},
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT},
};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use tungstenite::connect;

// Change the server address if needed.
const SERVER_ADDRESS: &str = "127.0.0.1:8188";

fn main() -> Result<(), Box<dyn Error>> {
    // Create a reqwest client.
    let client = Client::new();

    // Generate a unique client id.
    let client_id = uuid::Uuid::new_v4().to_string();

    // JSON prompt text.
    let prompt_text = include_str!("workflows/ref_image_gen.json");

    // Parse the prompt JSON.
    let mut prompt: Value = serde_json::from_str(prompt_text)?;

    // // Set the text prompt for our positive CLIPTextEncode.
    // if let Some(text_value) = prompt.pointer_mut("/6/inputs/text") {
    //     *text_value = Value::String("masterpiece best quality man".to_string());
    // }

    // // Set the seed for our KSampler node.
    // if let Some(seed_value) = prompt.pointer_mut("/3/inputs/seed") {
    //     *seed_value = Value::Number(serde_json::Number::from(5));
    // }

    // Connect to the websocket.
    let ws_url = format!("ws://{}/ws?clientId={}", SERVER_ADDRESS, client_id);

    let (mut ws, _response) = connect(ws_url)?;

    // test uploading image
    //upload_image( &client, "upload_test.png".to_string())?;

    // Wait for execution to complete and download the images.
    let images = get_images(&mut ws, &client, &prompt, &client_id)?;

    ws.close(None)?;

    // For demonstration, write each image to a file.
    for (node_id, images_vec) in images.iter() {
        for (i, image_data) in images_vec.iter().enumerate() {
            let filename = format!("node_{}_image_{}.png", node_id, i);
            std::fs::write(&filename, image_data)?;
            println!("Saved image to {}", filename);
        }
    }

    Ok(())
}

/// Sends the prompt to the server and returns the JSON response.
fn queue_prompt(client: &Client, prompt: &Value, client_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("http://{}/prompt", SERVER_ADDRESS);
    let payload = serde_json::json!({
        "prompt": prompt,
        "client_id": client_id,
    });
    let resp = client.post(&url).json(&payload).send()?.json()?;
    Ok(resp)
}

/// Retrieves an image by constructing a URL with query parameters.
fn get_image(
    client: &Client,
    filename: &str,
    subfolder: &str,
    folder_type: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let url = format!("http://{}/view", SERVER_ADDRESS);
    let params = [
        ("filename", filename),
        ("subfolder", subfolder),
        ("type", folder_type),
    ];
    let bytes = client.get(&url).query(&params).send()?.bytes()?;
    Ok(bytes.to_vec())
}

/// Retrieves the history JSON for a given prompt id.
fn get_history(client: &Client, prompt_id: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("http://{}/history/{}", SERVER_ADDRESS, prompt_id);
    let resp = client.get(&url).send()?.json()?;
    Ok(resp)
}

/// Uploads an image to the server.
fn upload_image(client: &Client, filename: String) -> Result<(), Box<dyn Error>> {
    let url = format!("http://{}/upload/image", SERVER_ADDRESS);

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));

    let file = std::fs::File::open(&filename)?;
    // Build the multipart part for the file.
    let file_part = multipart::Part::reader(file)
        .file_name(filename) // set the file name; adjust if needed
        .mime_str("image/png")?; // set the MIME type appropriately

    // Build the multipart form with additional fields.
    let form = multipart::Form::new()
        .part("image", file_part)
        .text("type", "input".to_string())
        .text("subfolder", "".to_string())
        .text("overwrite", "1".to_string());

    let resp = client.post(url).multipart(form).send()?;

    dbg!(&resp);

    Ok(())
}

/// Listens on the websocket until the prompt execution is done,
/// then downloads images from the history.
fn get_images(
    ws: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
    client: &Client,
    prompt: &Value,
    client_id: &str,
) -> Result<HashMap<String, Vec<Vec<u8>>>, Box<dyn Error>> {
    // Submit the prompt and get the prompt_id.
    let queue_resp = queue_prompt(client, prompt, client_id)?;
    dbg!(&queue_resp);
    let prompt_id = queue_resp["prompt_id"]
        .as_str()
        .ok_or("prompt_id not found")?;

    // Listen for websocket messages until we see one indicating execution is done.
    loop {
        let msg = ws.read()?;
        if msg.is_text() {
            let text = msg.into_text()?;
            dbg!(&text);
            let message: Value = serde_json::from_str(&text)?;
            if message["type"] == "executing" {
                let data = &message["data"];
                // When data["node"] is null and the prompt_id matches, execution is done.
                if data["node"].is_null() && data["prompt_id"] == prompt_id {
                    break;
                }
            }
        } else {
            // Skip binary messages (e.g. latent previews).
            continue;
        }
    }

    // Get history for the executed prompt.
    let history: Value = get_history(client, prompt_id)?;
    let history_for_prompt = &history[prompt_id];
    let outputs = history_for_prompt["outputs"]
        .as_object()
        .ok_or("outputs not found")?;

    let mut output_images: HashMap<String, Vec<Vec<u8>>> = HashMap::new();

    // Iterate through each node's output.
    for (node_id, node_output) in outputs.iter() {
        let mut images_output = Vec::new();
        if let Some(images) = node_output.get("images") {
            if let Some(arr) = images.as_array() {
                for image in arr {
                    let filename = image["filename"].as_str().ok_or("filename not found")?;
                    let subfolder = image["subfolder"].as_str().ok_or("subfolder not found")?;
                    let folder_type = image["type"].as_str().ok_or("type not found")?;
                    let image_data = get_image(client, filename, subfolder, folder_type)?;
                    images_output.push(image_data);
                }
            }
        }
        output_images.insert(node_id.clone(), images_output);
    }

    Ok(output_images)
}
