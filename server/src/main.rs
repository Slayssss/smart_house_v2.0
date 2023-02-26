use std::error::Error;

use models::{commands::Command, Request};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
mod room;

use room::Room;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on {:?}", listener);

    let mut room = Room::new();
    room.uniq_devices().await;

    loop {
        let (socket, _) = listener.accept().await?;

        let cloned_room = room.clone();

        tokio::spawn(async move {
            let _ = handle_client(socket, cloned_room).await;
        });
    }
}

async fn handle_client(mut socket: TcpStream, room: Room) {
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await.unwrap();

    // Десериализация полученных данных в структуру
    let request: Request = serde_json::from_slice(&buf[..n]).unwrap();

    // Обработка полученной структуры
    let response = parse_request(request, room).await;

    // Отправка ответа обратно клиенту
    socket.write_all(response.as_bytes()).await.unwrap();
}

async fn parse_request(request: Request, mut room: Room) -> String {
    let response: String = match request.cmd {
        Command::ShowDevices => room.get_report().await,
        Command::PowerOff => {
            room.power_off(request.name).await;
            room.get_report().await
        }
        Command::PowerOn => {
            room.power_on(request.name).await;
            room.get_report().await
        }
        Command::Unknown => "Unknown Command".to_string(),
    };

    response
}
