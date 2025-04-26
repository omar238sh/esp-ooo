use std::io::Read;
use std::net::TcpListener;

use embedded_svc::wifi::*;
use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::wifi::*;

use heapless::String;
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let p = Peripherals::take().unwrap();

    let sys_loop = EspSystemEventLoop::take().unwrap();
    let default_nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = EspWifi::new(p.modem, sys_loop.clone(), Some(default_nvs)).unwrap();

    let mut ssid: String<32> = String::new();
    let mut password: String<64> = String::new();
    ssid.push_str("esp32 ooo").unwrap();
    password.push_str("01000000").unwrap();
    let ap_config = Configuration::AccessPoint(AccessPointConfiguration {
        ssid,
        password,
        channel: 1,
        ..Default::default()
    });
    wifi.set_configuration(&ap_config).unwrap();
    wifi.start().unwrap();
    println!("✅ Access Point Started! Connect to Wi-Fi network: ESP32-Control");
    let mut pin2 = PinDriver::output(p.pins.gpio2).unwrap();
    let mut pin3 = PinDriver::output(p.pins.gpio4).unwrap();
    let mut pin4 = PinDriver::output(p.pins.gpio16).unwrap();
    let mut pin5 = PinDriver::output(p.pins.gpio17).unwrap();
    let mut pin6 = PinDriver::output(p.pins.gpio5).unwrap();
    let mut pin7 = PinDriver::output(p.pins.gpio18).unwrap();
    let mut pin8 = PinDriver::output(p.pins.gpio19).unwrap();
    let mut pin9 = PinDriver::output(p.pins.gpio21).unwrap();
    let mut pin10 = PinDriver::output(p.pins.gpio3).unwrap();
    let mut pin11 = PinDriver::output(p.pins.gpio1).unwrap();
    let listener = TcpListener::bind("0.0.0.0:5351").unwrap();
    println!("📡 TCP server listening on port 5351");
    let ip_info = wifi.ap_netif().get_ip_info().unwrap();
    println!("📡 ESP32 Access Point IP Address: {}", ip_info.ip);
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                let mut buf = [0u8; 128];
                if let Ok(size) = stream.read(&mut buf) {
                    let received = std::str::from_utf8(&buf[..size]).unwrap_or("");
                    println!("📨 Received: {}", received);
                    match received.trim() {
                        "2" => {
                            pin2.set_high().unwrap();
                        }
                        "s2" => {
                            pin2.set_low().unwrap();
                        }
                        "3" => {
                            pin3.set_high().unwrap();
                        }
                        "s3" => {
                            pin3.set_low().unwrap();
                        }
                        "4" => {
                            pin4.set_high().unwrap();
                        }
                        "s4" => {
                            pin4.set_low().unwrap();
                        }
                        "5" => {
                            pin5.set_high().unwrap();
                        }
                        "s5" => {
                            pin5.set_low().unwrap();
                        }
                        "6" => {
                            pin6.set_high().unwrap();
                        }
                        "s6" => {
                            pin6.set_low().unwrap();
                        }
                        "7" => {
                            pin7.set_high().unwrap();
                        }
                        "s7" => {
                            pin7.set_low().unwrap();
                        }
                        "8" => {
                            pin8.set_high().unwrap();
                        }
                        "s8" => {
                            pin8.set_low().unwrap();
                        }
                        "9" => {
                            pin9.set_high().unwrap();
                        }
                        "s9" => {
                            pin9.set_low().unwrap();
                        }
                        "10" => {
                            pin10.set_high().unwrap();
                        }
                        "s10" => {
                            pin10.set_low().unwrap();
                        }
                        "11" => {
                            pin11.set_high().unwrap();
                        }
                        "s11" => {
                            pin11.set_low().unwrap();
                        }
                        _ => {
                            println!("Unknown command: {}", received);
                        }
                    }
                }
            }
            Err(e) => {
                println!("❌ Error accepting connection: {:?}", e);
            }
        }
    }
}
