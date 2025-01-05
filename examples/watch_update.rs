use futures::stream::StreamExt;
use kachaka_api::KachakaApiClient;

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();

    let mut robot_serial_number_stream = client.watch_robot_serial_number().await;
    let mut robot_version_stream = client.watch_robot_version().await;
    let mut robot_pose_stream = client.watch_robot_pose().await;
    let mut battery_info_stream = client.watch_battery_info().await;
    let mut error_stream = client.watch_error().await;
    let mut last_command_result_stream = client.watch_last_command_result().await;

    loop {
        tokio::select! {
            Some(serial_number) = robot_serial_number_stream.next() => {
                println!("robot serial number: {:?}", serial_number);
            }
            Some(version) = robot_version_stream.next() => {
                println!("robot version: {:?}", version);
            }
            Some(pose) = robot_pose_stream.next() => {
                println!("robot pose: {:?}", pose);
            }
            Some(battery_info) = battery_info_stream.next() => {
                println!("battery info: {:?}", battery_info);
            }
            Some(errors) = error_stream.next() => {
                println!("errors: {:?}", errors);
            }
            Some(result) = last_command_result_stream.next() => {
                println!("last command result: {:?}", result);
            }
            else => break
        }
    }
}
