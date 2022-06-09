use std::{thread, time::*};

use std::result::Result::Ok;

use std::sync::Arc;
use embedded_svc::wifi::*;
use esp_idf_svc::wifi::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::sysloop::*;
use log::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::Read;
use std::io::Write;
use anyhow::*;

use std::sync::mpsc::{channel, Sender, Receiver};


pub struct TcpStreamKeyboard {
    tx:Sender<u8>,
    rx:Receiver<u8>,
    listener:TcpListener
}

pub trait Keyboard {
    // fn bind_keyboard(&self) -> Self;
    fn spawn_listener(&self);
    // fn handle_client(&self, stream: TcpStream);
}

impl Keyboard for TcpStreamKeyboard {

    // fn bind_keyboard(&self) -> Receiver<u8> {

    fn spawn_listener(&self) {
        let tx_owned = self.tx.to_owned();
        println!("Cloning listener");
        let listener_owned = self.listener.try_clone().unwrap();
        println!("Moving to thread");
        thread::spawn(move|| {
            // Receive new connection
            for stream in listener_owned.incoming() {
                match stream {
                    Ok(stream) => {
                        let tx_owned = tx_owned.clone();
                        thread::spawn(move|| {
                            println!("Keyabord connection from client succeeded");
                            handle_client(stream, tx_owned)
                        });
                    }
                    Err(e) => {
                    }
                }
            }
        });

    }

}


pub fn bind_keyboard(port: u32) -> Receiver<u8> {

    // wifi part
    #[allow(unused)]
    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    #[allow(unused)]
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    #[allow(unused)]
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());
    let _wifi = wifi(
        netif_stack.clone(),
        sys_loop_stack.clone(),
        default_nvs.clone(),
    ).unwrap();
    let (tx, rx) = channel();

    thread::spawn(move|| {
        let bind_string = format!("0.0.0.0:{}", port);
        println!("Binding to {}", bind_string);
        let listener = TcpListener::bind(bind_string).unwrap();
        listener.set_nonblocking(true).expect("Cannot set non-blocking");
        println!("Creating communication channel");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let tx_owned = tx.clone();
                    thread::spawn(move|| {
                        println!("Keyabord connection from client succeeded");
                        handle_client(stream, tx_owned)
                    });
                }
                Err(e) => {
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
              println!("Sending to queue: {}", data[n]);
              tx.send(data[n]).unwrap();
          }
          true
      },
      Err(_) => {
          println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
          stream.shutdown(Shutdown::Both).unwrap();
          false
      }
  } {}
}



/// This configuration is picked up at compile time by `build.rs` from the
/// file `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}



#[allow(dead_code)]
fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> anyhow::Result<Box<EspWifi>> {
    let app_config = CONFIG;
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: app_config.wifi_ssid.into(),
        password: app_config.wifi_psk.into(),
        auth_method: AuthMethod::None,
        ..Default::default()
    }))?;

    println!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    info!("to get status");
    let status = wifi.get_status();

    info!("got status)");
    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(
            ip_settings,
        ))),
        _,
    ) = status
    {
        println!("Wifi connected. IP address: {}", ip_settings.ip);
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}


