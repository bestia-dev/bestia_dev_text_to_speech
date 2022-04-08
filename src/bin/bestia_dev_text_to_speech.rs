//! bestia_dev_text_to_speech/src/bin/bestia_dev_text_to_speech.rs

// The `bin` has all the stdin and stdout.
// The `lib` must be in/out agnostic. That is the responsibility of the `bin`

/// entry point into the bin executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // super simple argument parsing. There are crates that can parse complex arguments.
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("print") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(my_name) => {
                print_my_name(my_name);
            }
            None => println!("Missing arguments `my_name`."),
        },
        Some("upper") => match std::env::args().nth(2).as_deref() {
            // second argument
            Some(my_name) => {
                upper_my_name(my_name);
            }
            None => println!("Missing arguments `my_name`."),
        },
        _ => println!("Unrecognized arguments. Try `bestia_dev_text_to_speech --help`"),
    }
}

/// print help
fn print_help() {
    println!(
        r#"
bestia_dev_text_to_speech --help
bestia_dev_text_to_speech print my_name
bestia_dev_text_to_speech upper my_name
"#
    );
}

/// print my name
fn print_my_name(my_name: &str) {
    // call the function from the `lib`
    println!("{}", bestia_dev_text_to_speech::format_hello_phrase(my_name));
}

/// print my name
fn upper_my_name(my_name: &str) {
    // call the function from the `lib`
    println!("{}", bestia_dev_text_to_speech::format_upper(my_name));
}
