use bevy::prelude::*;
use billboard::prelude::*;
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};

use crate::RefImage;


pub fn paste(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    info!("Paste event triggered");
    use std::io::Read;

    let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
    match result {
        Ok((mut pipe, _)) => {
            let mut contents = vec![];
            if let Ok(_) = pipe.read_to_end(&mut contents) {                
                let clipboard = String::from_utf8_lossy(&contents).to_string();
                debug!("Clipboard contents: {:?}", &clipboard);

                if clipboard.ends_with(".png") {
                    let path = std::path::Path::new(&clipboard);
                    let file_name = path.file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("clipboard_image.png")
                        .to_string();

                    let new_path = std::path::Path::new("assets/ref/").join(&file_name);

                    let new_asset_path = format!("ref/{}", &file_name);
                    // copy to ref assets
                    std::fs::copy(&path, &new_path)
                        .expect("Failed to copy file");


                    let image_handle: Handle<Image> = asset_server.load(&new_asset_path);
                    commands.spawn((
                        BillboardTexture(image_handle.clone()),
                        BillboardMesh(meshes.add(Rectangle::from_size(Vec2::splat(2.0)))),
                        Transform::default(),
                        Name::new(file_name),
                        RefImage {
                            path: new_asset_path.clone(),
                        },
                    ));
                        
                }
                else {
                    warn!("Clipboard contents not an image: {:?}", &clipboard);
                }
                

            }
        }
        Err(err) => {
            error!("Error pasting: {:?}", err);
        }
    }
}