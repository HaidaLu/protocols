use std::env;
use std::net::UdpSocket;
use std::str;

const LISTEN_PORT: u16 = 8902;
const SRC_PORT: u16 = 8901;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let dest_addr = args[1].clone();
        let num_msgs = args[2].clone();
        let num_of_messages = num_msgs.parse::<i32>().unwrap();
        let mut i = 0;
        loop {
            //let data_to_send = args[2].clone();
            let data_to_send = i.to_string();
            let src_addr = format!("0.0.0.0:{}", SRC_PORT);
            println!("binding to {} for sending", src_addr.as_str());
            let socket = UdpSocket::bind(src_addr).expect("bind should succeed");

            socket.set_broadcast(true).expect("set_broadcast to true should succeed");

            println!("broadcasting to {} data of {}", dest_addr.as_str(), data_to_send.as_str());
            socket
                .send_to(data_to_send.as_str().as_bytes(), format!("{}:{}", dest_addr.as_str(), LISTEN_PORT))
                .expect("couldn't send data");
            i += 1;
            if i > num_of_messages {
                break;
            }
        }

    } else {
        let listen_addr = format!("0.0.0.0:{}", LISTEN_PORT);
        println!("listening on {}...", listen_addr.as_str());
        let socket = UdpSocket::bind(listen_addr.as_str()).expect("bind should succeed");
        loop {
            let mut buf = [0; 100];
            let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("read should succeed");
            let filled_buf = &mut buf[..number_of_bytes];
            match str::from_utf8(filled_buf) {
                Ok(s) => println!("bytes are valid UTF8; string: {}", s),
                Err(_) => println!("bytes are not UTF8, raw bytes: {:?}", filled_buf),
            }
        }
    }
}
