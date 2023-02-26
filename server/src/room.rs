use std::sync::Arc;

use models::Thermometer;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Room {
    pub devices: Arc<Mutex<Vec<Thermometer>>>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            devices: Arc::new(Mutex::new(vec![
                Thermometer::new(),
                Thermometer::new(),
                Thermometer::new(),
                Thermometer::new(),
                Thermometer::new(),
            ])),
        }
    }

    pub async fn uniq_devices(&mut self) -> &Self {
        let mut devices = self.devices.lock().await;

        for (index, device) in devices.iter_mut().enumerate() {
            device.name = format!("Device {}", index + 1);
        }

        self
    }

    pub async fn get_report(&mut self) -> String {
        let mut report = String::new();

        let mut devices = self.devices.lock().await;

        devices.sort_by(|a, b| a.status.partial_cmp(&b.status).unwrap());

        for device in devices.iter() {
            report.push_str(format!("{}\n", device.get_data()).as_str())
        }

        report
    }

    pub async fn power_off(&mut self, name: String) -> &Self {
        let mut devices = self.devices.lock().await;

        for device in devices.iter_mut() {
            if device.name == name {
                device.power_off();
            }
        }
        self
    }

    pub async fn power_on(&mut self, name: String) -> &Self {
        let mut devices = self.devices.lock().await;

        for device in devices.iter_mut() {
            if device.name == name {
                device.power_on();
            }
        }
        self
    }
}
