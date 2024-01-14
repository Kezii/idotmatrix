use colors_transform::Color;
use std::time::Duration;

use tokio::time;

mod bluetooth;
mod commands;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let bluetooth = bluetooth::BluetoothWrapper::new().await;

    bluetooth
        .send_command(&commands::Command::ScreenBrightness(255))
        .await;

    bluetooth
        .send_command(&commands::Command::ImageMode(1))
        .await;

    let png_data = std::fs::read("./demo_32.png").unwrap();

    bluetooth
        .send_command(&commands::Command::UploadPng(png_data))
        .await;

    std::thread::sleep(Duration::from_secs(1));

    let mut hue = 180.0;

    let mut last_loop = time::Instant::now();
    loop {
        for y in 0..32 {
            for x in 0..32 {
                let color = colors_transform::Hsl::from(hue, 100.0, 50.0).to_rgb();
                hue += 0.2;
                hue %= 360.0;
                let color_u8 = color.as_tuple();
                let color_u8 = (color_u8.0 as u8, color_u8.1 as u8, color_u8.2 as u8);

                let command = commands::Command::SetPixel(commands::Pixel {
                    x: x as u8,
                    y: y as u8,
                    color: commands::Color {
                        r: color_u8.0,
                        g: color_u8.1,
                        b: color_u8.2,
                    },
                });
                bluetooth.send_command(&command).await;
            }
        }

        let now = time::Instant::now();
        let duration = now.duration_since(last_loop);
        println!("Loop duration: {:?}", duration);
        last_loop = now;
    }
}
