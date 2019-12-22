use std::io;
use std::net::UdpSocket;
use std::str;

pub fn connect(address: &str) -> Result<(), failure::Error> {
  let socket = UdpSocket::bind("127.0.0.1:0")?;
  loop {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    socket.send_to(input.as_bytes(), &address)?;

    let mut buf = [0u8; 1024];
    let (size, _) = socket.recv_from(&mut buf).expect("failed to receive");
    print!(
      "{}",
      str::from_utf8(&buf[..size]).expect("failed to convert to String")
    );
  }
}