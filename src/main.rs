mod message;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tokio::io::{self, AsyncWriteExt as _};
use tokio::net::UdpSocket;
use bytes::{Buf, BufMut, BytesMut};
use crate::message::Message;
use crate::message::handler::Handler;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = SocketAddr::new(IpAddr::from(Ipv4Addr::UNSPECIFIED), 5335);
    let socket = UdpSocket::bind(&addr).await?;
    // let socket = UdpSocket::bind(("0.0.0.0", 5335)).await?;
    let upstream = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(198,41,0,4)), 53);

    loop {
        println!("---------------------------");
        let (request, from) = get_message(&socket).await?;
        send_message(&socket, request.clone(), upstream).await?;
        let (response, _) = get_message(&socket).await?;
        //let response = Handler::get_response(&request);
        send_message(&socket, response.clone(), from).await?;
        println!("---------------------------");
    }
}

async fn get_message(socket : &UdpSocket) -> io::Result<(Message, SocketAddr)> {
    let mut buffer = BytesMut::zeroed(4096);
    println!("Waiting for new message.");
    let (bytes, from) = socket.recv_from(&mut buffer).await?;
    let message = Message::from(&mut buffer);
    println!("Received {} bytes from {}", bytes, from);
    Ok((message, from))
}

async fn send_message(socket : &UdpSocket, message : Message, address : SocketAddr) -> io::Result<()> {
    println!("Sending message to {}", address);
    let buffer = BytesMut::from(message);
    let length = buffer.len();
    let bytes = socket.send_to(&buffer[..length], address).await?;
    println!("Sent {} bytes to {}", bytes, address);
    Ok(())
}