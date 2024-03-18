use git_revise::revise::Revise;
use human_panic::setup_panic;

fn main() {
    setup_panic!();
    let mut revise = Revise::new();
    let result = revise.run();
    match result {
        Ok(()) => {
            std::process::exit(exitcode::OK);
        }
        Err(e) => {
            eprintln!("Error occurred when trying to commit, err: {e}");
            std::process::exit(exitcode::DATAERR);
        }
    }
}
