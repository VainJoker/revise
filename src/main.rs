use git_revise::{config, revise::Revise};
use human_panic::setup_panic;

fn main() {
    setup_panic!();
    config::initialize_config().unwrap_or_else(|e| {
        eprintln!("Load config err: {e}");
        std::process::exit(exitcode::CONFIG);
    });
    Revise::default().run().unwrap();
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
