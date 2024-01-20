use std::thread;
use std::time::Duration;

use btleplug::api::{Central, Characteristic, Manager as _, Peripheral as _, WriteType, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};

use uuid::Uuid;

lazy_static! {
    static ref UUID_WRITE_DATA: Uuid =
        Uuid::parse_str("0000fa02-0000-1000-8000-00805f9b34fb").unwrap();
    static ref UUID_READ_DATA: Uuid =
        Uuid::parse_str("0000fa03-0000-1000-8000-00805f9b34fb").unwrap();
}

async fn find_device(central: &Adapter) -> Option<Peripheral> {
    central.start_scan(ScanFilter::default()).await.unwrap();

    thread::sleep(Duration::from_secs(2));

    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("IDM"))
        {
            return Some(p);
        }
    }
    None
}

fn find_characteristic(
    chars: &std::collections::BTreeSet<Characteristic>,
    uuid: Uuid,
) -> &Characteristic {
    chars
        .iter()
        .find(|c| c.uuid == uuid)
        .expect("Unable to find characterics")
}

pub(crate) struct BluetoothWrapper {
    device: Peripheral,
    write_char: Characteristic,
}

impl BluetoothWrapper {
    pub async fn new() -> Self {
        let manager = Manager::new().await.unwrap();
        // get the first bluetooth adapter
        let adapter = manager
            .adapters()
            .await
            .expect("Unable to fetch adapter list.")
            .into_iter()
            .next()
            .expect("Unable to find adapters.");

        let device = find_device(&adapter).await.unwrap();

        println!("Found device: {:?}", device.properties().await.unwrap());

        device.connect().await.unwrap();

        device.discover_services().await.unwrap();

        let chars = device.characteristics();
        let write_char = find_characteristic(&chars, *UUID_WRITE_DATA);

        println!("Found write characteristic: {:#?}", write_char);

        Self {
            device,
            write_char: write_char.clone(),
        }
    }

    pub async fn send_command(&self, data: &crate::commands::Command) {
        let chunks = data.to_bytes();
        let chunks = chunks.chunks(20);

        let write_type = if chunks.len() > 1 {
            WriteType::WithoutResponse
        } else {
            WriteType::WithResponse
        };

        for chunk in chunks {
            println!("Sending: {:x?}", chunk);

            self.device
                .write(&self.write_char, chunk, write_type)
                .await
                .unwrap();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
