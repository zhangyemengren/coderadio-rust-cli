use std::io::{BufReader, Cursor};
use std::thread::{self, JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::stream::StreamExt;
use serde_json::{Value};
use rodio::Decoder;

fn print_begin() {
    println!(r#"
  _________  ___  _______  ___  ___   ___  ________
 / ___/ __ \/ _ \/ __/ _ \/ _ \/ _ | / _ \/  _/ __ \
/ /__/ /_/ / // / _// , _/ , _/ __ |/ // // // /_/ /
\___/\____/____/___/_/|_/_/|_/_/ |_/____/___/\____/ "#);

}

async fn wss() {
    let url = "wss://coderadio-admin.freecodecamp.org/api/live/nowplaying/coderadio";
    let (w, _) = connect_async(url).await.unwrap();
    let (_, mut reader) = w.split();
    while let Some(v) = reader.next().await {
        let message = v.unwrap();
        if let Message::Text(text) = message {
            let json: Value = serde_json::from_str(text.as_str()).unwrap();
            println!("{:?}", json["station"]["listen_url"]);
        }
    }
}
fn get_audio() -> JoinHandle<()>{
    thread::spawn(move || {
        let url = "https://coderadio-admin.freecodecamp.org/radio/8010/radio.mp3";
        let url = "http://websrvr90va.audiovideoweb.com/va90web25003/companions/Foundations%20of%20Rock/13.01.mp3";
        let mut res = reqwest::blocking::get(url).unwrap();
        // println!("{:?}", res.bytes().unwrap());
        let mut cursor = Cursor::new(res.bytes().unwrap());
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let mut data = Decoder::new(cursor).unwrap();
        sink.append(data);
        sink.sleep_until_end();
    })
}
#[tokio::main]
async fn main() {
    print_begin();
    get_audio().join().unwrap();
}