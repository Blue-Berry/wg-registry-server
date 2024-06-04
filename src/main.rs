mod peers;
mod query;
mod structure;

// Uncomment this block to pass the first stage
use core::net::IpAddr;
use std::net::UdpSocket;

use crate::structure::{Opcode, ResponseCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wg = peers::WG::init("wg0")?;
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                assert!(size >= structure::HEADER_SIZE, "Message too short");
                println!("Received {} bytes from {}", size, source);

                let header = structure::Header::from_bytes(&buf[0..3]);
                if header.qr == structure::RESPONSE {
                    println!("Not a query");
                    continue;
                }
                if header.opcode != Opcode::EndpointQuery {
                    println!("Not an endpoint query");
                    continue;
                }
                match query::query_handle(&buf, &header.opcode, &mut wg) {
                    Err(e) => {
                        println!("Error handling query: {}", e);
                        continue;
                    }
                    Ok((ResponseCode::NoError, response_body)) => {
                        let response_header = structure::Header::new(
                            header.msg_id,
                            structure::RESPONSE,
                            Opcode::EndpointQuery,
                            ResponseCode::NoError,
                        );
                        let header_bytes = response_header.to_bytes();
                        match response_body.to_bytes() {
                            structure::EndpointBytes::V4(ip) => {
                                let mut bytes = [0; (6 + 4)];
                                bytes[..4].copy_from_slice(&header_bytes);
                                bytes[4..].copy_from_slice(&ip);
                                udp_socket
                                    .send_to(&bytes, source)
                                    .expect("Failed to send response");
                            }
                            structure::EndpointBytes::V6(ip) => {
                                let mut bytes = [0; (18 + 4)];
                                bytes[..4].copy_from_slice(&header_bytes);
                                bytes[4..].copy_from_slice(&ip);
                                udp_socket
                                    .send_to(&bytes, source)
                                    .expect("Failed to send response");
                            }
                        }
                    }
                    _ => {
                        todo!();
                    }
                };

                // udp_socket
                //     .send_to(&response, source)
                //     .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
    Ok(())
}
