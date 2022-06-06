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
use log::*;

use esp_idf_hal::prelude::*;

static mut tx:Option<Sender<u8>> = None;
static mut rx:Option<Receiver<u8>> = None;
static mut listener:Option<TcpListener> = None;

/// This configuration is picked up at compile time by `build.rs` from the
/// file `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

use std::sync::mpsc::{channel, Sender, Receiver};

fn handle_client(mut stream: TcpStream, tx_owned:Sender<u8>) {
    let mut data = [0 as u8; 256]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            for n in 0..size {
                println!("Sending to queue: {}", data[n]);
                tx_owned.send(data[n]).unwrap();
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
            _ip_settings,
        ))),
        _,
    ) = status
    {
        println!("Wifi connected");
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

pub fn bind_keyboard() -> Option<Receiver<u8>> {
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

    listener = Some(TcpListener::bind("0.0.0.0:80").unwrap());
    listener.unwrap().set_nonblocking(true).expect("Cannot set non-blocking");
    let (tx_local, rx_local) = channel();
    tx = Some(tx_local);
    rx = Some(rx_local);
    rx
}

pub fn spawn_listener() {
    let tx_owned = tx.to_owned();
    thread::spawn(move|| {
        // Receive new connection
        for stream in listener.unwrap().incoming() {
            match stream {
                Ok(stream) => {
                    let tx_owned = tx_owned.unwrap().clone();
                    thread::spawn(move|| {
                        // connection succeeded
                        handle_client(stream, tx_owned)
                    });
                }
                Err(e) => {
                }
            }
        }
    });

}


