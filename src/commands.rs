#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[derive(Debug)]
pub(crate) struct Pixel {
    pub x: u8,
    pub y: u8,
    pub color: Color,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Command {
    SetPixel(Pixel),
    ImageMode(u8),
    UploadPng(Vec<u8>),
    FullScreenColor(Color),
    ScreenBrightness(u8),
    ScreenOn,
    ScreenOff,
    CountdownStart(std::time::Duration),
    CountdownCancel,
    CountdownPause,
    CountdownResume,
}

impl Command {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Command::SetPixel(pixel) => {
                let mut command: Vec<u8> = vec![10, 0, 5, 1, 0];
                command.push(pixel.color.r);
                command.push(pixel.color.g);
                command.push(pixel.color.b);
                command.push(pixel.x);
                command.push(pixel.y);
                command
            }
            Command::ImageMode(mode) => {
                vec![5, 0, 4, 1, *mode]
            }
            Command::UploadPng(data) => create_png_payload(data),
            Command::FullScreenColor(color) => {
                vec![7, 0, 2, 2, color.r, color.g, color.b]
            }
            Command::ScreenBrightness(brightness) => {
                vec![5, 0, 4, 128, *brightness]
            }
            Command::ScreenOn => {
                vec![5, 0, 7, 1, 0]
            }
            Command::ScreenOff => {
                vec![5, 0, 7, 1, 1]
            }
            Command::CountdownStart(duration) => {
                let seconds = duration.as_secs();
                let minutes = seconds / 60;
                let seconds = seconds % 60;
                let mut command = vec![7, 0, 8, 128, 1];

                command.push(minutes as u8);
                command.push(seconds as u8);
                command
            }
            Command::CountdownCancel => {
                vec![7, 0, 8, 128, 0, 0, 0]
            }
            Command::CountdownPause => {
                vec![7, 0, 8, 128, 2, 0, 0]
            }
            Command::CountdownResume => {
                vec![7, 0, 8, 128, 3, 0, 0]
            }
        }
    }
}

fn create_png_payload(png_data: &Vec<u8>) -> Vec<u8> {
    let mut payload = Vec::new();

    let png_chunks = png_data.chunks(4096);
    let idk = png_data.len() + png_chunks.len();

    for (i, chunk) in png_chunks.enumerate() {
        payload.extend_from_slice(&(idk as u16).to_le_bytes());
        payload.extend_from_slice(&[0, 0, if i > 0 { 2 } else { 0 }]);
        payload.extend_from_slice(&(png_data.len() as u32).to_le_bytes());
        payload.extend_from_slice(chunk);
    }
    payload
}
