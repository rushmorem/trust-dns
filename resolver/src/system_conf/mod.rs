//! System configuration loading
//!
//! This module is resposible for parsing and returning the configuration from
//!  the host system. It will read from the default location on each operating
//!  system, e.g. most Unixes have this written to `/etc/resolv.conf`
#![allow(missing_docs)]

use std::io::BufRead;

/// resolv.conf parser
// TODO: make crate only...
pub mod resolv_conf;
mod resolv_conf_ast;

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::net::*;
    use std::time::Duration;
    use trust_dns::rr::Name;
    use super::*;
    use super::resolv_conf_ast::*;

    #[test]
    fn test_comment() {
        let mut errors = Vec::new();
        resolv_conf::parse_comment(&mut errors, "#").unwrap();
        resolv_conf::parse_comment(&mut errors, ";").unwrap();
        resolv_conf::parse_comment(&mut errors, "#junk").unwrap();
        resolv_conf::parse_comment(&mut errors, "# junk").unwrap();
        resolv_conf::parse_comment(&mut errors, ";junk").unwrap();
        resolv_conf::parse_comment(&mut errors, "; junk").unwrap();
    }

    #[test]
    fn test_basic_options() {
        let mut errors = Vec::new();
        assert_eq!(
            resolv_conf::parse_basic_option(&mut errors, "nameserver 127.0.0.1").expect("failed"),
            BasicOption::Nameserver(IpAddr::from_str("127.0.0.1").unwrap())
        );
        assert_eq!(
            resolv_conf::parse_basic_option(&mut errors, "search localnet.").expect("failed"),
            BasicOption::Search(vec![Name::from_labels(vec!["localnet"])])
        );
        assert_eq!(
            resolv_conf::parse_basic_option(&mut errors, "domain example.com.").expect("failed"),
            BasicOption::Domain(Name::from_labels(vec!["example", "com"]))
        );
    }

    #[test]
    fn test_ip_addr() {
        let mut errors = Vec::new();
        assert_eq!(
            resolv_conf::parse_ip_addr(&mut errors, "127.0.0.1").expect("failed"),
            Ipv4Addr::new(127, 0, 0, 1)
        );

        assert_eq!(
            resolv_conf::parse_ip_addr(&mut errors, "::1").expect("failed"),
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)
        );

        assert_eq!(
            resolv_conf::parse_ip_addr(&mut errors, "2001:db8:85a3:8d3:1319:8a2e:370:7348")
                .expect("failed"),
            Ipv6Addr::new(0x2001, 0xdb8, 0x85a3, 0x8d3, 0x1319, 0x8a2e, 0x370, 0x7348)
        );

        assert_eq!(
            resolv_conf::parse_ip_addr(&mut errors, "::ffff:192.0.2.128").expect("failed"),
            Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc000, 0x0280)
        );
    }

    #[test]
    fn test_ip_addrs() {
        let mut errors = Vec::new();
        assert_eq!(
            resolv_conf::parse_ip_addrs(&mut errors, "127.0.0.1 127.0.0.2").expect("failed"),
            vec![Ipv4Addr::new(127, 0, 0, 1), Ipv4Addr::new(127, 0, 0, 2)]
        );
    }

    #[test]
    fn test_name() {
        let mut errors = Vec::new();
        assert_eq!(
            resolv_conf::parse_name(&mut errors, ".").unwrap(),
            Name::from_labels::<String>(vec![])
        );

        assert_eq!(
            resolv_conf::parse_name(&mut errors, "com.").unwrap(),
            Name::from_labels(vec!["com"])
        );

        assert_eq!(
            resolv_conf::parse_name(&mut errors, "example.com.").unwrap(),
            Name::from_labels(vec!["example", "com"])
        );
    }

    #[test]
    fn test_names() {
        let mut errors = Vec::new();
        assert_eq!(
            resolv_conf::parse_names(&mut errors, "localnet. local.net. intra.local.net.").unwrap(),
            vec![
                Name::from_labels(vec!["localnet"]),
                Name::from_labels(vec!["local", "net"]),
                Name::from_labels(vec!["intra", "local", "net"]),
            ]
        );
    }

    #[test]
    fn test_config_line() {
        let mut errors = Vec::new();
        // no comment
        assert_eq!(
            resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1").expect("failed"),
            Some(ConfigOption::Basic(BasicOption::Nameserver(
                IpAddr::from_str("127.0.0.1").unwrap(),
            )))
        );

        // FIXME: add these tests back, need a custom Lexer for comments...
        // assert_eq!(
        //     resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1; a comment")
        //         .expect("failed"),
        //     Some(ConfigOption::Basic(BasicOption::Nameserver(
        //         IpAddr::from_str("127.0.0.1").unwrap(),
        //     )))
        // );

        // assert_eq!(
        //     resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1# a comment")
        //         .expect("failed"),
        //     Some(ConfigOption::Basic(BasicOption::Nameserver(
        //         IpAddr::from_str("127.0.0.1").unwrap(),
        //     )))
        // );

        // assert_eq!(
        //     resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1 #a comment")
        //         .expect("failed"),
        //     Some(ConfigOption::Basic(BasicOption::Nameserver(
        //         IpAddr::from_str("127.0.0.1").unwrap(),
        //     )))
        // );

        // assert_eq!(
        //     resolv_conf::parse_config_line(&mut errors, "nameserver 127.0.0.1 # a comment")
        //         .expect("failed"),
        //     Some(ConfigOption::Basic(BasicOption::Nameserver(
        //         IpAddr::from_str("127.0.0.1").unwrap(),
        //     )))
        // );

        // assert_eq!(
        //     resolv_conf::parse_config_line(&mut errors, "options ndots:8 # a comment")
        //         .expect("failed"),
        //     Some(ConfigOption::Advanced(
        //         vec![AdvancedOption::NumberOfDots(8)],
        //     ))
        // );

        // only comment
        assert_eq!(
            resolv_conf::parse_config_line(&mut errors, "# a comment").expect("failed"),
            None
        );
    }

    #[test]
    fn test_advanced_option() {
        let mut errors = Vec::new();
        assert_eq!(
            resolv_conf::parse_advanced_option(&mut errors, "ndots:8").expect("failed"),
            AdvancedOption::NumberOfDots(8)
        );

        assert_eq!(
            resolv_conf::parse_advanced_option(&mut errors, "timeout:8").expect("failed"),
            AdvancedOption::Timeout(Duration::from_secs(8))
        );

        assert_eq!(
            resolv_conf::parse_advanced_option(&mut errors, "attempts:8").expect("failed"),
            AdvancedOption::Attempts(8)
        );
    }

    #[test]
    fn test_advanced_options() {
        let mut errors = Vec::new();
        assert_eq!(
            resolv_conf::parse_advanced_options(&mut errors, "options ndots:8 timeout:8 attempts:8")
                .expect("failed"),
            vec![
                AdvancedOption::NumberOfDots(8),
                AdvancedOption::Timeout(Duration::from_secs(8)),
                AdvancedOption::Attempts(8),
            ]
        );
    }

    //     #[test]
    //     fn test_full_file() {
    //         let file = "
    // #
    // # This is an example
    // #
    // options ndots:8 timeout:8 attempts:8
    // domain example.com.
    // nameserver 2001:4860:4860::8888
    // nameserver 2001:4860:4860::8844
    // nameserver 8.8.8.8
    // nameserver 8.8.4.4
    // ";
    //         let configuration = vec![
    //             ConfigOption::Advanced(vec![
    //                 AdvancedOption::NumberOfDots(8),
    //                 AdvancedOption::Timeout(Duration::from_secs(8)),
    //                 AdvancedOption::Attempts(8),
    //             ]),
    //             ConfigOption::Basic(BasicOption::Domain(
    //                 Name::from_labels(vec!["example", "com"]),
    //             )),
    //             ConfigOption::Basic(BasicOption::Nameserver(
    //                 IpAddr::from_str("2001:4860:4860::8888").unwrap(),
    //             )),
    //             ConfigOption::Basic(BasicOption::Nameserver(
    //                 IpAddr::from_str("2001:4860:4860::8844").unwrap(),
    //             )),
    //             ConfigOption::Basic(BasicOption::Nameserver(
    //                 IpAddr::from_str("8.8.8.8").unwrap(),
    //             )),
    //             ConfigOption::Basic(BasicOption::Nameserver(
    //                 IpAddr::from_str("8.8.4.4").unwrap(),
    //             )),
    //         ];

    //         let mut errors = Vec::new();
    //         assert_eq!(
    //             resolv_conf::parse_config(&mut errors, file).expect("failed"),
    //             configuration
    //         );
    //     }
}