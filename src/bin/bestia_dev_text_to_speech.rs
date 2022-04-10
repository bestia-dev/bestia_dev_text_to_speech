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
bestia_dev_text_to_speech sample_files/text_1.txt
"#
    );
}

/// reads the text in file_name, converts to speech and saves a audio file with the same name, but extension mp3
fn text_to_speech(file_name: &str, api_key: &str) {
    // path should always use the Path type and not the overly generic String
    let file_name = std::path::Path::new(file_name);
    // early exit on every error
    if !file_name.exists() {
        log::error!("File name {} does not exist!", file_name.to_string_lossy());
        return;
    }
    // then `bin` must solve all input/output. The `lib` is IO agnostic.
    let text = std::fs::read_to_string(file_name).unwrap();
    // call the function from the `lib`
    let mp3_bytes = bestia_dev_text_to_speech::post_text_to_speech(&text, api_key);
    let mut new_file_name = std::path::PathBuf::from(file_name);
    new_file_name.set_extension("mp3");
    // save file
    std::fs::write(&new_file_name, mp3_bytes).unwrap();
    println!("");
    println!("The speech audio file: {}. Play it.", &new_file_name.to_string_lossy());
    println!("");
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
