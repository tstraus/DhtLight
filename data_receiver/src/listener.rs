extern crate serde_json;

use std::str;
use std::net::UdpSocket;
use std::time::Duration;
use std::thread::sleep;
use serde_json::Value;

fn main() {
    let socket = UdpSocket::bind("192.168.0.107:1234").expect("couldn't bind to address");

    loop {
        let mut buf: [u8; 4096] = [0; 4096];
        let (num_bytes, _) = socket.recv_from(&mut buf).expect("Didn't receive data");

        if num_bytes > 0 {
            // convert to str and remove unused part of buffer
            let msg = str::from_utf8(&buf).unwrap();
            let (msg, _) = msg.split_at(msg.rfind('}').unwrap() + 1);

            // parse json
            let v: Value = serde_json::from_str(msg).unwrap();
            println!("light: {}", v["light"]);
            println!("temp: {}", v["temp"]);
            println!("humidity: {}", v["humidity"]);
            println!("heatIndex: {}", v["heatIndex"]);
        }

        sleep(Duration::from_millis(100));
    }
}