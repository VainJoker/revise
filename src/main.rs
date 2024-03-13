use inquire::InquireError;
use revise::commit::Commit;
use revise::config::Config;

fn main() {
    let config = Config::load_config().expect("load config error");
    let mut commit = Commit::default();
    let result = commit.commit(&config);
    match result {
        Err(err) => {
            if let Some(specific_err) = err.downcast_ref::<InquireError>() {
                match specific_err {
                    InquireError::OperationCanceled | InquireError::OperationInterrupted => {
                        // Handle OperationCanceled or OperationInterrupted error.
                    }
                    _ => {
                        
                    }
                }
            }
        }
        Ok(_) => { /* Handle Ok result */ }
    }
}
