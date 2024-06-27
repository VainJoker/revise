use git_revise::revise::{prompts::{commit_subject::Part, Inquire}, Revise};
use human_panic::setup_panic;

fn main() {
    setup_panic!();
    Revise::new().run().unwrap();
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
