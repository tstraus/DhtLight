extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate chrono_tz;

#[macro_use]
extern crate serde_derive;

use std::str;
use std::net::UdpSocket;
use std::time::Duration;
use std::thread::sleep;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::{TimeZone, Utc};
use chrono_tz::US::Eastern;

#[derive(Serialize, Deserialize)]
struct Data {
    light: i32,
    temp: f32,
    humidity: f32,
    heat_index: f32,
}

fn main() {
    // create csv file and write data format
    let mut file = File::create("./data.csv").expect("couldn't create file");
    file.write("time, light, temp(F), humidity(%), heatIndex(F)\n".as_bytes()).unwrap();
    drop(file);

    // create socket and buffer
    let socket = UdpSocket::bind("tstra.us:3420").expect("couldn't bind to address");
    let mut buf: [u8; 4096];

    loop {
        // clear buffer and listen for data
        buf = [0; 4096];
        let (num_bytes, _) = socket.recv_from(&mut buf).expect("didn't receive data");
        
        if num_bytes > 0 {
            // convert to str and remove unused part of buffer
            let msg = str::from_utf8(&buf).unwrap();
            let (msg, _) = msg.split_at(msg.rfind('}').unwrap() + 1);

            // time of the sample
            let utc = Utc::now().naive_utc();
            let time = Eastern.from_utc_datetime(&utc);
            println!("{}\n", time);

            // parse json
            let data: Data = serde_json::from_str(msg).unwrap();
            println!("light: {}", data.light);
            println!("temp: {}", data.temp);
            println!("humidity: {}", data.humidity);
            println!("heat_index: {}", data.heat_index);

            // append data to csv file
            file = OpenOptions::new().append(true).open("./data.csv").expect("couldn't open file");
            write!(file, "{}, {}, {}, {}, {}\n", time, data.light, data.temp, data.humidity, data.heat_index).unwrap();
        }

        sleep(Duration::from_millis(100));
    }
}
