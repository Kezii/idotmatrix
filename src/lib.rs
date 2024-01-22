#[derive(Debug, Clone)]
pub struct IDMColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[derive(Debug, Clone)]
pub struct IDMPixel {
    pub x: u8,
    pub y: u8,
    pub color: IDMColor,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum IDMCommand {
    SetPixel(IDMPixel),
    ImageMode(u8),
    UploadPng(Vec<u8>),
    UploadGif(Vec<u8>),
    FullScreenColor(IDMColor),
    ScreenBrightness(u8),
    ScreenOn,
    ScreenOff,
    CountdownStart(std::time::Duration),
    CountdownCancel,
    CountdownPause,
    CountdownResume,
}

impl IDMCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            IDMCommand::SetPixel(pixel) => vec![
                10,
                0,
                5,
                1,
                0,
                pixel.color.r,
                pixel.color.g,
                pixel.color.b,
                pixel.x,
                pixel.y,
            ],

            IDMCommand::ImageMode(mode) => vec![5, 0, 4, 1, *mode],

            IDMCommand::UploadPng(data) => create_png_payload(data),

            IDMCommand::UploadGif(data) => create_gif_payload(data),

            IDMCommand::FullScreenColor(color) => vec![7, 0, 2, 2, color.r, color.g, color.b],

            IDMCommand::ScreenBrightness(brightness) => vec![5, 0, 4, 128, *brightness],

            IDMCommand::ScreenOn => vec![5, 0, 7, 1, 1],

            IDMCommand::ScreenOff => vec![5, 0, 7, 1, 0],

            IDMCommand::CountdownStart(duration) => {
                let seconds = duration.as_secs();
                let minutes = seconds / 60;
                let seconds = seconds % 60;

                vec![7, 0, 8, 128, 1, minutes as u8, seconds as u8]
            }
            IDMCommand::CountdownCancel => vec![7, 0, 8, 128, 0, 0, 0],

            IDMCommand::CountdownPause => vec![7, 0, 8, 128, 2, 0, 0],

            IDMCommand::CountdownResume => vec![7, 0, 8, 128, 3, 0, 0],
        }
    }
}

fn create_png_payload(png_data: &[u8]) -> Vec<u8> {
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

fn create_gif_payload(gif_data: &[u8]) -> Vec<u8> {
    let mut payload = Vec::new();
    let header_length = 16;

    let crc = crc32fast::hash(gif_data);
    let gif_chunks = gif_data.chunks(4096);

    for (i, chunk) in gif_chunks.enumerate() {
        payload.extend_from_slice(&((chunk.len() + header_length) as u16).to_le_bytes());
        payload.extend_from_slice(&[1, 0, if i > 0 { 2 } else { 0 }]);
        payload.extend_from_slice(&((gif_data.len() + header_length) as u32).to_le_bytes());
        payload.extend_from_slice(&crc.to_le_bytes());
        payload.extend_from_slice(&[5, 0, 13]);

        payload.extend_from_slice(chunk);
    }

    payload
}
