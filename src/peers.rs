use std::collections::HashMap;

use defguard_wireguard_rs::{
    error::WireguardInterfaceError, host::Peer, key::Key, InterfaceConfiguration, WGApi,
    WireguardInterfaceApi,
};

pub struct WG(WGApi);

impl WG {
    pub fn init(ifname: &str) -> Result<Self, WireguardInterfaceError> {
        let wgapi = WGApi::new(ifname.into(), false)?;
        wgapi.create_interface()?;
        let host = wgapi.read_interface_data()?;
        println!("{:?}", host);

        let interface_config = InterfaceConfiguration {
            name: ifname.into(),
            prvkey: "eGzYWppafb7VQ+Z4PUQmzhgNabzzjcbnpMj8FfZDwVQ=".into(),
            address: "10.1.1.1".into(),
            port: 5280,
            peers: vec![],
        };
        wgapi.configure_interface(&interface_config)?;
        let host = wgapi.read_interface_data()?;
        println!("{:?}", host);
        Ok(Self(wgapi))
    }

    pub fn close(&mut self) -> Result<(), WireguardInterfaceError> {
        self.0.remove_interface()?;
        Ok(())
    }

    pub fn add_peer(&mut self, peer: &Peer) -> Result<(), WireguardInterfaceError> {
        self.0.configure_peer(peer)?;
        Ok(())
    }

    pub fn remove_peer(&mut self, key: &Key) -> Result<(), WireguardInterfaceError> {
        self.0.remove_peer(key)?;
        Ok(())
    }

    pub fn get_peers(&self) -> Result<HashMap<Key, Peer>, WireguardInterfaceError> {
        Ok(self.0.read_interface_data()?.peers)
    }
}
