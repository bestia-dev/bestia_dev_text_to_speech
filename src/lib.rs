// bestia_dev_text_to_speech/src/lib.rs

#![doc=include_str!("../README.md")]

// The `bin` has all the stdin and stdout.
// The `lib` must be in/out agnostic. That is the responsibility of the `bin`

mod utils_mod;

/// post to google cloud the request for text-to-speech and return a byte[]
pub fn post_text_to_speech(text: &str, api_key: &str) -> Vec<u8> {
    let json = prepare_request_json(text);
    // because of https only the part `texttospeech.googleapis.com` is visible on the wire.
    // The rest of the URL `/v1/text:synthesize?key={}` is encrypted.
    // So the secret api-key is encrypted over the wire.
    let url = format!("https://texttospeech.googleapis.com/v1/text:synthesize?key={}", api_key);
    let response_body: String = ureq::post(&url)
        .set("Content-Type", "application/json; charset=utf-8")
        .send_string(&json)
        .unwrap()
        .into_string()
        .unwrap();
    // extract the audio content from json format
    // I should use a json library, but the format is fixed and is easy to extract the data.
    let encoded_base64 = response_body
        .trim_start_matches("{\n  \"audioContent\": \"")
        .trim_end_matches("\"\n}\n");
    // base64 decode
    // return Vec<u8>
    base64::decode(encoded_base64).unwrap()
}

/// format the request json from the text file
pub fn prepare_request_json(text: &str) -> String {
    // I should be using a json library here, but this is really a simple replace going on here.
    //escape for json string: only 2 characters " and \
    let text = text.replace(r#"\"#, r#"\\"#).replace(r#"""#, r#"\""#);
    let json = format!(
        r#"{{
"input":{{
    "text":"{}"
}},
"voice":{{
    "languageCode":"en-AU",
    "name":"en-AU-Chirp3-HD-Zubenelgenubi",
    "ssmlGender":"MALE"
}},
"audioConfig":{{
    "audioEncoding":"MP3"
}}
}}"#,
        text
    );
    // return
    json
}

/// format the hello phrase
pub fn format_hello_phrase(my_name: &str) -> String {
    log::info!("start format_hello_phrase()");
    // return
    format!("Hello {}!", my_name)
}

/// format the hello phrase with uppercase name
pub fn format_upper_hello_phrase(my_name: &str) -> String {
    log::info!("start format_upper_hello_phrase()");
    // shadowing the same variable name:
    let my_name = utils_mod::make_uppercase(my_name);
    // return
    format!("Hello {}!", &my_name)
}
