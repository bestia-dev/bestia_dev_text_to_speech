// examples/example_1.rs

//! A simple example how to use the `lib`

use bestia_dev_text_to_speech::*;

/// example how to use format_hello_phrase() and format_upper()
fn main() {
    let my_name = "john doe";
    let phrase = format_hello_phrase(my_name);
    println!("{}", phrase);
    // shadowing the variable name
    let phrase = format_upper(my_name);
    println!("{}", phrase);
}
