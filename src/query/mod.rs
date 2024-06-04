mod endpoint;
use crate::peers::WG;
use crate::structure::QueryBody;
use crate::structure::ResponseBody;
use crate::structure::ResponseCode;
use crate::IpAddr;
use crate::Opcode;
use defguard_wireguard_rs::key::Key;
use defguard_wireguard_rs::{self, error::WireguardInterfaceError};

pub fn query_handle(
    buf: &[u8],
    opcode: &Opcode,
    wg: &mut WG,
) -> Result<(ResponseCode, ResponseBody), WireguardInterfaceError> {
    let body = match opcode {
        Opcode::EndpointQuery => {
            assert_eq!(buf.len(), 36);
            let mut pub_key = [0; 32];
            pub_key.copy_from_slice(&buf[4..36]);
            QueryBody::EndpointQuery(pub_key)
        }
        _ => todo!("Implement other query types"),
    };

    match body {
        QueryBody::EndpointQuery(pub_key_bytes) => {
            println!("Endpoint query: {:?}", pub_key_bytes);
            let key = Key::new(pub_key_bytes);
            println!("Key: {:?}", key.to_string());
            match endpoint::endpoint(wg, &key)? {
                Some(ip) => {
                    println!("Endpoint found: {:?}", ip);
                    Ok((
                        ResponseCode::NoError,
                        ResponseBody::EndpointResponse(Some(ip)),
                    ))
                }
                None => {
                    println!("Endpoint not found");
                    Ok((ResponseCode::DontKnow, ResponseBody::EndpointResponse(None)))
                }
            }
        }
        _ => todo!("Implement other query types"),
    }
}
