use futures::stream::StreamExt;
use kachaka_api::KachakaApiClient;

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();

    let mut last_command_result_stream = client.watch_last_command_result().await;

    while let Some(result) = last_command_result_stream.next().await {
        println!("last command result: {:?}", result);
    }
}
