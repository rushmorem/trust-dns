use std::fmt::{Debug, Formatter, Error};
use std::net::IpAddr;
use std::time::Duration;

use trust_dns::rr::Name;

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigOption {
    /// BasicOptions are only list one per line
    Basic(BasicOption),
    /// Options are listed many per line...
    Advanced(Vec<AdvancedOption>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum BasicOption {
    /// Name server IP address
    Nameserver(IpAddr),
    /// Local domain name
    Domain(Name),
    /// Search list for host-name lookup
    Search(Vec<Name>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum AdvancedOption {
    /// Defaults to 1, where this number of dots is required before attempting the lookup as a FQDN.
    NumberOfDots(u8),
    /// Timeout in seconds
    Timeout(Duration),
    /// Number of attempts before giving up on requests
    Attempts(u8),
}