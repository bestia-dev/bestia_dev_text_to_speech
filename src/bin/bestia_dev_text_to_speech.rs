//! src/bin/bestia_dev_text_to_speech.rs

// The `bin` has all the stdin and stdout.
// The `lib` must be in/out agnostic. That is the responsibility of the `bin`

/// entry point into the bin executable
fn main() {
    // logging is essential for every project
    pretty_env_logger::init();

    // The google api-key must be in the env variable bestia_dev_text_to_speech_api_key
    match std::env::var("bestia_dev_text_to_speech_api_key") {
        Err(_err) => println!("Error: Env variable $bestia_dev_text_to_speech_api_key not found!"),
        Ok(api_key) => {
            // the first CLI argument is the filename of the text to speech
            match std::env::args().nth(1).as_deref() {
                None | Some("--help") | Some("-h") => print_help(),
                Some(file_name) => text_to_speech(file_name, &api_key),
            }
        }
    }
}

/// print help
fn print_help() {
    println!(
        r#"        
bestia_dev_text_to_speech --help
The first and only argument is the file_name of the text to speech:
bestia_dev_text_to_speech text.txt
"#
    );
}

/// print my name
fn text_to_speech(file_name: &str, api_key: &str) {
    // call the function from the `lib`
    bestia_dev_text_to_speech::post_text_to_speech(file_name, api_key);
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
