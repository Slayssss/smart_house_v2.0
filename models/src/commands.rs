use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
    ShowDevices,
    PowerOff,
    PowerOn,
    Unknown,
}

// "exit" => break,
//           "show_devices" => {
//               println!("Show devices")
//           }
//           "power-off" => {
//               println!("Power off")
//           }
//           "power-on" => {
//               println!("Power on")
//           }
//           _ => {
//               println!("Unknown command")
//           }
