use futures::stream::StreamExt;
use kachaka_api::KachakaApiClient;

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();

    let error_code_json = client.get_robot_error_code_json().await.unwrap();

    let mut error_stream = client.watch_error().await;

    loop {
        let errors = error_stream.next().await.unwrap().unwrap();
        for error in errors {
            println!("error occurred with code: {:?}", error.error_code);
            let error_info = error_code_json.get(&error.error_code).unwrap();
            println!("error info:");
            for (k, v) in error_info {
                println!("- {}: {}", k, v);
            }
        }
    }
}
