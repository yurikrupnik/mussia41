use crate::get_port;
use serde::{Deserialize, Serialize};
use std::io;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};

#[derive(Serialize, Deserialize)]
pub enum WebUrl {
    Address(Ipv4Addr, u16),
    AddressString(String),
}

// impl WebUrl {
//     fn into(&self) -> io::Result<String> {
//         match self {
//             WebUrl::Address(ip, port) => Ok("".to_string()),
//             WebUrl::AddressString(s) => Ok(s.to_string()),
//         }
//     }
// }

impl ToSocketAddrs for WebUrl {
    type Iter = std::vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        match self {
            WebUrl::Address(_ip, port) => {
                let addr = SocketAddr::new((*_ip).into(), *port);
                Ok(vec![addr].into_iter())
            }
            WebUrl::AddressString(s) => s.to_socket_addrs(),
        }
    }
}

pub fn get_web_url(return_string: bool) -> WebUrl {
    let port = get_port(); // Assuming get_port() is defined elsewhere and returns u16
    if return_string {
        // fails for axum - using get_web_url_v1
        WebUrl::AddressString(get_web_url_v1())
    } else {
        WebUrl::Address(Ipv4Addr::UNSPECIFIED, port)
    }
}
pub fn get_web_url_v1() -> String {
    format!("{}:{}", Ipv4Addr::UNSPECIFIED, get_port())
}
