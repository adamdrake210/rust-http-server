use std::net::TcpListener;
use std::convert::TryFrom;
use crate::http::Request;
use std::io::Read;

pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { 
      addr
    }
  }
  
  pub fn run(self) {
    println!("Listening on {}", self.addr);

    let listener = TcpListener::bind(&self.addr).unwrap();

    loop {
        match listener.accept() {
          Ok((mut stream, _)) => {
            println!("Something is happening...");
            let mut buffer = [0; 1024]; // TODO this value could be too small
            match stream.read(&mut buffer) {
              Ok(_) => {
                println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                match Request::try_from(&buffer[..]) {
                  Ok(request) => {},
                  Err(e) => println!("Failed to parse a request: {}", e),
                }
                

              }
              Err(e) => println!("Failed to read from connection: {}", e)
            }

          },
          Err(e) => println!("Error:, {}", e)
        }
    }
  }
}