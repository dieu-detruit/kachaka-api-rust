use kachaka_api::{KachakaApiClient, StartCommandOptions};

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();
    let response = client
        .speak(
            "こんにちは、私の名前はなんだと思いますか？当ててみてください",
            StartCommandOptions::default()
                .title("こんにちは")
                .tts_on_success("こんにちは")
                .cancel_all(true),
        )
        .await
        .unwrap();
    println!("{:?}", response);

    // sleep 1 second
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let response = client.cancel_command().await.unwrap();
    println!("{:?}", response);
}
