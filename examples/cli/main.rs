use clap::Parser;
use colors_transform::Color;
use idotmatrix::{IDMColor, IDMCommand, IDMPixel};
use std::{path::PathBuf, time::Duration};

use tokio::time;

mod bluetooth;

#[macro_use]
extern crate lazy_static;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    screen_on: bool,
    #[arg(long)]
    screen_off: bool,
    /// Pixel in format x,y,#ffffff
    #[arg(long, value_parser = parse_pixel_string)]
    set_pixel: Option<IDMPixel>,
    #[arg(long)]
    image_mode: Option<u8>,
    /// Path to png file
    #[arg(long)]
    upload_png: Option<PathBuf>,
    /// Path to gif file
    #[arg(long)]
    upload_gif: Option<PathBuf>,
    /// Color in hex format, e.g. #ffffff
    #[arg(long, value_parser = parse_color_string)]
    full_screen_color: Option<IDMColor>,
    /// Brightness in percent, e.g. 100
    #[arg(long, value_parser = clap::value_parser!(u8).range(0..=100))]
    screen_brightness: Option<u8>,
    /// Countdown in seconds
    #[arg(long)]
    countdown_start: Option<u64>,
    #[arg(long)]
    countdown_cancel: bool,
    #[arg(long)]
    countdown_pause: bool,
    #[arg(long)]
    countdown_resume: bool,
    /// Continuously change color demo
    #[arg(long)]
    color_hue: bool,
}

#[tokio::main]
async fn main() {
    let cli = Args::parse();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("idotmatrix=info"))
        .init();

    let bluetooth = bluetooth::BluetoothWrapper::new().await;

    if cli.screen_on {
        bluetooth.send_command(&IDMCommand::ScreenOn).await;
    }

    if cli.screen_off {
        bluetooth.send_command(&IDMCommand::ScreenOff).await;
    }

    if let Some(pixel) = cli.set_pixel {
        bluetooth.send_command(&IDMCommand::SetPixel(pixel)).await;
    }

    if let Some(mode) = cli.image_mode {
        bluetooth.send_command(&IDMCommand::ImageMode(mode)).await;
    }

    if let Some(png) = cli.upload_png {
        let png_data = std::fs::read(png).unwrap();
        bluetooth
            .send_command(&IDMCommand::UploadPng(png_data))
            .await;
    }

    if let Some(gif) = cli.upload_gif {
        let gif_data = std::fs::read(gif).unwrap();
        bluetooth
            .send_command(&IDMCommand::UploadGif(gif_data))
            .await;
    }

    if let Some(color) = cli.full_screen_color {
        bluetooth
            .send_command(&IDMCommand::FullScreenColor(color))
            .await;
    }

    if let Some(brightness) = cli.screen_brightness {
        bluetooth
            .send_command(&IDMCommand::ScreenBrightness(brightness))
            .await;
    }

    if let Some(duration) = cli.countdown_start {
        bluetooth
            .send_command(&IDMCommand::CountdownStart(Duration::from_secs(duration)))
            .await;
    }

    if cli.countdown_cancel {
        bluetooth.send_command(&IDMCommand::CountdownCancel).await;
    }

    if cli.countdown_pause {
        bluetooth.send_command(&IDMCommand::CountdownPause).await;
    }

    if cli.countdown_resume {
        bluetooth.send_command(&IDMCommand::CountdownResume).await;
    }

    if cli.color_hue {
        color_hue(&bluetooth).await;
    }
}

fn parse_color_string(color: &str) -> Result<IDMColor, String> {
    let color = color.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16).unwrap();
    let g = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b = u8::from_str_radix(&color[4..6], 16).unwrap();
    Ok(IDMColor { r, g, b })
}

fn parse_pixel_string(pixel: &str) -> Result<IDMPixel, String> {
    let mut pixel = pixel.split(',');
    let x = pixel.next().unwrap().parse::<u8>().unwrap();
    let y = pixel.next().unwrap().parse::<u8>().unwrap();
    let color = parse_color_string(pixel.next().unwrap()).unwrap();
    Ok(IDMPixel { x, y, color })
}

async fn color_hue(bluetooth: &bluetooth::BluetoothWrapper) {
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

                let command = IDMCommand::SetPixel(IDMPixel {
                    x: x as u8,
                    y: y as u8,
                    color: IDMColor {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pixel_string() {
        let pixel = parse_pixel_string("2,2,#ffffff");
        assert_eq!(pixel.x, 2);
        assert_eq!(pixel.y, 2);
        assert_eq!(pixel.color.r, 255);
        assert_eq!(pixel.color.g, 255);
        assert_eq!(pixel.color.b, 255);
    }

    #[test]
    fn test_parse_color_string() {
        let color = parse_color_string("#ffffff");
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }
}
