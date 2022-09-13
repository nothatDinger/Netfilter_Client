use std::str::FromStr;
use ipnetwork::Ipv4Network;
use crate::common::{TransportType, MyParseError};
#[derive(Debug)]
pub struct Rule{
    src_net : Ipv4Network,
    dst_net : Ipv4Network, 
    src_port: u16,
    dst_port: u16,
    protocol: TransportType,
    action: u8,
    log: u8,
}

// impl ToString for Rule {
//     fn to_string(&self) -> String {
//         format!("Rule: {} {} {} {} {} {} {}",
//             self.src_net, self.dst_net, self.src_port, self.dst_port,
//             self.protocol, self.action, self.log
//         )
//     }
// }

impl FromStr for Rule {
    type Err = MyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim_start_matches("Rule: ").split_whitespace().collect();
        let src_net = words[0].parse::<Ipv4Network>()?;
        let dst_net = words[1].parse::<Ipv4Network>()?;
        let src_port = words[2].parse::<u16>()?;
        let dst_port = words[3].parse::<u16>()?;
        let protocol = words[4].parse::<TransportType>()?;
        let action = words[5].parse::<u8>()?;
        let log = words[6].parse::<u8>()?;
        Ok( Rule{ src_net, dst_net, src_port, dst_port, protocol, action, log })
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "{}",format!("Rule: {} {}      {}    {}       {}      {}    {}",
                self.src_net, self.dst_net, self.src_port, self.dst_port,
                self.protocol, self.action, self.log)
        )
    }
}