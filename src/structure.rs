use std::net::SocketAddr;

// Header
// 0 1 2 3 4 5 6 7 8 9 A B C D E F
// [Random MSG ID (16)]
// [QR (1)] [OPCODE (3)] [RSVD (9)] [RCODE (3)]
//
//
// • MSG ID A random 16bit message ID, a response should have the same ID.
// • QR Query or response bit, 0 for query 1 for response.
// • OPCODE Specifies the message type. And therefore dictates the message body.
// 1. 0 Endpoint query
// 2. 1 Provision query
// 3. 2 Establish connection
// • RSVD Reserved for future use
// • RCODE The response code.
// 1. 0 No error
// 2. 1 Format error

#[derive(Clone, Copy)]
pub struct Header {
    pub msg_id: u16,
    pub qr: u8,
    pub opcode: Opcode,
    pub rcode: ResponseCode,
}

pub const HEADER_SIZE: usize = 4;
pub const QUERY: u8 = 0;
pub const RESPONSE: u8 = 1;

impl Header {
    pub fn new(msg_id: u16, qr: u8, opcode: Opcode, rcode: ResponseCode) -> Self {
        Self {
            msg_id,
            qr,
            opcode,
            rcode,
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let mut bytes = [0; 4];
        bytes[0] = (self.msg_id >> 8) as u8;
        bytes[1] = self.msg_id as u8;
        bytes[2] = (self.qr << 7) | ((self.opcode as u8) << 4) | (self.rcode as u8);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            msg_id: (bytes[0] as u16) << 8 | bytes[1] as u16,
            qr: bytes[2] >> 7,
            opcode: match (bytes[2] >> 4) & 0b00000111 {
                0 => Opcode::EndpointQuery,
                1 => Opcode::ProvisionQuery,
                2 => Opcode::EstablishConnection,
                _ => Opcode::EndpointQuery,
            },
            rcode: (bytes[2] & 0b00000111).into(),
        }
    }
}

/// OPCODE Specifies the message type. And therefore dictates the message body.
/// 1. 0 Endpoint query
/// 2. 1 Provision query
/// 3. 2 Establish connection
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    EndpointQuery,
    ProvisionQuery,
    EstablishConnection,
}

/// Body
/// 2.1 Endpoint Query
/// Query for the endpoint of a given public key (256 bits).
/// 2.1.1 Query
/// This body is a 256 bit pub key of the desired peer’s endpoint
/// 2.1.2 Response
/// first bit dictates if the endpoint is ipv4 or ipv6 (0 is 4 and 1 is 6).
/// 0 1 2 3 4 5 6 7 8 9 A B C D E F
/// [V (1)] [RSVD (15)]
/// [PORT (16)]
/// [IPv4 (32) / IPv6 (128)]
///
/// • V 0 for ipv4 and 1 for ipv6
pub enum QueryBody {
    EndpointQuery([u8; 32]),
    ProvisionQuery {
        msg_id: u16,
        query_bit: u8,
        opcode: Opcode,
        domain_name: String,
    },
    EstablishConnection {
        msg_id: u16,
        query_bit: u8,
        opcode: Opcode,
        domain_name: String,
    },
}

/// RCODE The response code.
/// 1. 0 No error
/// 2. 1 Format error
#[derive(Clone, Copy)]
pub enum ResponseCode {
    NoError,
    FormatError,
    DontKnow,
}

impl From<u8> for ResponseCode {
    fn from(code: u8) -> Self {
        match code {
            0 => ResponseCode::NoError,
            1 => ResponseCode::FormatError,
            _ => ResponseCode::DontKnow,
        }
    }
}

pub enum ResponseBody {
    EndpointResponse(Option<SocketAddr>),
    ProvisionResponse {
        msg_id: u16,
        query_bit: u8,
        opcode: Opcode,
        domain_name: String,
        ip_address: String,
    },
    EstablishConnectionResponse {
        msg_id: u16,
        query_bit: u8,
        opcode: Opcode,
        domain_name: String,
        ip_address: String,
    },
}

pub enum EndpointBytes {
    V4([u8; 7]),
    V6([u8; 19]),
}
impl ResponseBody {
    pub fn to_bytes(&self) -> EndpointBytes {
        match self {
            ResponseBody::EndpointResponse(ip) => match ip {
                Some(SocketAddr::V4(socket_addr)) => {
                    let ip_v: u8 = 0;
                    let port: [u8; 2] = socket_addr.port().to_be_bytes();
                    let ip: &[u8; 4] = &socket_addr.ip().octets();
                    let bytes: [u8; 7] = [ip_v, port[0], port[1], ip[0], ip[1], ip[2], ip[3]];
                    EndpointBytes::V4(bytes)
                }
                Some(SocketAddr::V6(socket_addr)) => {
                    let ip_v: u8 = 1;
                    let port: [u8; 2] = socket_addr.port().to_be_bytes();
                    let ip: &[u8; 16] = &socket_addr.ip().octets();
                    let bytes: [u8; 19] = [
                        ip_v, port[0], port[1], ip[0], ip[1], ip[2], ip[3], ip[4], ip[5], ip[6],
                        ip[7], ip[8], ip[9], ip[10], ip[11], ip[12], ip[13], ip[14], ip[15],
                    ];
                    EndpointBytes::V6(bytes)
                }
                None => {
                    todo!();
                }
            },
            _ => todo!(),
        }
    }
}
