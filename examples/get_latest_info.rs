use kachaka_api::KachakaApiClient;

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();

    let response = client.get_robot_serial_number(0).await.unwrap();
    println!("robot serial number: {:?}", response);

    let response = client.get_robot_version(0).await.unwrap();
    println!("robot version: {:?}", response);

    let response = client.get_robot_pose(0).await.unwrap();
    println!("pose: {:?}", response);

    let response = client.get_battery_info(0).await.unwrap();
    println!("battery info: {:?}", response);

    let response = client.get_command_state(0).await.unwrap();
    println!("command state: {:?}", response);

    let response = client.get_last_command_result(0).await.unwrap();
    println!("last command result: {:?}", response);
}
