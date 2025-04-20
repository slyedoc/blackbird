
use orbit::{states::AppState, init_bevy_app};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[clap(short, long, value_enum, default_value_t = AppState::Splash)]
    state: AppState,
}

fn main() {
    let args = Args::parse();
    init_bevy_app(args.state).run();
}
