use revise::revise::Revise;

fn main() {
    let mut revise = Revise::new();
    let result = revise.run();
    match result {
        Ok(_) => {
            std::process::exit(exitcode::OK);
        }
        Err(e) => {
            eprintln!("Error occurred when trying to commit, err: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
