mod message;

use std::fmt::Debug;
use tokio::net::UdpSocket;
use std::io;
use std::io::BufRead;
use bytes::{Buf, BytesMut};

use crate::message::{Message, MessageBuilder};
#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind(("127.0.0.1", 5335)).await?;

    loop {
        let mut recv_buf = BytesMut::zeroed(512);
        let (bytes, from) = sock.recv_from(&mut recv_buf).await?;
        let request = Message::from(&mut recv_buf);
        //sock.send_to(&request_buf[..bytes], from).await?;
        println!("Received {} bytes from {}", bytes, from);
        println!("Header: {:?}", request.header);
        println!("Questions {:?}", request.questions);
        println!("Answers: {:?}", request.answers);
        println!("Additional Records: {:?}", request.additional_records);
    }
}