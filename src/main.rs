use inquire::{InquireError, Select};
use revise::config::Config;

fn main() {
    let config = Config::load_config().expect("load config error");
    let message_options = config.messages.clone();

    for msg in message_options {
        let types_options = config.get_types().clone();
        let ans: Result<String, InquireError> = Select::new(&msg.value, types_options).prompt();
        match ans {
            Ok(choice) => println!("{}! That's mine too!", choice),
            Err(_) => println!("There was an error, please try again"),
        }
    }
}
