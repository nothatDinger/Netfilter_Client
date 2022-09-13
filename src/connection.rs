use std::str::FromStr;
use ipnetwork::Ipv4Network;
use crate::common::{TransportType, MyParseError};
pub struct Connection{
    src_net : Ipv4Network,
    dst_net : Ipv4Network, 
    src_port: u16,
    dst_port: u16,
    protocol: TransportType,
    time_alive: u32,
}

impl ToString for Connection {
    fn to_string(&self) -> String {
        format!("{} {} {} {} {} {}",
            self.src_net, self.dst_net, self.src_port, self.dst_port,
            self.protocol, self.time_alive
        )
    }
}

impl FromStr for Connection {
    type Err = MyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();
        let src_net = words[0].parse::<Ipv4Network>()?;
        let dst_net = words[1].parse::<Ipv4Network>()?;
        let src_port = words[2].parse::<u16>()?;
        let dst_port = words[3].parse::<u16>()?;
        let protocol = words[4].parse::<TransportType>()?;
        let time_alive = words[5].parse::<u32>()?;
        Ok( Connection{ src_net, dst_net, src_port, dst_port, protocol, time_alive})
    }
}