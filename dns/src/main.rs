mod message;
mod resource_record;
mod traits;
mod resource_data;
mod question;
mod header;
mod types;
mod classes;

use std::fmt::Debug;
use tokio::net::UdpSocket;
use std::io;
use bytes::{Buf, BytesMut};

use message::DnsMessage;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind(("127.0.0.1", 5335)).await?;

    loop {
        let mut recv_buf = BytesMut::zeroed(512);
        let (bytes, from) = sock.recv_from(&mut recv_buf).await?;
        let request = DnsMessage::from(&mut recv_buf);
        println!("===============================================");
        println!("Received {} bytes from {}", bytes, from);
        println!("Header: {:?}", request.header);
        println!("Question: {:?}", request.question);
        println!("Answers: {:?}", request.answer);
        println!("Authority: {:?}", request.authority);
        println!("Additional: {:?}", request.additional);
        println!("===============================================");
        //sock.send_to(&recv_buf, from).await?;
    }
}