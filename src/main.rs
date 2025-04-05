mod message;

use std::fmt::Debug;
use tokio::net::UdpSocket;
use std::io;
use std::io::BufRead;
use bytes::{Buf, BytesMut};

use crate::message::{Message, MessageBuilder};
use crate::message::handler::Handler;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind(("127.0.0.1", 5335)).await?;

    loop {
        let mut request_buf = BytesMut::zeroed(512);
        let (bytes, from) = sock.recv_from(&mut request_buf).await?;
        let request = Message::from(&mut request_buf);
        println!("Received {} bytes from {}", bytes, from);
        println!("Header: {:?}", request.header());
        println!("Questions {:?}", request.questions());
        println!("Answers: {:?}", request.answers());
        println!("Additional Records: {:?}", request.additional_records());
        let response = Handler::get_response(&request);
        let response_buf = BytesMut::from(response);
        sock.send_to(&response_buf[..], from).await?;
    }
}