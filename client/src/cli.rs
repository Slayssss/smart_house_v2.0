use std::string::String;

use clap::{Arg, Command};
use models::commands::Command as CommandModel;
use models::Request;

pub fn run() -> Option<Request> {
    let matches = clap::Command::new("Client for Smart House")
          .author("Kirill")
          .about(r" ________  _____ ______   ________  ________  _________  ___  ___  ________  ___  ___  ________  _______      
          |\   ____\|\   _ \  _   \|\   __  \|\   __  \|\___   ___\\  \|\  \|\   __  \|\  \|\  \|\   ____\|\  ___ \     
          \ \  \___|\ \  \\\__\ \  \ \  \|\  \ \  \|\  \|___ \  \_\ \  \\\  \ \  \|\  \ \  \\\  \ \  \___|\ \   __/|    
           \ \_____  \ \  \\|__| \  \ \   __  \ \   _  _\   \ \  \ \ \   __  \ \  \\\  \ \  \\\  \ \_____  \ \  \_|/__  
            \|____|\  \ \  \    \ \  \ \  \ \  \ \  \\  \|   \ \  \ \ \  \ \  \ \  \\\  \ \  \\\  \|____|\  \ \  \_|\ \ 
              ____\_\  \ \__\    \ \__\ \__\ \__\ \__\\ _\    \ \__\ \ \__\ \__\ \_______\ \_______\____\_\  \ \_______\
             |\_________\|__|     \|__|\|__|\|__|\|__|\|__|    \|__|  \|__|\|__|\|_______|\|_______|\_________\|_______|
             \|_________|                                                                          \|_________|     ")
        .subcommand(Command::new("show").about("Get a list of devices in room"))
        .subcommand(Command::new("power-off").about("Power off the device").arg( Arg::new("device").required(true)))
        .subcommand(Command::new("power-on").about("Power on the device").arg( Arg::new("device").required(true)))
        .subcommand(Command::new("exit").about("Close session"))
        .get_matches();

    let request: Option<Request> = matches
        .subcommand()
        .map(|(command, args)| {
            match (command, args) {
                ("exit", _) => {
                    // exit the loop if the "exit" command is entered
                    std::process::exit(0);
                }
                ("show", _) => Request {
                    cmd: CommandModel::ShowDevices,
                    name: "none".to_string(),
                },
                ("power-off", arg) => Request {
                    cmd: CommandModel::PowerOff,
                    name: arg.get_one::<String>("device").unwrap().to_string(),
                },
                ("power-on", arg) => Request {
                    cmd: CommandModel::PowerOn,
                    name: arg.get_one::<String>("device").unwrap().to_string(),
                },
                _ => {
                    eprintln!("Invalid command: {:?}", matches);
                    return None;
                }
            }
            .into()
        })
        .unwrap();

    request
}
