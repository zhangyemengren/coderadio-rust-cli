use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::stream::StreamExt;
use serde_json::{Value};

fn print_begin() {
    println!(r#"
  _________  ___  _______  ___  ___   ___  ________
 / ___/ __ \/ _ \/ __/ _ \/ _ \/ _ | / _ \/  _/ __ \
/ /__/ /_/ / // / _// , _/ , _/ __ |/ // // // /_/ /
\___/\____/____/___/_/|_/_/|_/_/ |_/____/___/\____/ "#);

}

#[tokio::main]
async fn main() {
    print_begin();
    let url = "wss://coderadio-admin.freecodecamp.org/api/live/nowplaying/coderadio";
    let (w, _) = connect_async(url).await.unwrap();
    let (_, mut reader) = w.split();
    while let Some(v) = reader.next().await {
        let message = v.unwrap();
        if let Message::Text(text) = message {
            let json: Value = serde_json::from_str(text.as_str()).unwrap();
            println!("{:?}", json);
        }
    }
}