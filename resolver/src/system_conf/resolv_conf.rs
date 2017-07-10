use std::str::FromStr;
use std::net::IpAddr;
use std::time::Duration;
use lalrpop_util::ErrorRecovery;
use trust_dns::rr::Name;
use system_conf::resolv_conf_ast::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__advanced_option {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, 0, 0, 4, 0, 0, 5, 0, 0, 0, 0,
        // State 1
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0,
        // State 5
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 6
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 7
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 8
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -1,
        0,
        0,
        0,
        -15,
        -41,
        -13,
        -14,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_advanced_option<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<AdvancedOption, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<AdvancedOption,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__advanced_option::parse_advanced_option;

mod __parse__advanced_options {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
        // State 1
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 2
        6, 0, 0, 7, 0, 0, 8, 0, 0, 0, 0,
        // State 3
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 4
        6, 0, 0, 7, 0, 0, 8, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 8
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 9
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 10
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 11
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 12
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -2,
        0,
        -16,
        -18,
        0,
        0,
        0,
        -17,
        -15,
        -41,
        -13,
        -14,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_advanced_options<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Vec<AdvancedOption>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<AdvancedOption>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__advanced_options::parse_advanced_options;

mod __parse__basic_option {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 3, 4, 0, 0, 5, 0, 0, 0, 0, 0,
        // State 1
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 5
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 6
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 7
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 8
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 9
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 11
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 12
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -3,
        0,
        0,
        0,
        -20,
        -37,
        -19,
        -33,
        -38,
        -40,
        -21,
        -39,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 12, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_basic_option<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<BasicOption, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<BasicOption,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__basic_option::parse_basic_option;

mod __parse__comment {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
        // State 1
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 2
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -4,
        -22,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_comment<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<(), __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<(),__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__comment::parse_comment;

mod __parse__config {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 9, 10, 0, 11, 12, 0, 0, 0, 0, 13,
        // State 1
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 2
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 3
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 4
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 5
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 6
        0, 9, 10, 0, 11, 12, 0, 0, 0, 0, 13,
        // State 7
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0,
        // State 10
        21, 0, 0, 22, 0, 0, 23, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0,
        // State 12
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 13
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 14
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 15
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 16
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 17
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 18
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 19
        21, -18, -18, 22, -18, -18, 23, 0, 0, 0, -18,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0,
        // State 23
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 24
        0, -40, -40, 0, -40, -40, 0, 0, 0, 16, -40,
        // State 25
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 26
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 27
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 28
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 29
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 30
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 31
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -23,
        -32,
        -31,
        -26,
        -5,
        -29,
        -24,
        -25,
        0,
        0,
        0,
        0,
        -22,
        -30,
        -20,
        -37,
        -19,
        -33,
        -16,
        -18,
        0,
        0,
        0,
        -38,
        -40,
        -21,
        -17,
        -15,
        -41,
        -13,
        -14,
        -39,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 0, 14, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 26, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_config<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Vec<ConfigOption>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<ConfigOption>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__config::parse_config;

mod __parse__config_line {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 7, 8, 0, 9, 10, 0, 0, 0, 0, 11,
        // State 1
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 2
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 3
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 4
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 5
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0,
        // State 8
        18, 0, 0, 19, 0, 0, 20, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 10
        -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 11
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 12
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 13
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 14
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 15
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 16
        18, 0, 0, 19, 0, 0, 20, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0,
        // State 20
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0,
        // State 22
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 23
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 24
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 25
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 26
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 27
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 28
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -32,
        -31,
        -26,
        -6,
        -25,
        0,
        0,
        0,
        0,
        -22,
        -20,
        -37,
        -19,
        -33,
        -16,
        -18,
        0,
        0,
        0,
        -38,
        -40,
        -21,
        -17,
        -15,
        -41,
        -13,
        -14,
        -39,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 0, 5, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22, 23, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_config_line<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Option<ConfigOption>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Option<ConfigOption>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__config_line::parse_config_line;

mod __parse__config_option {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 5, 6, 0, 7, 8, 0, 0, 0, 0, 0,
        // State 1
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 2
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 3
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0,
        // State 6
        15, 0, 0, 16, 0, 0, 17, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0,
        // State 8
        -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 9
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 10
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 11
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 12
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 13
        15, 0, 0, 16, 0, 0, 17, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0,
        // State 17
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0,
        // State 19
        -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 20
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 21
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 22
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
        // State 23
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 24
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 25
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -32,
        -31,
        -7,
        0,
        0,
        0,
        0,
        -20,
        -37,
        -19,
        -33,
        -16,
        -18,
        0,
        0,
        0,
        -38,
        -40,
        -21,
        -17,
        -15,
        -41,
        -13,
        -14,
        -39,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 19, 20, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_config_option<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<ConfigOption, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ConfigOption,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__config_option::parse_config_option;

mod __parse__ip_addr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
        // State 1
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 2
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -8,
        -33,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_ip_addr<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<IpAddr, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<IpAddr,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__ip_addr::parse_ip_addr;

mod __parse__ip_addrs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0,
        // State 1
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0,
        // State 3
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 4
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 5
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -34,
        -36,
        -9,
        -33,
        -35,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_ip_addrs<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Vec<IpAddr>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<IpAddr>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__ip_addrs::parse_ip_addrs;

mod __parse__name {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 2
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -10,
        -37,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_name<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Name, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Name,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__name::parse_name;

mod __parse__names {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 1
        -38, -38, -38, -38, -38, -38, -38, -38, -38, -38, -38,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 3
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 4
        -37, -37, -37, -37, -37, -37, -37, -37, -37, -37, -37,
        // State 5
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -38,
        -40,
        -11,
        -37,
        -39,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_names<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Vec<Name>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Name>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____u8(__nt), __end));
                11
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__names::parse_names;

mod __parse__u8 {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22attempts_3a_22(&'input str),
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22ndots_3a_22(&'input str),
        Term_22options_22(&'input str),
        Term_22search_22(&'input str),
        Term_22timeout_3a_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____advanced__option(AdvancedOption),
        Nt____advanced__options(Vec<AdvancedOption>),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____config(Vec<ConfigOption>),
        Nt____config__line(Option<ConfigOption>),
        Nt____config__option(ConfigOption),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Nt____u8(u8),
        Ntadvanced__option(AdvancedOption),
        Ntadvanced__option_2b(::std::vec::Vec<AdvancedOption>),
        Ntadvanced__options(Vec<AdvancedOption>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntconfig(Vec<ConfigOption>),
        Ntconfig__line(Option<ConfigOption>),
        Ntconfig__line_2a(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__line_2b(::std::vec::Vec<Option<ConfigOption>>),
        Ntconfig__option(ConfigOption),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
        Ntu8(u8),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
        // State 1
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 2
        -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -12,
        -41,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""attempts:""###,
            r###""domain""###,
            r###""nameserver""###,
            r###""ndots:""###,
            r###""options""###,
            r###""search""###,
            r###""timeout:""###,
            r###"r#"([0-9]+)"#"###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 11)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_u8<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<u8, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (4, _) if true => 0,
                (5, _) if true => 1,
                (6, _) if true => 2,
                (7, _) if true => 3,
                (8, _) if true => 4,
                (9, _) if true => 5,
                (10, _) if true => 6,
                (0, _) if true => 7,
                (1, _) if true => 8,
                (2, _) if true => 9,
                (3, _) if true => 10,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 11 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22attempts_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22ndots_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22options_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22timeout_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<u8,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // __advanced_option = advanced_option => ActionFn(4);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__option(__nt), __end));
                0
            }
            2 => {
                // __advanced_options = advanced_options => ActionFn(3);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____advanced__options(__nt), __end));
                1
            }
            3 => {
                // __basic_option = basic_option => ActionFn(5);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                2
            }
            4 => {
                // __comment = comment => ActionFn(8);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                3
            }
            5 => {
                // __config = config => ActionFn(0);
                let __sym0 = __pop_Ntconfig(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config(__nt), __end));
                4
            }
            6 => {
                // __config_line = config_line => ActionFn(1);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__line(__nt), __end));
                5
            }
            7 => {
                // __config_option = config_option => ActionFn(2);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____config__option(__nt), __end));
                6
            }
            8 => {
                // __ip_addr = ip_addr => ActionFn(9);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                7
            }
            9 => {
                // __ip_addrs = ip_addrs => ActionFn(6);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                8
            }
            10 => {
                // __name = name => ActionFn(10);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                9
            }
            11 => {
                // __names = names => ActionFn(7);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                10
            }
            12 => {
                // __u8 = u8 => ActionFn(11);
                let __sym0 = __pop_Ntu8(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            13 => {
                // advanced_option = "ndots:", u8 => ActionFn(18);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22ndots_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            14 => {
                // advanced_option = "timeout:", u8 => ActionFn(19);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22timeout_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            15 => {
                // advanced_option = "attempts:", u8 => ActionFn(20);
                let __sym1 = __pop_Ntu8(__symbols);
                let __sym0 = __pop_Term_22attempts_3a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option(__nt), __end));
                12
            }
            16 => {
                // advanced_option+ = advanced_option => ActionFn(34);
                let __sym0 = __pop_Ntadvanced__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            17 => {
                // advanced_option+ = advanced_option+, advanced_option => ActionFn(35);
                let __sym1 = __pop_Ntadvanced__option(__symbols);
                let __sym0 = __pop_Ntadvanced__option_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__option_2b(__nt), __end));
                13
            }
            18 => {
                // advanced_options = "options", advanced_option+ => ActionFn(17);
                let __sym1 = __pop_Ntadvanced__option_2b(__symbols);
                let __sym0 = __pop_Term_22options_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntadvanced__options(__nt), __end));
                14
            }
            19 => {
                // basic_option = "nameserver", ip_addr => ActionFn(21);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            20 => {
                // basic_option = "domain", name => ActionFn(22);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            21 => {
                // basic_option = "search", names => ActionFn(23);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                15
            }
            22 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(26);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                16
            }
            23 => {
                // config =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            24 => {
                // config = config_line+ => ActionFn(41);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig(__nt), __end));
                17
            }
            25 => {
                // config_line = config_option => ActionFn(13);
                let __sym0 = __pop_Ntconfig__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            26 => {
                // config_line = comment => ActionFn(14);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line(__nt), __end));
                18
            }
            27 => {
                // config_line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            28 => {
                // config_line* = config_line+ => ActionFn(37);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2a(__nt), __end));
                19
            }
            29 => {
                // config_line+ = config_line => ActionFn(38);
                let __sym0 = __pop_Ntconfig__line(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            30 => {
                // config_line+ = config_line+, config_line => ActionFn(39);
                let __sym1 = __pop_Ntconfig__line(__symbols);
                let __sym0 = __pop_Ntconfig__line_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntconfig__line_2b(__nt), __end));
                20
            }
            31 => {
                // config_option = basic_option => ActionFn(15);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            32 => {
                // config_option = advanced_options => ActionFn(16);
                let __sym0 = __pop_Ntadvanced__options(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntconfig__option(__nt), __end));
                21
            }
            33 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(27);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                22
            }
            34 => {
                // ip_addr+ = ip_addr => ActionFn(32);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            35 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(33);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                23
            }
            36 => {
                // ip_addrs = ip_addr+ => ActionFn(24);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                24
            }
            37 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(28);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                25
            }
            38 => {
                // name+ = name => ActionFn(30);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            39 => {
                // name+ = name+, name => ActionFn(31);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                26
            }
            40 => {
                // names = name+ => ActionFn(25);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                27
            }
            41 => {
                // u8 = r#"([0-9]+)"# => ActionFn(29);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntu8(__nt), __end));
                28
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22attempts_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22attempts_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22domain_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22domain_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22nameserver_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22nameserver_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ndots_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ndots_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22options_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22options_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22search_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22search_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22timeout_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22timeout_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_2b_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_2b_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____advanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____advanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____basic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____basic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____config__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____config__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____ip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____ip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____name<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____name(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____names<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____names(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____u8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____u8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AdvancedOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__option_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__option_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntadvanced__options<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<AdvancedOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntadvanced__options(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntbasic__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BasicOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntbasic__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntcomment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntcomment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<ConfigOption>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__line_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Option<ConfigOption>>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__line_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntconfig__option<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConfigOption, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntconfig__option(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IpAddr, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addr_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addr_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntip__addrs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<IpAddr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntip__addrs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Name, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntname_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntname_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntnames<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Name>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntnames(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Ntu8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u8, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Ntu8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__u8::parse_u8;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use std::net::IpAddr;
    use std::time::Duration;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^((?u:[0-9])+)",
                "^((?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}|(?:(?u:[0-9a-f]){0, 4}(?u::)){2, 7}(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}|(?:(?u:[0-9a-f]){0, 4}(?u::)){2, 7}(?u:[0-9a-f]){1, 4})",
                "^((?u:[-\u{0}-\u{8}\u{e}-\u{1f}!-\"\\$-,/-:<-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])(?:(?u:[\u{0}-\u{8}\u{e}-\u{1f}!-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])+(?u:\\.))+|(?u:\\.))",
                "^(?u:[\\#-\\#;-;])(?u:[\u{0}-\t\u{b}-\u{10ffff}])*",
                "^(?u:attempts:)",
                "^(?u:domain)",
                "^(?u:nameserver)",
                "^(?u:ndots:)",
                "^(?u:options)",
                "^(?u:search)",
                "^(?u:timeout:)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:[0-9])+)").unwrap(),
                __regex::Regex::new("^((?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}|(?:(?u:[0-9a-f]){0, 4}(?u::)){2, 7}(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}|(?:(?u:[0-9a-f]){0, 4}(?u::)){2, 7}(?u:[0-9a-f]){1, 4})").unwrap(),
                __regex::Regex::new("^((?u:[-\u{0}-\u{8}\u{e}-\u{1f}!-\"\\$-,/-:<-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])(?:(?u:[\u{0}-\u{8}\u{e}-\u{1f}!-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])+(?u:\\.))+|(?u:\\.))").unwrap(),
                __regex::Regex::new("^(?u:[\\#-\\#;-;])(?u:[\u{0}-\t\u{b}-\u{10ffff}])*").unwrap(),
                __regex::Regex::new("^(?u:attempts:)").unwrap(),
                __regex::Regex::new("^(?u:domain)").unwrap(),
                __regex::Regex::new("^(?u:nameserver)").unwrap(),
                __regex::Regex::new("^(?u:ndots:)").unwrap(),
                __regex::Regex::new("^(?u:options)").unwrap(),
                __regex::Regex::new("^(?u:search)").unwrap(),
                __regex::Regex::new("^(?u:timeout:)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 11 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<ConfigOption>, usize),
) -> Vec<ConfigOption>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Option<ConfigOption>, usize),
) -> Option<ConfigOption>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, ConfigOption, usize),
) -> ConfigOption
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<AdvancedOption>, usize),
) -> Vec<AdvancedOption>
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, AdvancedOption, usize),
) -> AdvancedOption
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, BasicOption, usize),
) -> BasicOption
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<IpAddr>, usize),
) -> Vec<IpAddr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<Name>, usize),
) -> Vec<Name>
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, IpAddr, usize),
) -> IpAddr
{
    (__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Name, usize),
) -> Name
{
    (__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, u8, usize),
) -> u8
{
    (__0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Option<ConfigOption>>, usize),
) -> Vec<ConfigOption>
{
    v.into_iter().filter_map(|c| c).collect()
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, o, _): (usize, ConfigOption, usize),
) -> Option<ConfigOption>
{
    Some(o)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> Option<ConfigOption>
{
    None
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, BasicOption, usize),
) -> ConfigOption
{
    ConfigOption::Basic(__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<AdvancedOption>, usize),
) -> ConfigOption
{
    ConfigOption::Advanced(__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, ::std::vec::Vec<AdvancedOption>, usize),
) -> Vec<AdvancedOption>
{
    v
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u8, usize),
) -> AdvancedOption
{
    AdvancedOption::NumberOfDots(n)
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u8, usize),
) -> AdvancedOption
{
    AdvancedOption::Timeout(Duration::from_secs(n as u64))
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, u8, usize),
) -> AdvancedOption
{
    AdvancedOption::Attempts(n)
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, IpAddr, usize),
) -> BasicOption
{
    BasicOption::Nameserver(i)
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, Name, usize),
) -> BasicOption
{
    BasicOption::Domain(n)
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, Vec<Name>, usize),
) -> BasicOption
{
    BasicOption::Search(n)
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<IpAddr>, usize),
) -> Vec<IpAddr>
{
    __0
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Name>, usize),
) -> Vec<Name>
{
    __0
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> IpAddr
{
    IpAddr::from_str(__0).expect("failed to parse IpAddr")
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Name
{
    Name::from_str(__0).expect("failed to parse Name")
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> u8
{
    u8::from_str(n).expect("failed to parse unsigned 8bit number")
}

#[allow(unused_variables)]
fn __action30<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Name, usize),
) -> ::std::vec::Vec<Name>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Name>, usize),
    (_, e, _): (usize, Name, usize),
) -> ::std::vec::Vec<Name>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action32<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, IpAddr, usize),
) -> ::std::vec::Vec<IpAddr>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action33<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<IpAddr>, usize),
    (_, e, _): (usize, IpAddr, usize),
) -> ::std::vec::Vec<IpAddr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action34<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, AdvancedOption, usize),
) -> ::std::vec::Vec<AdvancedOption>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action35<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<AdvancedOption>, usize),
    (_, e, _): (usize, AdvancedOption, usize),
) -> ::std::vec::Vec<AdvancedOption>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action36<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Option<ConfigOption>>
{
    vec![]
}

#[allow(unused_variables)]
fn __action37<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Option<ConfigOption>>, usize),
) -> ::std::vec::Vec<Option<ConfigOption>>
{
    v
}

#[allow(unused_variables)]
fn __action38<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Option<ConfigOption>, usize),
) -> ::std::vec::Vec<Option<ConfigOption>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action39<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Option<ConfigOption>>, usize),
    (_, e, _): (usize, Option<ConfigOption>, usize),
) -> ::std::vec::Vec<Option<ConfigOption>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action40<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ConfigOption>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action36(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action41<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Option<ConfigOption>>, usize),
) -> Vec<ConfigOption>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        errors,
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, 'err, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
