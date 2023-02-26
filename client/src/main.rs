use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let request = cli::run().unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let serialized = serde_json::to_string(&request).unwrap();

    stream.write_all(serialized.as_bytes()).await?;

    // Получение ответа от сервера
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    let response = std::str::from_utf8(&buf[..n]).unwrap();
    println!("{}", response);

    Ok(())
}
