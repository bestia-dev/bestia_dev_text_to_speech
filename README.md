# bestia_dev_text_to_speech

[//]: # (auto_cargo_toml_to_md start)

**Text-to-speech CLI using Google api**  
***version: 0.1.44 date: 2023-05-31 author: [bestia.dev]() repository: [Github](https://github.com/bestia-dev/bestia_dev_text_to_speech)***  

 ![status](https://img.shields.io/badge/maintained-green)
 ![status](https://img.shields.io/badge/ready_for_use-green)

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-98-green.svg)](https://github.com/bestia-dev/bestia_dev_text_to_speech/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-161-blue.svg)](https://github.com/bestia-dev/bestia_dev_text_to_speech/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-30-purple.svg)](https://github.com/bestia-dev/bestia_dev_text_to_speech/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-15-yellow.svg)](https://github.com/bestia-dev/bestia_dev_text_to_speech/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-20-orange.svg)](https://github.com/bestia-dev/bestia_dev_text_to_speech/)

[//]: # (auto_lines_of_code end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/bestia_dev_text_to_speech/blob/main/LICENSE)
 [![Rust](https://github.com/bestia-dev/bestia_dev_text_to_speech/workflows/RustAction/badge.svg)](https://github.com/bestia-dev/bestia_dev_text_to_speech/)
  ![bestia_dev_text_to_speech](https://bestia.dev/webpage_hit_counter/get_svg_image/791106357.svg)

Hashtags: #rustlang #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Motivation

After creating some interesting projects and tutorials for rust development and sharing them on Github, I wanted to create youtube videos for them. Sometimes it is easier to follow a video tutorial then to read kilometers of text tutorials.  
The easy part is capturing the screen with `OBS Studio`. Everything visual is going on on the screen. It is a computer project, after all. The hard part is the spoken text.  
It is impossible to just naturally talk about what I am doing while recording the screen. Maybe somebody has the talent of fast and focused speech. I don't. Maybe because I am not a native English speaker. I am not at home as a speaker.  
And my voice sound terrible and the accent and intonation is just not what english should sound like.
But I can write pretty well. It takes ages and it is very slow, but I am happy with the result.  
I am a programmer and for me every problem has a software solution. This is the story when you have a hammer, everything looks like a nail.

Finally I decided to use a text-to-speech approach.

I have a (very cheap) google account that I use for a VM that hosts my domain and my website. Google is very kind to offer text-to-speech api for free for 1 million characters. I hope this could be the right solution for me. It sounds very natural. It uses some magic neural network and it sounds definitely better then me.  
But it is only an api. There is no prepared application or website that I can use.

What is more fun, than write an application in Rust for my problem?  

## Rust CLI from template

The easiest application to write in Rust is a CLI in Linux. And honestly I don't need more. I will have a text file as input and an mp3 file as output. Perfect for a CLI application.
For the first time I will develope a new Rust application entirely inside a docker container using my project <https://github.com/bestia-dev/docker_rust_development>. I want to have my rust development sandboxed. I am scared to allow tools and crates from unknown developers to have full access to my underlying system: Win10+WSL2 (<https://github.com/bestia-dev/win10_wsl2_debian11>).  
In `WSL2 terminal`:

```bash
cd rustprojects/docker_rust_development/
# my development pod/container is already created. I just need to restart it after reboot
sh rust_dev_pod_after_reboot.sh
```

In VSCode: `F1`, type `ssh`, choose `Remote-SSH:Connect to Host...`, choose `rust_dev_pod`, type the passphrase for the SSH key. We have opened a new VSCode window that works inside the Rust development container.

I will also use for the first time my template for Rust CLI development: <https://github.com/bestia-dev/bestia_dev_cargo_auto_new_cli>. It separates a lib and a bin, so the project could be used as a library for other projects. It has prepared automation tasks for building the project and some samples how to use tests and examples.  
In `VSCode terminal`:

```bash
# save the passphrase to communicate with Github over SSH
eval $(ssh-agent)
ssh-add /home/rustdevuser/.ssh/githubssh1
# clone the template
mkdir ~/rustprojects
cd ~/rustprojects
git clone git@github.com:bestia-dev/bestia_dev_cargo_auto_new_cli.git
# rename the template to the project name
mv bestia_dev_cargo_auto_new_cli bestia_dev_text_to_speech
cd bestia_dev_text_to_speech
# remove the git files, because now it is a new project
rm -r -f .git
rm -r docs
mv src/bin/bestia_dev_cargo_auto_new_cli.rs src/bin/bestia_dev_text_to_speech.rs
# open a new VSCode window inside the new project
code .

```

## Template Renaming

1. In Cargo.toml change the name and the description and some other info.
2. In `VSCode terminal` type `cargo auto release` to copy the title and description to README.md and lib.rs.
3. Replace the title in other files. In VSCode Ctrl+Shift+H to open the replace in all files. Search for `bestia_dev_cargo_auto_new_cli`, replace with `bestia_dev_text_to_speech`. When you click on the result list item, you can see what you are actually replacing. A small icon at the right end of the item is `Replace (Ctrl+Shift+1)`.

For now I will leave the functions that are used in examples and tests. I will modify them, when the new functions will be ready.  

## Github push new project

First in VSCode open Source Control (Ctrl+Shift+G), click `Initialize Repository`, type the message `init` and Ctrl+Enter.
On the Github website create a new project <https://github.com/new>, copy the name and the description from Cargo.toml. You don't need other files magically created. Save the new project. Github is very kind to give us some Git commands we can use. Now in `VSCode terminal` copy the commands from Github (second group):

```bash
git remote add origin git@github.com:bestia-dev/bestia_dev_text_to_speech.git
git branch -M main
git push -u origin main
```

## README.md

Open `README.md` and delete the text you don't need. Don't delete the markers for the automation. You can see the markdown preview with (Ctrl+k, v). Save the changes.
Commit the changes and push in `VSCode terminal`: `cargo auto commit_and_push "readme cleaned"`
Commit and push often to reduce the risk of loosing your work.

## google api text-to-speech

In my googlecloud account I enabled the text-to-speech api: <https://console.cloud.google.com/apis/api/texttospeech.googleapis.com>  
Service name `texttospeech.googleapis.com`  
I need some credentials to access the api: <https://console.cloud.google.com/apis/credentials>
Click `+Create credentials`, `API key`, then `edit`, rename to `API text-to-speech`, restrict to only `Cloud Text-to-Speech API`.
Use this key in your application by passing it with the `?key=API_KEY` parameter for every request.
Save this api key in env variable:

```bash
# put a space before the command to disable bash history
 export bestia_dev_text_to_speech_api_key=YOUR_API_KEY
env
echo $bestia_dev_text_to_speech_api_key
# finally to delete the env variable after use
unset bestia_dev_text_to_speech_api_key
```

The simplest example from google:

```bash
curl -X POST \
-H "Content-Type: application/json; charset=utf-8" \
-d @sample_files/request.json \
"https://texttospeech.googleapis.com/v1/text:synthesize?key=$bestia_dev_text_to_speech_api_key"
```

Because of https only the domain part of the url `texttospeech.googleapis.com` is visible on the wire. The rest of the url `/v1/text:synthesize?key=$bestia_dev_text_to_speech_api_key` is encrypted. So it looks that the api-key is secure. They are encrypted on the wire (in transport) but if either end (user or server) logs the URL to a plain text file and does not sanitize credentials... now that's a different conversation. Browsers can save the entire url in history, but I am not using a browser.

## https client

Among different crates for https client: curl-rust, hyper, reqwest, Isahc, Surf and ureq,  
I choose ureq. It is minimal.

## Build with cargo auto

All needed automation tasks to build and manage the project are centralized in the directory `automation_tasks_rs`. This is a simple Rust project and it is easy to modify the tasks in your preferred language. The classic tasks are build, release, doc and commit_and_push. But you can add any task you need to repeat and is important for the managing of the project.  
Use `cargo auto` to list the possible tasks. The command has bash-completion for easy writing command with just few keys.  


## The result is mp3 file

In VSCode right click on the file `speech.mp3` and choose `Download` to transfer the file over SSH to the local system.

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev) to verify the trustworthiness of each of your dependencies.

Please, spread this info.

You can also read crev reviews quickly on the web:

<https://web.crev.dev/rust-reviews/crates/>

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
