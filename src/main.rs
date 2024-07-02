use git_revise::{cli, config, revise::Revise};
use human_panic::setup_panic;

#[tokio::main]
async fn main() {
    setup_panic!();
    // cli::cmd();
    config::initialize_config().unwrap_or_else(|e| {
        eprintln!("Load config err: {e}");
        std::process::exit(exitcode::CONFIG);
    });
    Revise::default().run().await.unwrap();
    // match Revise::default().run() {
    //     Ok(()) => {
    //         std::process::exit(exitcode::OK);
    //     }
    //     Err(e) => {
    //         eprintln!("Error occurred when trying to commit, err: {e}");
    //         std::process::exit(exitcode::DATAERR);
    //     }
    // }
}
