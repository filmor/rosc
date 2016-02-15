extern crate rosc;
extern crate byteorder;

use byteorder::{ByteOrder, BigEndian};
use std::mem;
use std::ascii::AsciiExt;

use rosc::{types, errors, decoder, encoder};

#[test]
fn test_decode_no_args() {
    // message to build: /some/valid/address/4 ,
    let raw_addr = "/some/valid/address/4";
    let addr = encoder::encode_string(raw_addr);
    let type_tags = encoder::encode_string(",");
    let merged: Vec<u8> = addr.into_iter()
                              .chain(type_tags.into_iter())
                              .collect();
    let osc_packet: Result<types::OscPacket, errors::OscError> = decoder::decode(&merged);
    assert!(osc_packet.is_ok());
    match osc_packet {
        Ok(types::OscPacket::Message(msg)) => {
            assert_eq!(raw_addr, msg.addr);
            assert!(msg.args.is_none());
        }
        Ok(_) => panic!("Expected an OscMessage!"),
        Err(e) => panic!(e),
    }
}

#[test]
fn test_decode_args() {
    // /another/valid/address/123 ,fdih 3.1415 3.14159265359 12345678i32
    // -1234567891011
    let addr = encoder::encode_string("/another/valid/address/123");
    // args
    let f = 3.1415f32;
    let mut f_bytes: [u8; 4] = [0u8; 4];
    BigEndian::write_f32(&mut f_bytes, f);
    assert_eq!(BigEndian::read_f32(&f_bytes), f);

    let d = 3.14159265359f64;
    let mut d_bytes: [u8; 8] = [0u8; 8];
    BigEndian::write_f64(&mut d_bytes, d);
    assert_eq!(BigEndian::read_f64(&d_bytes), d);

    let i = 12345678i32;
    let i_bytes: [u8; 4] = unsafe { mem::transmute(i.to_be()) };

    let l = -1234567891011i64;
    let h_bytes: [u8; 8] = unsafe { mem::transmute(l.to_be()) };

    let blob_size: [u8; 4] = unsafe { mem::transmute(6u32.to_be()) };
    let blob: Vec<u8> = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8];

    let s = "I am an osc test string.";
    assert!(s.is_ascii());
    // Osc strings are null terminated like in C!
    let s_bytes: Vec<u8> = encoder::encode_string(s);

    let c = '$';
    let c_bytes: [u8; 4] = unsafe { mem::transmute((c as u32).to_be()) };

    let type_tags = encoder::encode_string(",fdsTFibhNIc");

    let args: Vec<u8> = f_bytes.iter()
                               .chain(d_bytes.iter())
                               .chain(s_bytes.iter())
                               .chain(i_bytes.iter())
                               .chain(blob_size.iter())
                               .chain(blob.iter())
                               .chain(vec![0u8, 0u8].iter())
                               .chain(h_bytes.iter())
                               .chain(c_bytes.iter())
                               .map(|x| *x)
                               .collect::<Vec<u8>>();

    let merged: Vec<u8> = addr.into_iter()
                              .chain(type_tags.into_iter())
                              .chain(args)
                              .collect::<Vec<u8>>();

    match decoder::decode(&merged).unwrap() {
        types::OscPacket::Message(msg) => {
            for arg in msg.args.unwrap() {
                match arg {
                    types::OscType::Int(x) => assert_eq!(i, x),
                    types::OscType::Long(x) => assert_eq!(l, x),
                    types::OscType::Float(x) => assert_eq!(f, x),
                    types::OscType::Double(x) => assert_eq!(d, x),
                    types::OscType::String(x) => assert_eq!(s, x),
                    types::OscType::Blob(x) => assert_eq!(blob, x),
                    // cant assign bool args to type_tag
                    // , so there is no real test wether the value is
                    // correct or not
                    types::OscType::Bool(_) => (),
                    types::OscType::Inf => (),
                    types::OscType::Nil => (),
                    // test time-tags, midi-messages and chars
                    types::OscType::Char(x) => assert_eq!(c, x),
                    _ => panic!(),
                }

            }
        }
        _ => panic!("Expected an OSC message!"),
    }
}
