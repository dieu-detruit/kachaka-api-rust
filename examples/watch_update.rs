use futures::stream::StreamExt;
use kachaka_api::KachakaApiClient;

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();

    let mut last_command_result_stream = client.watch_last_command_result().await;
    let mut robot_pose_stream = client.watch_robot_pose().await;

    loop {
        tokio::select! {
            Some(result) = last_command_result_stream.next() => {
                println!("last command result: {:?}", result);
            }
            Some(pose) = robot_pose_stream.next() => {
                println!("robot pose: {:?}", pose);
            }
            else => break
        }
    }
}
