pub use events_lib::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod events_lib {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccrueInterest"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AccrueInterest"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("prevBorrowRate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("interest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeShares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Borrow"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Borrow"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CreateMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CreateMarket"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnableIrm"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnableIrm"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("irm"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EnableLltv"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EnableLltv"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("lltv"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FlashLoan"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FlashLoan"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncrementNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("IncrementNonce"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("usedNonce"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Liquidate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Liquidate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("borrower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("repaidAssets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("repaidShares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("seizedAssets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("badDebtAssets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("badDebtShares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Repay"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Repay"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetAuthorization"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetAuthorization"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("authorized"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newIsAuthorized"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetFeeRecipient"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetFeeRecipient"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("newFeeRecipient"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SetOwner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Supply"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Supply"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SupplyCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SupplyCollateral"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("WithdrawCollateral"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("caller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EVENTSLIB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`U`2`\x0B\x82\x82\x829\x80Q_\x1A`s\x14`&WcNH{q`\xE0\x1B_R_`\x04R`$_\xFD[0_R`s\x81S\x82\x81\xF3\xFEs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 \n\xED`k\xB4'\x93Z\x14\x17\x9E\x10\xF7\xC7hSE.\x8D\x9F\x1B\xDF\x11\xDDDX\x90\x88b\xB8\x8CwdsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static EVENTSLIB_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14`\x80`@R_\x80\xFD\xFE\xA2dipfsX\"\x12 \n\xED`k\xB4'\x93Z\x14\x17\x9E\x10\xF7\xC7hSE.\x8D\x9F\x1B\xDF\x11\xDDDX\x90\x88b\xB8\x8CwdsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static EVENTSLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EventsLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EventsLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EventsLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EventsLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EventsLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EventsLib)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EventsLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), EVENTSLIB_ABI.clone(), client))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                EVENTSLIB_ABI.clone(),
                EVENTSLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `AccrueInterest` event
        pub fn accrue_interest_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccrueInterestFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Borrow` event
        pub fn borrow_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BorrowFilter> {
            self.0.event()
        }
        ///Gets the contract's `CreateMarket` event
        pub fn create_market_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CreateMarketFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EnableIrm` event
        pub fn enable_irm_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EnableIrmFilter> {
            self.0.event()
        }
        ///Gets the contract's `EnableLltv` event
        pub fn enable_lltv_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EnableLltvFilter> {
            self.0.event()
        }
        ///Gets the contract's `FlashLoan` event
        pub fn flash_loan_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FlashLoanFilter> {
            self.0.event()
        }
        ///Gets the contract's `IncrementNonce` event
        pub fn increment_nonce_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IncrementNonceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Liquidate` event
        pub fn liquidate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LiquidateFilter> {
            self.0.event()
        }
        ///Gets the contract's `Repay` event
        pub fn repay_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RepayFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetAuthorization` event
        pub fn set_authorization_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetAuthorizationFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetFee` event
        pub fn set_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetFeeFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetFeeRecipient` event
        pub fn set_fee_recipient_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetFeeRecipientFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetOwner` event
        pub fn set_owner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetOwnerFilter> {
            self.0.event()
        }
        ///Gets the contract's `Supply` event
        pub fn supply_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SupplyFilter> {
            self.0.event()
        }
        ///Gets the contract's `SupplyCollateral` event
        pub fn supply_collateral_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SupplyCollateralFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawFilter> {
            self.0.event()
        }
        ///Gets the contract's `WithdrawCollateral` event
        pub fn withdraw_collateral_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawCollateralFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EventsLibEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for EventsLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "AccrueInterest", abi = "AccrueInterest(bytes32,uint256,uint256,uint256)")]
    pub struct AccrueInterestFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub prev_borrow_rate: ::ethers::core::types::U256,
        pub interest: ::ethers::core::types::U256,
        pub fee_shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Borrow", abi = "Borrow(bytes32,address,address,address,uint256,uint256)")]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "CreateMarket",
        abi = "CreateMarket(bytes32,(address,address,address,address,uint256))"
    )]
    pub struct CreateMarketFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub market_params: MarketParams,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "EnableIrm", abi = "EnableIrm(address)")]
    pub struct EnableIrmFilter {
        #[ethevent(indexed)]
        pub irm: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "EnableLltv", abi = "EnableLltv(uint256)")]
    pub struct EnableLltvFilter {
        pub lltv: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "FlashLoan", abi = "FlashLoan(address,address,uint256)")]
    pub struct FlashLoanFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "IncrementNonce", abi = "IncrementNonce(address,address,uint256)")]
    pub struct IncrementNonceFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
        pub used_nonce: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Liquidate",
        abi = "Liquidate(bytes32,address,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct LiquidateFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub borrower: ::ethers::core::types::Address,
        pub repaid_assets: ::ethers::core::types::U256,
        pub repaid_shares: ::ethers::core::types::U256,
        pub seized_assets: ::ethers::core::types::U256,
        pub bad_debt_assets: ::ethers::core::types::U256,
        pub bad_debt_shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Repay", abi = "Repay(bytes32,address,address,uint256,uint256)")]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetAuthorization", abi = "SetAuthorization(address,address,address,bool)")]
    pub struct SetAuthorizationFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub authorizer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub authorized: ::ethers::core::types::Address,
        pub new_is_authorized: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetFee", abi = "SetFee(bytes32,uint256)")]
    pub struct SetFeeFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub new_fee: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetFeeRecipient", abi = "SetFeeRecipient(address)")]
    pub struct SetFeeRecipientFilter {
        #[ethevent(indexed)]
        pub new_fee_recipient: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetOwner", abi = "SetOwner(address)")]
    pub struct SetOwnerFilter {
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Supply", abi = "Supply(bytes32,address,address,uint256,uint256)")]
    pub struct SupplyFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "SupplyCollateral",
        abi = "SupplyCollateral(bytes32,address,address,uint256)"
    )]
    pub struct SupplyCollateralFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Withdraw",
        abi = "Withdraw(bytes32,address,address,address,uint256,uint256)"
    )]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "WithdrawCollateral",
        abi = "WithdrawCollateral(bytes32,address,address,address,uint256)"
    )]
    pub struct WithdrawCollateralFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum EventsLibEvents {
        AccrueInterestFilter(AccrueInterestFilter),
        BorrowFilter(BorrowFilter),
        CreateMarketFilter(CreateMarketFilter),
        EnableIrmFilter(EnableIrmFilter),
        EnableLltvFilter(EnableLltvFilter),
        FlashLoanFilter(FlashLoanFilter),
        IncrementNonceFilter(IncrementNonceFilter),
        LiquidateFilter(LiquidateFilter),
        RepayFilter(RepayFilter),
        SetAuthorizationFilter(SetAuthorizationFilter),
        SetFeeFilter(SetFeeFilter),
        SetFeeRecipientFilter(SetFeeRecipientFilter),
        SetOwnerFilter(SetOwnerFilter),
        SupplyFilter(SupplyFilter),
        SupplyCollateralFilter(SupplyCollateralFilter),
        WithdrawFilter(WithdrawFilter),
        WithdrawCollateralFilter(WithdrawCollateralFilter),
    }
    impl ::ethers::contract::EthLogDecode for EventsLibEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccrueInterestFilter::decode_log(log) {
                return Ok(EventsLibEvents::AccrueInterestFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(EventsLibEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = CreateMarketFilter::decode_log(log) {
                return Ok(EventsLibEvents::CreateMarketFilter(decoded));
            }
            if let Ok(decoded) = EnableIrmFilter::decode_log(log) {
                return Ok(EventsLibEvents::EnableIrmFilter(decoded));
            }
            if let Ok(decoded) = EnableLltvFilter::decode_log(log) {
                return Ok(EventsLibEvents::EnableLltvFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::decode_log(log) {
                return Ok(EventsLibEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = IncrementNonceFilter::decode_log(log) {
                return Ok(EventsLibEvents::IncrementNonceFilter(decoded));
            }
            if let Ok(decoded) = LiquidateFilter::decode_log(log) {
                return Ok(EventsLibEvents::LiquidateFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(EventsLibEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = SetAuthorizationFilter::decode_log(log) {
                return Ok(EventsLibEvents::SetAuthorizationFilter(decoded));
            }
            if let Ok(decoded) = SetFeeFilter::decode_log(log) {
                return Ok(EventsLibEvents::SetFeeFilter(decoded));
            }
            if let Ok(decoded) = SetFeeRecipientFilter::decode_log(log) {
                return Ok(EventsLibEvents::SetFeeRecipientFilter(decoded));
            }
            if let Ok(decoded) = SetOwnerFilter::decode_log(log) {
                return Ok(EventsLibEvents::SetOwnerFilter(decoded));
            }
            if let Ok(decoded) = SupplyFilter::decode_log(log) {
                return Ok(EventsLibEvents::SupplyFilter(decoded));
            }
            if let Ok(decoded) = SupplyCollateralFilter::decode_log(log) {
                return Ok(EventsLibEvents::SupplyCollateralFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(EventsLibEvents::WithdrawFilter(decoded));
            }
            if let Ok(decoded) = WithdrawCollateralFilter::decode_log(log) {
                return Ok(EventsLibEvents::WithdrawCollateralFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EventsLibEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccrueInterestFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateMarketFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableIrmFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableLltvFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoanFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementNonceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthorizationFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeRecipientFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOwnerFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyCollateralFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawCollateralFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccrueInterestFilter> for EventsLibEvents {
        fn from(value: AccrueInterestFilter) -> Self {
            Self::AccrueInterestFilter(value)
        }
    }
    impl ::core::convert::From<BorrowFilter> for EventsLibEvents {
        fn from(value: BorrowFilter) -> Self {
            Self::BorrowFilter(value)
        }
    }
    impl ::core::convert::From<CreateMarketFilter> for EventsLibEvents {
        fn from(value: CreateMarketFilter) -> Self {
            Self::CreateMarketFilter(value)
        }
    }
    impl ::core::convert::From<EnableIrmFilter> for EventsLibEvents {
        fn from(value: EnableIrmFilter) -> Self {
            Self::EnableIrmFilter(value)
        }
    }
    impl ::core::convert::From<EnableLltvFilter> for EventsLibEvents {
        fn from(value: EnableLltvFilter) -> Self {
            Self::EnableLltvFilter(value)
        }
    }
    impl ::core::convert::From<FlashLoanFilter> for EventsLibEvents {
        fn from(value: FlashLoanFilter) -> Self {
            Self::FlashLoanFilter(value)
        }
    }
    impl ::core::convert::From<IncrementNonceFilter> for EventsLibEvents {
        fn from(value: IncrementNonceFilter) -> Self {
            Self::IncrementNonceFilter(value)
        }
    }
    impl ::core::convert::From<LiquidateFilter> for EventsLibEvents {
        fn from(value: LiquidateFilter) -> Self {
            Self::LiquidateFilter(value)
        }
    }
    impl ::core::convert::From<RepayFilter> for EventsLibEvents {
        fn from(value: RepayFilter) -> Self {
            Self::RepayFilter(value)
        }
    }
    impl ::core::convert::From<SetAuthorizationFilter> for EventsLibEvents {
        fn from(value: SetAuthorizationFilter) -> Self {
            Self::SetAuthorizationFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeFilter> for EventsLibEvents {
        fn from(value: SetFeeFilter) -> Self {
            Self::SetFeeFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeRecipientFilter> for EventsLibEvents {
        fn from(value: SetFeeRecipientFilter) -> Self {
            Self::SetFeeRecipientFilter(value)
        }
    }
    impl ::core::convert::From<SetOwnerFilter> for EventsLibEvents {
        fn from(value: SetOwnerFilter) -> Self {
            Self::SetOwnerFilter(value)
        }
    }
    impl ::core::convert::From<SupplyFilter> for EventsLibEvents {
        fn from(value: SupplyFilter) -> Self {
            Self::SupplyFilter(value)
        }
    }
    impl ::core::convert::From<SupplyCollateralFilter> for EventsLibEvents {
        fn from(value: SupplyCollateralFilter) -> Self {
            Self::SupplyCollateralFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for EventsLibEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawCollateralFilter> for EventsLibEvents {
        fn from(value: WithdrawCollateralFilter) -> Self {
            Self::WithdrawCollateralFilter(value)
        }
    }
}
