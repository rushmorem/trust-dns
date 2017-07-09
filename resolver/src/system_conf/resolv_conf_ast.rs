use std::fmt::{Debug, Formatter, Error};
use std::net::IpAddr;

use trust_dns::rr::Name;

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigOption {
    Basic(BasicOption),
    Advanced(AdvancedOption),
}

#[derive(Debug, Eq, PartialEq)]
pub enum BasicOption {
    /// Name server IP address
    Nameserver(Vec<IpAddr>),
    /// Local domain name
    Domain(Name),
    /// Search list for host-name lookup
    Search(Vec<Name>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum AdvancedOption {
    /// Defaults to 1, where this number of dots is required before attempting the lookup as a FQDN.
    NumberOfDots(usize),
    /// Timeout in seconds
    Timeout(usize),
    /// Number of attempts before giving up on requests
    Attempts(usize),
}