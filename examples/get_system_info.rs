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
}
