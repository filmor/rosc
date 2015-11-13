extern crate rosc;

use std::{net, env, process};
use rosc::types::{OscPacket};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage ./receive IP:PORT");
        process::exit(1);
    }

    let (ip, port) = rosc::utils::parse_ip_and_port(&args[1]).unwrap();
    let sock = net::UdpSocket::bind((ip, port)).unwrap();
    println!("Listening to {}:{}", ip, port);

    let mut buf = [0u8; rosc::decoder::MTP];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("Received packet with size {} from: {}", size, addr);
                let packet = rosc::decoder::decode(&mut buf).unwrap();
                handle_packet(packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }

    drop(sock);
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            match msg.args {
                Some(args) => {
                    println!("OSC arguments: {:?}", args);
                }
                None => println!("No arguments in message."),
            }
        }
        _ => (),
    }
}
