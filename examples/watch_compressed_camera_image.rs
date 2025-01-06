use futures::stream::StreamExt;
use image::{DynamicImage, GenericImageView};
use kachaka_api::KachakaApiClient;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;

fn run_minifb_window(mut rx: UnboundedReceiver<DynamicImage>) {
    let mut window = Window::new("My Window", 800, 600, WindowOptions::default())
        .expect("Failed to create window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Ok(dynamic_image) = rx.try_recv() {
            let (width, height) = dynamic_image.dimensions();
            let rgba = dynamic_image.to_rgba8();

            let buffer: Vec<u32> = rgba
                .chunks_exact(4)
                .map(|p| {
                    let (r, g, b, a) = (p[0], p[1], p[2], p[3]);
                    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
                })
                .collect();

            window
                .update_with_buffer(&buffer, width as usize, height as usize)
                .unwrap();
        } else {
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}

fn main() {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let mut client = KachakaApiClient::connect("http://kachaka-020.local:26400")
                .await
                .unwrap();

            let mut front_compressed_camera_image_stream =
                client.watch_front_camera_ros_compressed_image().await;
            while let Some(Ok(image)) = front_compressed_camera_image_stream.next().await {
                tx.send(image).unwrap();
            }
        });
    });
    run_minifb_window(rx);
}
