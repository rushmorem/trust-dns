use std::str::FromStr;
use std::net::IpAddr;
use lalrpop_util::ErrorRecovery;
use trust_dns::rr::Name;
use system_conf::resolv_conf_ast::*;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__basic_option {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22search_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        3, 4, 5, 0, 0, 0,
        // State 1
        -1, -1, -1, -1, -1, -1,
        // State 2
        0, 0, 0, 0, 7, 0,
        // State 3
        0, 0, 0, 11, 0, 0,
        // State 4
        0, 0, 0, 0, 7, 0,
        // State 5
        -8, -8, -8, -8, -8, -8,
        // State 6
        -15, -15, -15, -15, -15, -15,
        // State 7
        -12, -12, -12, -12, -12, -12,
        // State 8
        0, 0, 0, 11, 0, 0,
        // State 9
        -7, -7, -7, -7, -7, -7,
        // State 10
        -11, -11, -11, -11, -11, -11,
        // State 11
        -16, -16, -16, -16, -16, -16,
        // State 12
        0, 0, 0, 0, 7, 0,
        // State 13
        -9, -9, -9, -9, -9, -9,
        // State 14
        -13, -13, -13, -13, -13, -13,
        // State 15
        -17, -17, -17, -17, -17, -17,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -1,
        0,
        0,
        0,
        -8,
        -15,
        -12,
        -14,
        -7,
        -11,
        -16,
        -18,
        -9,
        -13,
        -17,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 14,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""domain""###,
            r###""nameserver""###,
            r###""search""###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 6)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (3, _) if true => 0,
                (4, _) if true => 1,
                (5, _) if true => 2,
                (0, _) if true => 3,
                (1, _) if true => 4,
                (2, _) if true => 5,
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
                let __action = __ACTION[__state * 6 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
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
                // __basic_option = basic_option => ActionFn(1);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            2 => {
                // __comment = comment => ActionFn(0);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                1
            }
            3 => {
                // __ip_addr = ip_addr => ActionFn(4);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                2
            }
            4 => {
                // __ip_addrs = ip_addrs => ActionFn(2);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                3
            }
            5 => {
                // __name = name => ActionFn(5);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                4
            }
            6 => {
                // __names = names => ActionFn(3);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                5
            }
            7 => {
                // basic_option = "nameserver", ip_addrs => ActionFn(7);
                let __sym1 = __pop_Ntip__addrs(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            8 => {
                // basic_option = "domain", name => ActionFn(8);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            9 => {
                // basic_option = "search", names => ActionFn(9);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            10 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                7
            }
            11 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                8
            }
            12 => {
                // ip_addr+ = ip_addr => ActionFn(16);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            13 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(17);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            14 => {
                // ip_addrs = ip_addr+ => ActionFn(10);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                10
            }
            15 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            16 => {
                // name+ = name => ActionFn(14);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            17 => {
                // name+ = name+, name => ActionFn(15);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            18 => {
                // names = name+ => ActionFn(11);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
}
pub use self::__parse__basic_option::parse_basic_option;

mod __parse__comment {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22search_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 3,
        // State 1
        -2, -2, -2, -2, -2, -2,
        // State 2
        -10, -10, -10, -10, -10, -10,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -2,
        -10,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""domain""###,
            r###""nameserver""###,
            r###""search""###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 6)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (3, _) if true => 0,
                (4, _) if true => 1,
                (5, _) if true => 2,
                (0, _) if true => 3,
                (1, _) if true => 4,
                (2, _) if true => 5,
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
                let __action = __ACTION[__state * 6 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
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
                // __basic_option = basic_option => ActionFn(1);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                0
            }
            2 => {
                // __comment = comment => ActionFn(0);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            3 => {
                // __ip_addr = ip_addr => ActionFn(4);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                2
            }
            4 => {
                // __ip_addrs = ip_addrs => ActionFn(2);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                3
            }
            5 => {
                // __name = name => ActionFn(5);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                4
            }
            6 => {
                // __names = names => ActionFn(3);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                5
            }
            7 => {
                // basic_option = "nameserver", ip_addrs => ActionFn(7);
                let __sym1 = __pop_Ntip__addrs(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            8 => {
                // basic_option = "domain", name => ActionFn(8);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            9 => {
                // basic_option = "search", names => ActionFn(9);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            10 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                7
            }
            11 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                8
            }
            12 => {
                // ip_addr+ = ip_addr => ActionFn(16);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            13 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(17);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            14 => {
                // ip_addrs = ip_addr+ => ActionFn(10);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                10
            }
            15 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            16 => {
                // name+ = name => ActionFn(14);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            17 => {
                // name+ = name+, name => ActionFn(15);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            18 => {
                // names = name+ => ActionFn(11);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
}
pub use self::__parse__comment::parse_comment;

mod __parse__ip_addr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22search_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 3, 0, 0,
        // State 1
        -3, -3, -3, -3, -3, -3,
        // State 2
        -11, -11, -11, -11, -11, -11,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -3,
        -11,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""domain""###,
            r###""nameserver""###,
            r###""search""###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 6)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (3, _) if true => 0,
                (4, _) if true => 1,
                (5, _) if true => 2,
                (0, _) if true => 3,
                (1, _) if true => 4,
                (2, _) if true => 5,
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
                let __action = __ACTION[__state * 6 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
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
                // __basic_option = basic_option => ActionFn(1);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                0
            }
            2 => {
                // __comment = comment => ActionFn(0);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                1
            }
            3 => {
                // __ip_addr = ip_addr => ActionFn(4);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            4 => {
                // __ip_addrs = ip_addrs => ActionFn(2);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                3
            }
            5 => {
                // __name = name => ActionFn(5);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                4
            }
            6 => {
                // __names = names => ActionFn(3);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                5
            }
            7 => {
                // basic_option = "nameserver", ip_addrs => ActionFn(7);
                let __sym1 = __pop_Ntip__addrs(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            8 => {
                // basic_option = "domain", name => ActionFn(8);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            9 => {
                // basic_option = "search", names => ActionFn(9);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            10 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                7
            }
            11 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                8
            }
            12 => {
                // ip_addr+ = ip_addr => ActionFn(16);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            13 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(17);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            14 => {
                // ip_addrs = ip_addr+ => ActionFn(10);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                10
            }
            15 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            16 => {
                // name+ = name => ActionFn(14);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            17 => {
                // name+ = name+, name => ActionFn(15);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            18 => {
                // names = name+ => ActionFn(11);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
}
pub use self::__parse__ip_addr::parse_ip_addr;

mod __parse__ip_addrs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22search_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 5, 0, 0,
        // State 1
        -12, -12, -12, -12, -12, -12,
        // State 2
        0, 0, 0, 5, 0, 0,
        // State 3
        -4, -4, -4, -4, -4, -4,
        // State 4
        -11, -11, -11, -11, -11, -11,
        // State 5
        -13, -13, -13, -13, -13, -13,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -12,
        -14,
        -4,
        -11,
        -13,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""domain""###,
            r###""nameserver""###,
            r###""search""###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 6)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (3, _) if true => 0,
                (4, _) if true => 1,
                (5, _) if true => 2,
                (0, _) if true => 3,
                (1, _) if true => 4,
                (2, _) if true => 5,
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
                let __action = __ACTION[__state * 6 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
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
                // __basic_option = basic_option => ActionFn(1);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                0
            }
            2 => {
                // __comment = comment => ActionFn(0);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                1
            }
            3 => {
                // __ip_addr = ip_addr => ActionFn(4);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                2
            }
            4 => {
                // __ip_addrs = ip_addrs => ActionFn(2);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            5 => {
                // __name = name => ActionFn(5);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                4
            }
            6 => {
                // __names = names => ActionFn(3);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                5
            }
            7 => {
                // basic_option = "nameserver", ip_addrs => ActionFn(7);
                let __sym1 = __pop_Ntip__addrs(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            8 => {
                // basic_option = "domain", name => ActionFn(8);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            9 => {
                // basic_option = "search", names => ActionFn(9);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            10 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                7
            }
            11 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                8
            }
            12 => {
                // ip_addr+ = ip_addr => ActionFn(16);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            13 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(17);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            14 => {
                // ip_addrs = ip_addr+ => ActionFn(10);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                10
            }
            15 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            16 => {
                // name+ = name => ActionFn(14);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            17 => {
                // name+ = name+, name => ActionFn(15);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            18 => {
                // names = name+ => ActionFn(11);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
}
pub use self::__parse__ip_addrs::parse_ip_addrs;

mod __parse__name {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22search_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 3, 0,
        // State 1
        -5, -5, -5, -5, -5, -5,
        // State 2
        -15, -15, -15, -15, -15, -15,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -5,
        -15,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""domain""###,
            r###""nameserver""###,
            r###""search""###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 6)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (3, _) if true => 0,
                (4, _) if true => 1,
                (5, _) if true => 2,
                (0, _) if true => 3,
                (1, _) if true => 4,
                (2, _) if true => 5,
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
                let __action = __ACTION[__state * 6 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
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
                // __basic_option = basic_option => ActionFn(1);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                0
            }
            2 => {
                // __comment = comment => ActionFn(0);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                1
            }
            3 => {
                // __ip_addr = ip_addr => ActionFn(4);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                2
            }
            4 => {
                // __ip_addrs = ip_addrs => ActionFn(2);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                3
            }
            5 => {
                // __name = name => ActionFn(5);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            6 => {
                // __names = names => ActionFn(3);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____names(__nt), __end));
                5
            }
            7 => {
                // basic_option = "nameserver", ip_addrs => ActionFn(7);
                let __sym1 = __pop_Ntip__addrs(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            8 => {
                // basic_option = "domain", name => ActionFn(8);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            9 => {
                // basic_option = "search", names => ActionFn(9);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            10 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                7
            }
            11 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                8
            }
            12 => {
                // ip_addr+ = ip_addr => ActionFn(16);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            13 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(17);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            14 => {
                // ip_addrs = ip_addr+ => ActionFn(10);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                10
            }
            15 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            16 => {
                // name+ = name => ActionFn(14);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            17 => {
                // name+ = name+, name => ActionFn(15);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            18 => {
                // names = name+ => ActionFn(11);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
}
pub use self::__parse__name::parse_name;

mod __parse__names {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use std::net::IpAddr;
    use lalrpop_util::ErrorRecovery;
    use trust_dns::rr::Name;
    use system_conf::resolv_conf_ast::*;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22domain_22(&'input str),
        Term_22nameserver_22(&'input str),
        Term_22search_22(&'input str),
        Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(&'input str),
        Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(&'input str),
        Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(&'input str),
        Nt____basic__option(BasicOption),
        Nt____comment(()),
        Nt____ip__addr(IpAddr),
        Nt____ip__addrs(Vec<IpAddr>),
        Nt____name(Name),
        Nt____names(Vec<Name>),
        Ntbasic__option(BasicOption),
        Ntcomment(()),
        Ntip__addr(IpAddr),
        Ntip__addr_2b(::std::vec::Vec<IpAddr>),
        Ntip__addrs(Vec<IpAddr>),
        Ntname(Name),
        Ntname_2b(::std::vec::Vec<Name>),
        Ntnames(Vec<Name>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 5, 0,
        // State 1
        -16, -16, -16, -16, -16, -16,
        // State 2
        0, 0, 0, 0, 5, 0,
        // State 3
        -6, -6, -6, -6, -6, -6,
        // State 4
        -15, -15, -15, -15, -15, -15,
        // State 5
        -17, -17, -17, -17, -17, -17,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -16,
        -18,
        -6,
        -15,
        -17,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""domain""###,
            r###""nameserver""###,
            r###""search""###,
            r###"r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"#"###,
            r###"r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"#"###,
            r###"r#"[#;][^\\n]*"#"###,
        ];
        __ACTION[(__state * 6)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                (3, _) if true => 0,
                (4, _) if true => 1,
                (5, _) if true => 2,
                (0, _) if true => 3,
                (1, _) if true => 4,
                (2, _) if true => 5,
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
                let __action = __ACTION[__state * 6 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22domain_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22nameserver_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22search_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23((__tok0)),
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
                // __basic_option = basic_option => ActionFn(1);
                let __sym0 = __pop_Ntbasic__option(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____basic__option(__nt), __end));
                0
            }
            2 => {
                // __comment = comment => ActionFn(0);
                let __sym0 = __pop_Ntcomment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____comment(__nt), __end));
                1
            }
            3 => {
                // __ip_addr = ip_addr => ActionFn(4);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addr(__nt), __end));
                2
            }
            4 => {
                // __ip_addrs = ip_addrs => ActionFn(2);
                let __sym0 = __pop_Ntip__addrs(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____ip__addrs(__nt), __end));
                3
            }
            5 => {
                // __name = name => ActionFn(5);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____name(__nt), __end));
                4
            }
            6 => {
                // __names = names => ActionFn(3);
                let __sym0 = __pop_Ntnames(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            7 => {
                // basic_option = "nameserver", ip_addrs => ActionFn(7);
                let __sym1 = __pop_Ntip__addrs(__symbols);
                let __sym0 = __pop_Term_22nameserver_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            8 => {
                // basic_option = "domain", name => ActionFn(8);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Term_22domain_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            9 => {
                // basic_option = "search", names => ActionFn(9);
                let __sym1 = __pop_Ntnames(__symbols);
                let __sym0 = __pop_Term_22search_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntbasic__option(__nt), __end));
                6
            }
            10 => {
                // comment = r#"[#;][^\\n]*"# => ActionFn(6);
                let __sym0 = __pop_Termr_23_22_5b_23_3b_5d_5b_5e_5c_5cn_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntcomment(__nt), __end));
                7
            }
            11 => {
                // ip_addr = r#"([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|(?:[0-9a-f]{0,4}:){2,7}[0-9a-f]{1,4})"# => ActionFn(12);
                let __sym0 = __pop_Termr_23_22_28_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_5c_5c_2e_5b0_2d9_5d_7b1_2c3_7d_7c_28_3f_3a_5b0_2d9a_2df_5d_7b0_2c4_7d_3a_29_7b2_2c7_7d_5b0_2d9a_2df_5d_7b1_2c4_7d_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr(__nt), __end));
                8
            }
            12 => {
                // ip_addr+ = ip_addr => ActionFn(16);
                let __sym0 = __pop_Ntip__addr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            13 => {
                // ip_addr+ = ip_addr+, ip_addr => ActionFn(17);
                let __sym1 = __pop_Ntip__addr(__symbols);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntip__addr_2b(__nt), __end));
                9
            }
            14 => {
                // ip_addrs = ip_addr+ => ActionFn(10);
                let __sym0 = __pop_Ntip__addr_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntip__addrs(__nt), __end));
                10
            }
            15 => {
                // name = r#"([^#;\\.\\s](?:[\\w\\S]+\\.)+|\\.)"# => ActionFn(13);
                let __sym0 = __pop_Termr_23_22_28_5b_5e_23_3b_5c_5c_2e_5c_5cs_5d_28_3f_3a_5b_5c_5cw_5c_5cS_5d_2b_5c_5c_2e_29_2b_7c_5c_5c_2e_29_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname(__nt), __end));
                11
            }
            16 => {
                // name+ = name => ActionFn(14);
                let __sym0 = __pop_Ntname(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            17 => {
                // name+ = name+, name => ActionFn(15);
                let __sym1 = __pop_Ntname(__symbols);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Ntname_2b(__nt), __end));
                12
            }
            18 => {
                // names = name+ => ActionFn(11);
                let __sym0 = __pop_Ntname_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Ntnames(__nt), __end));
                13
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
}
pub use self::__parse__names::parse_names;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use std::net::IpAddr;
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
                "^((?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}|(?:(?u:[0-9a-f]){0, 4}(?u::)){2, 7}(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}|(?:(?u:[0-9a-f]){0, 4}(?u::)){2, 7}(?u:[0-9a-f]){1, 4})",
                "^((?u:[-\u{0}-\u{8}\u{e}-\u{1f}!-\"\\$-,/-:<-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])(?:(?u:[\u{0}-\u{8}\u{e}-\u{1f}!-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])+(?u:\\.))+|(?u:\\.))",
                "^(?u:[\\#-\\#;-;])(?u:[\u{0}-\t\u{b}-\u{10ffff}])*",
                "^(?u:domain)",
                "^(?u:nameserver)",
                "^(?u:search)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}|(?:(?u:[0-9a-f]){0, 4}(?u::)){2, 7}(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}(?u:\\.)(?u:[0-9]){1, 3}|(?:(?u:[0-9a-f]){0, 4}(?u::)){2, 7}(?u:[0-9a-f]){1, 4})").unwrap(),
                __regex::Regex::new("^((?u:[-\u{0}-\u{8}\u{e}-\u{1f}!-\"\\$-,/-:<-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])(?:(?u:[\u{0}-\u{8}\u{e}-\u{1f}!-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}])+(?u:\\.))+|(?u:\\.))").unwrap(),
                __regex::Regex::new("^(?u:[\\#-\\#;-;])(?u:[\u{0}-\t\u{b}-\u{10ffff}])*").unwrap(),
                __regex::Regex::new("^(?u:domain)").unwrap(),
                __regex::Regex::new("^(?u:nameserver)").unwrap(),
                __regex::Regex::new("^(?u:search)").unwrap(),
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
                    for __i in 0 .. 6 {
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
    (_, __0, _): (usize, (), usize),
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action1<
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
fn __action2<
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
fn __action3<
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
fn __action4<
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
fn __action5<
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
fn __action6<
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
fn __action7<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, Vec<IpAddr>, usize),
) -> BasicOption
{
    BasicOption::Nameserver(i)
}

#[allow(unused_variables)]
fn __action8<
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
fn __action9<
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
fn __action10<
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
fn __action11<
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
fn __action12<
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
fn __action13<
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
fn __action14<
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
fn __action15<
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
fn __action16<
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
fn __action17<
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
