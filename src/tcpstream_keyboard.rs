use std::{thread};

use std::result::Result::Ok;

use log::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;
use std::io::Write;

use std::sync::mpsc::{channel, Sender, Receiver};

pub fn bind_keyboard(port: u32) -> Receiver<u8> {
    let (tx, rx) = channel();

    thread::spawn(move|| {
        let bind_string = format!("0.0.0.0:{}", port);
        info!("Binding to {}", bind_string);
        let listener = TcpListener::bind(bind_string).unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        info!("Creating communication channel");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let tx_owned = tx.clone();
                    thread::spawn(move|| {
                        info!("Keyabord connection from client succeeded");
                        handle_client(stream, tx_owned)
                    });
                }
                Err(_e) => {
                }
            }
        }

    });
    rx
}


fn handle_client(mut stream: TcpStream, tx: Sender<u8>) {
  let mut data = [0 as u8; 256]; // using 50 byte buffer
  while match stream.read(&mut data) {
      Ok(size) => {
          // echo everything!
          stream.write(&data[0..size]).unwrap();
          for n in 0..size {
              info!("Sending to queue: {}", data[n]);
              tx.send(data[n]).unwrap();
          }
          true
      },
      Err(_) => {
          error!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
          stream.shutdown(Shutdown::Both).unwrap();
          false
      }
  } {}
}
