use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;

include!(concat!(env!("OUT_DIR"), "/builtins.rs"));

pub fn play_builtin(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !BUILTINS.contains_key(name) {
        return Err(format!(r#"Builtin named `{}` not found."#, name).into());
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let cur = Cursor::new(BUILTINS[name].to_vec());
    let source = Decoder::new(cur).unwrap().buffered();

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
