use kachaka_api::{KachakaApiClient, StartCommandOptions};

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();
    let response = client
        .speak(
            "こんにちは",
            StartCommandOptions::default()
                .title("こんにちは")
                .tts_on_success("こんにちは")
                .cancel_all(true),
        )
        .await
        .unwrap();
    println!("{:?}", response);
}
