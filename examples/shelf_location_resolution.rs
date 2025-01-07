use futures::stream::StreamExt;
use kachaka_api::shelf_location_resolver::ShelfLocationResolver;
use kachaka_api::KachakaApiClient;

#[tokio::main]
async fn main() {
    let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
        .await
        .unwrap();

    let resolver = ShelfLocationResolver::new(client.clone());
    let mut moving_shelf_id_stream = client.watch_moving_shelf_id().await;

    tokio::join! {
        async {
            resolver.run_update_loop().await
        },
        async {
            while let Some(Ok(moving_shelf_id)) = moving_shelf_id_stream.next().await {
                println!("Moving shelf ID: {}", moving_shelf_id);
                if let Some(shelf) = resolver.get_shelf_by_id(&moving_shelf_id).await {
                    println!("a.k.a. shelf: {:?}", shelf);
                }
            }
            ()
        }
    };
}
