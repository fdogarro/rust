use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

let (_stream, stream_handle) = OutputStream::try_default().unwrap();

let sink = rodio::Sink::try_new(&stream_handle).unwrap();
// Load a sound from a file, using a path relative to Cargo.toml
let file = std::fs::File::open("sounds/water.mp3").unwrap();
sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
sink.sleep_until_end();