use crate::peers;
use defguard_wireguard_rs::{self, error::WireguardInterfaceError};

pub fn endpoint(
    wg: &peers::WG,
    key: &defguard_wireguard_rs::key::Key,
) -> Result<Option<core::net::SocketAddr>, WireguardInterfaceError> {
    let peers = wg.get_peers()?;
    let peer = peers
        .get(key)
        .map(|peer| peer.endpoint)
        .map_or(None, |endpoint| endpoint)
        .map_or(None, |endpoint| Some(endpoint));
    Ok(peer)
}
