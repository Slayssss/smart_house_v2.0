use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod commands;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub cmd: commands::Command,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub value: f64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: chrono::DateTime<Utc>,
}

impl Data {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(-10.0..20.0);

        Self {
            value: value,
            time: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, PartialOrd)]
pub enum Status {
    PowerOn,
    PowerOff,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thermometer {
    pub name: String,
    pub data: Data,
    pub voltage: f64,
    pub status: Status,
}

impl Thermometer {
    pub fn new() -> Self {
        Thermometer {
            data: Data {
                value: 0.0,
                time: Utc::now(),
            },
            voltage: 0.0,
            status: Status::PowerOff,
            name: "Default".to_owned(),
        }
    }

    pub fn power_on(&mut self) -> &Self {
        self.status = Status::PowerOn;
        self.update()
    }

    pub fn power_off(&mut self) -> &Self {
        self.status = Status::PowerOff;
        self.update()
    }

    pub fn update(&mut self) -> &Self {
        match self.status {
            Status::PowerOff => {
                self.data = Data::new();
                self.voltage = 0.0;
            }
            Status::PowerOn => {
                let data = Data::new();
                let mut rng = rand::thread_rng();
                let value = rng.gen_range(40.0..80.0);

                self.data = data.clone();
                self.voltage = value;
            }
        }

        self
    }

    pub fn get_data(&self) -> String {
        let info_status = match self.status {
            Status::PowerOn => "\x1B[32mOn\x1B[0m",
            Status::PowerOff => "\x1B[31mOff\x1B[0m",
        };

        format!(
            "Device: {} | Status: {} | Temperature: {:.1}°C | Voltage: {:.1} kW | {}",
            self.name,
            info_status,
            self.data.value,
            self.voltage,
            self.data.time.format("%d/%m/%Y %H:%M:%S"),
        )
    }
}
