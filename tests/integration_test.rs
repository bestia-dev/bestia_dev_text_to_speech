// tests/integration_test.rs

use bestia_dev_text_to_speech::*;

#[test]
fn integration_test_01() {
    let my_name = "abcd";
    assert_eq!(format_hello_phrase(my_name), "Hello abcd!");
    assert_eq!(format_upper_hello_phrase(my_name), "Hello ABCD!");
}
