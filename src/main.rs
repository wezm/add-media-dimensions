use std::process::Command;
use std::{env, fs, io};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Tweet {
    url: String,
    author_name: String,
    author_url: String,
    html: String,
    width: u32,
    height: Option<u32>,
    #[serde(rename = "type")]
    type_: String,
    cache_age: String,
    provider_name: String,
    provider_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_source: Option<String>,
    version: String,
}

#[derive(Deserialize)]
struct VideoInfo {
    streams: Vec<Stream>,
}

#[derive(Deserialize)]
struct Stream {
    width: u32,
    height: u32,
}

fn main() {
    let input_file = env::args_os()
        .skip(1)
        .next()
        .map(|path| PathBuf::from(path))
        .expect("No input JSON specified");
    let working_dir = input_file.parent().expect("input file has no parent");
    env::set_current_dir(working_dir).expect("unable to cd");

    let json = fs::read_to_string(input_file).expect("unable to read file");
    let mut tweets: Vec<Tweet> = serde_json::from_str(&json).expect("unable to parse JSON");
    tweets.iter_mut().for_each(|tweet| {
        if let Some(path) = tweet.media.as_ref() {
            eprintln!("{}", path);
            let (width, height) = media_dimensions(path);
            tweet.width = width;
            tweet.height = Some(height);
        }
    });

    let stdout = io::stdout();
    let writer = stdout.lock();
    serde_json::to_writer_pretty(writer, &tweets).expect("error serialising to JSON");
}

fn media_dimensions(path: &str) -> (u32, u32) {
    let output = Command::new("ffprobe")
        .args(&[
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            path,
        ])
        .output()
        .unwrap_or_else(|_| panic!("error running ffprobe on {}", path));
    let video_info: VideoInfo = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        let json =
            String::from_utf8(output.stdout).unwrap_or_else(|_| String::from("json is not UTF-8"));
        panic!("unable to parse ffprobe json for {}, {}", path, json)
    });
    let stream = video_info
        .streams
        .first()
        .unwrap_or_else(|| panic!("no streams for {}", path));
    (stream.width, stream.height)
}
