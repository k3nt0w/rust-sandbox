use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub fn serve(address: &str) -> Result<(), failure::Error> {
  // listening socket (server socket) for waiting to complete connection
  let server_socket = TcpListener::bind(address)?;
  loop {
    // client socket for messaging data
    let (stream, _) = server_socket.accept()?;
    thread::spawn(move || handler(stream).unwrap_or_else(|error| error!("{:?}", error)));
  }
}

fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
  debug!("Handling data from {}", stream.peer_addr()?);
  let mut buffer = [0u8; 1024];
  loop {
    let nbytes = stream.read(&mut buffer)?;
    if nbytes == 0 {
      debug!("Connection closed.");
      return Ok(());
    }
    print!("{}", str::from_utf8(&buffer[..nbytes])?);
    stream.write_all(&buffer[..nbytes])?;
  }
}

