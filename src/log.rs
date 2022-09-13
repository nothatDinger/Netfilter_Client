use ipnetwork::Ipv4Network;
use crate::common::{TransportType, MyParseError};
use std::str::FromStr;
pub struct Log{
    src_net : Ipv4Network,
    dst_net : Ipv4Network, 
    src_port: u16,
    dst_port: u16,
    protocol: TransportType,
    action: u8,
}

impl ToString for Log {
    fn to_string(&self) -> String {
        format!("{} {} {} {} {} {}",
            self.src_net, self.dst_net, self.src_port, self.dst_port,
            self.protocol, self.action
        )
    }
}

impl FromStr for Log {
    type Err = MyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();
        let src_net = words[0].parse::<Ipv4Network>()?;
        let dst_net = words[1].parse::<Ipv4Network>()?;
        let src_port = words[2].parse::<u16>()?;
        let dst_port = words[3].parse::<u16>()?;
        let protocol = words[4].parse::<TransportType>()?;
        let action = words[5].parse::<u8>()?;
        Ok( Log{ src_net, dst_net, src_port, dst_port, protocol, action})
    }
}