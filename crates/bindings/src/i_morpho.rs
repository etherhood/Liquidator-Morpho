pub use i_morpho::*;
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
pub mod i_morpho {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("accrueInterest"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("accrueInterest"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marketParams"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("borrow"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("borrow"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assetsBorrowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sharesBorrowed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createMarket"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createMarket"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("marketParams"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableIrm"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableIrm"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("irm"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableLltv"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableLltv"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("lltv"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("extSloads"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("extSloads"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("slots"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feeRecipient"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feeRecipient"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("flashLoan"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("flashLoan"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("idToMarketParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("idToMarketParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("Id"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isAuthorized"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isAuthorized"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("authorizer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("authorized"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isIrmEnabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isIrmEnabled"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("irm"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isLltvEnabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isLltvEnabled"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("lltv"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("borrower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("seizedAssets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("repaidShares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("market"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("market"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("Id"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("m"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Market"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nonce"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("authorizer"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("position"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("position"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("Id"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("p"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Position"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("repay"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("repay"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assetsRepaid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sharesRepaid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAuthorization"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAuthorization"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("authorized"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newIsAuthorized"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAuthorizationWithSig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAuthorizationWithSig",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("authorization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Authorization"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Signature"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFeeRecipient"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFeeRecipient"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newFeeRecipient"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setOwner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supply"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supply"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assetsSupplied"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sharesSupplied"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supplyCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supplyCollateral"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdraw"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("shares"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assetsWithdrawn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sharesWithdrawn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawCollateral"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("marketParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct MarketParams"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("assets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("onBehalf"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("receiver"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
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
    pub static IMORPHO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IMorpho<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMorpho<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMorpho<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMorpho<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMorpho<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IMorpho)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMorpho<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), IMORPHO_ABI.clone(), client))
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accrueInterest` (0x151c1ade) function
        pub fn accrue_interest(
            &self,
            market_params: MarketParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 28, 26, 222], (market_params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `borrow` (0x50d8cd4b) function
        pub fn borrow(
            &self,
            market_params: MarketParams,
            assets: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
            on_behalf: ::ethers::core::types::Address,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [80, 216, 205, 75],
                    (market_params, assets, shares, on_behalf, receiver),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createMarket` (0x8c1358a2) function
        pub fn create_market(
            &self,
            market_params: MarketParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 19, 88, 162], (market_params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableIrm` (0x5a64f51e) function
        pub fn enable_irm(
            &self,
            irm: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 100, 245, 30], irm)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableLltv` (0x4d98a93b) function
        pub fn enable_lltv(
            &self,
            lltv: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 152, 169, 59], lltv)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `extSloads` (0x7784c685) function
        pub fn ext_sloads(
            &self,
            slots: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([119, 132, 198, 133], slots)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeRecipient` (0x46904840) function
        pub fn fee_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([70, 144, 72, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashLoan` (0xe0232b42) function
        pub fn flash_loan(
            &self,
            token: ::ethers::core::types::Address,
            assets: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 35, 43, 66], (token, assets, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `idToMarketParams` (0x2c3c9157) function
        pub fn id_to_market_params(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, MarketParams> {
            self.0
                .method_hash([44, 60, 145, 87], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAuthorized` (0x65e4ad9e) function
        pub fn is_authorized(
            &self,
            authorizer: ::ethers::core::types::Address,
            authorized: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([101, 228, 173, 158], (authorizer, authorized))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isIrmEnabled` (0xf2b863ce) function
        pub fn is_irm_enabled(
            &self,
            irm: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([242, 184, 99, 206], irm)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isLltvEnabled` (0xb485f3b8) function
        pub fn is_lltv_enabled(
            &self,
            lltv: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([180, 133, 243, 184], lltv)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0xd8eabcb8) function
        pub fn liquidate(
            &self,
            market_params: MarketParams,
            borrower: ::ethers::core::types::Address,
            seized_assets: ::ethers::core::types::U256,
            repaid_shares: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [216, 234, 188, 184],
                    (market_params, borrower, seized_assets, repaid_shares, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `market` (0x5c60e39a) function
        pub fn market(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, Market> {
            self.0
                .method_hash([92, 96, 227, 154], id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0x70ae92d2) function
        pub fn nonce(
            &self,
            authorizer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 174, 146, 210], authorizer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `position` (0x93c52062) function
        pub fn position(
            &self,
            id: [u8; 32],
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Position> {
            self.0
                .method_hash([147, 197, 32, 98], (id, user))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repay` (0x20b76e81) function
        pub fn repay(
            &self,
            market_params: MarketParams,
            assets: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
            on_behalf: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([32, 183, 110, 129], (market_params, assets, shares, on_behalf, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAuthorization` (0xeecea000) function
        pub fn set_authorization(
            &self,
            authorized: ::ethers::core::types::Address,
            new_is_authorized: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 206, 160, 0], (authorized, new_is_authorized))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAuthorizationWithSig` (0x8069218f) function
        pub fn set_authorization_with_sig(
            &self,
            authorization: Authorization,
            signature: Signature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 105, 33, 143], (authorization, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFee` (0x2b4f013c) function
        pub fn set_fee(
            &self,
            market_params: MarketParams,
            new_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 79, 1, 60], (market_params, new_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeRecipient` (0xe74b981b) function
        pub fn set_fee_recipient(
            &self,
            new_fee_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 75, 152, 27], new_fee_recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOwner` (0x13af4035) function
        pub fn set_owner(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supply` (0xa99aad89) function
        pub fn supply(
            &self,
            market_params: MarketParams,
            assets: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
            on_behalf: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([169, 154, 173, 137], (market_params, assets, shares, on_behalf, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supplyCollateral` (0x238d6579) function
        pub fn supply_collateral(
            &self,
            market_params: MarketParams,
            assets: ::ethers::core::types::U256,
            on_behalf: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 141, 101, 121], (market_params, assets, on_behalf, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x5c2bea49) function
        pub fn withdraw(
            &self,
            market_params: MarketParams,
            assets: ::ethers::core::types::U256,
            shares: ::ethers::core::types::U256,
            on_behalf: ::ethers::core::types::Address,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [92, 43, 234, 73],
                    (market_params, assets, shares, on_behalf, receiver),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawCollateral` (0x8720316d) function
        pub fn withdraw_collateral(
            &self,
            market_params: MarketParams,
            assets: ::ethers::core::types::U256,
            on_behalf: ::ethers::core::types::Address,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 32, 49, 109], (market_params, assets, on_behalf, receiver))
                .expect("method not found (this should never happen)")
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IMorphoEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IMorpho<M> {
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
    pub enum IMorphoEvents {
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
    impl ::ethers::contract::EthLogDecode for IMorphoEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccrueInterestFilter::decode_log(log) {
                return Ok(IMorphoEvents::AccrueInterestFilter(decoded));
            }
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(IMorphoEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = CreateMarketFilter::decode_log(log) {
                return Ok(IMorphoEvents::CreateMarketFilter(decoded));
            }
            if let Ok(decoded) = EnableIrmFilter::decode_log(log) {
                return Ok(IMorphoEvents::EnableIrmFilter(decoded));
            }
            if let Ok(decoded) = EnableLltvFilter::decode_log(log) {
                return Ok(IMorphoEvents::EnableLltvFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::decode_log(log) {
                return Ok(IMorphoEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = IncrementNonceFilter::decode_log(log) {
                return Ok(IMorphoEvents::IncrementNonceFilter(decoded));
            }
            if let Ok(decoded) = LiquidateFilter::decode_log(log) {
                return Ok(IMorphoEvents::LiquidateFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(IMorphoEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = SetAuthorizationFilter::decode_log(log) {
                return Ok(IMorphoEvents::SetAuthorizationFilter(decoded));
            }
            if let Ok(decoded) = SetFeeFilter::decode_log(log) {
                return Ok(IMorphoEvents::SetFeeFilter(decoded));
            }
            if let Ok(decoded) = SetFeeRecipientFilter::decode_log(log) {
                return Ok(IMorphoEvents::SetFeeRecipientFilter(decoded));
            }
            if let Ok(decoded) = SetOwnerFilter::decode_log(log) {
                return Ok(IMorphoEvents::SetOwnerFilter(decoded));
            }
            if let Ok(decoded) = SupplyFilter::decode_log(log) {
                return Ok(IMorphoEvents::SupplyFilter(decoded));
            }
            if let Ok(decoded) = SupplyCollateralFilter::decode_log(log) {
                return Ok(IMorphoEvents::SupplyCollateralFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(IMorphoEvents::WithdrawFilter(decoded));
            }
            if let Ok(decoded) = WithdrawCollateralFilter::decode_log(log) {
                return Ok(IMorphoEvents::WithdrawCollateralFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IMorphoEvents {
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
    impl ::core::convert::From<AccrueInterestFilter> for IMorphoEvents {
        fn from(value: AccrueInterestFilter) -> Self {
            Self::AccrueInterestFilter(value)
        }
    }
    impl ::core::convert::From<BorrowFilter> for IMorphoEvents {
        fn from(value: BorrowFilter) -> Self {
            Self::BorrowFilter(value)
        }
    }
    impl ::core::convert::From<CreateMarketFilter> for IMorphoEvents {
        fn from(value: CreateMarketFilter) -> Self {
            Self::CreateMarketFilter(value)
        }
    }
    impl ::core::convert::From<EnableIrmFilter> for IMorphoEvents {
        fn from(value: EnableIrmFilter) -> Self {
            Self::EnableIrmFilter(value)
        }
    }
    impl ::core::convert::From<EnableLltvFilter> for IMorphoEvents {
        fn from(value: EnableLltvFilter) -> Self {
            Self::EnableLltvFilter(value)
        }
    }
    impl ::core::convert::From<FlashLoanFilter> for IMorphoEvents {
        fn from(value: FlashLoanFilter) -> Self {
            Self::FlashLoanFilter(value)
        }
    }
    impl ::core::convert::From<IncrementNonceFilter> for IMorphoEvents {
        fn from(value: IncrementNonceFilter) -> Self {
            Self::IncrementNonceFilter(value)
        }
    }
    impl ::core::convert::From<LiquidateFilter> for IMorphoEvents {
        fn from(value: LiquidateFilter) -> Self {
            Self::LiquidateFilter(value)
        }
    }
    impl ::core::convert::From<RepayFilter> for IMorphoEvents {
        fn from(value: RepayFilter) -> Self {
            Self::RepayFilter(value)
        }
    }
    impl ::core::convert::From<SetAuthorizationFilter> for IMorphoEvents {
        fn from(value: SetAuthorizationFilter) -> Self {
            Self::SetAuthorizationFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeFilter> for IMorphoEvents {
        fn from(value: SetFeeFilter) -> Self {
            Self::SetFeeFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeRecipientFilter> for IMorphoEvents {
        fn from(value: SetFeeRecipientFilter) -> Self {
            Self::SetFeeRecipientFilter(value)
        }
    }
    impl ::core::convert::From<SetOwnerFilter> for IMorphoEvents {
        fn from(value: SetOwnerFilter) -> Self {
            Self::SetOwnerFilter(value)
        }
    }
    impl ::core::convert::From<SupplyFilter> for IMorphoEvents {
        fn from(value: SupplyFilter) -> Self {
            Self::SupplyFilter(value)
        }
    }
    impl ::core::convert::From<SupplyCollateralFilter> for IMorphoEvents {
        fn from(value: SupplyCollateralFilter) -> Self {
            Self::SupplyCollateralFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for IMorphoEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawCollateralFilter> for IMorphoEvents {
        fn from(value: WithdrawCollateralFilter) -> Self {
            Self::WithdrawCollateralFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `accrueInterest` function with signature `accrueInterest((address,address,address,address,uint256))` and selector `0x151c1ade`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "accrueInterest",
        abi = "accrueInterest((address,address,address,address,uint256))"
    )]
    pub struct AccrueInterestCall {
        pub market_params: MarketParams,
    }
    ///Container type for all input parameters for the `borrow` function with signature `borrow((address,address,address,address,uint256),uint256,uint256,address,address)` and selector `0x50d8cd4b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "borrow",
        abi = "borrow((address,address,address,address,uint256),uint256,uint256,address,address)"
    )]
    pub struct BorrowCall {
        pub market_params: MarketParams,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
        pub on_behalf: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createMarket` function with signature `createMarket((address,address,address,address,uint256))` and selector `0x8c1358a2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "createMarket",
        abi = "createMarket((address,address,address,address,uint256))"
    )]
    pub struct CreateMarketCall {
        pub market_params: MarketParams,
    }
    ///Container type for all input parameters for the `enableIrm` function with signature `enableIrm(address)` and selector `0x5a64f51e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "enableIrm", abi = "enableIrm(address)")]
    pub struct EnableIrmCall {
        pub irm: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `enableLltv` function with signature `enableLltv(uint256)` and selector `0x4d98a93b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "enableLltv", abi = "enableLltv(uint256)")]
    pub struct EnableLltvCall {
        pub lltv: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `extSloads` function with signature `extSloads(bytes32[])` and selector `0x7784c685`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "extSloads", abi = "extSloads(bytes32[])")]
    pub struct ExtSloadsCall {
        pub slots: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feeRecipient", abi = "feeRecipient()")]
    pub struct FeeRecipientCall;
    ///Container type for all input parameters for the `flashLoan` function with signature `flashLoan(address,uint256,bytes)` and selector `0xe0232b42`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "flashLoan", abi = "flashLoan(address,uint256,bytes)")]
    pub struct FlashLoanCall {
        pub token: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `idToMarketParams` function with signature `idToMarketParams(bytes32)` and selector `0x2c3c9157`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "idToMarketParams", abi = "idToMarketParams(bytes32)")]
    pub struct IdToMarketParamsCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `isAuthorized` function with signature `isAuthorized(address,address)` and selector `0x65e4ad9e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isAuthorized", abi = "isAuthorized(address,address)")]
    pub struct IsAuthorizedCall {
        pub authorizer: ::ethers::core::types::Address,
        pub authorized: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isIrmEnabled` function with signature `isIrmEnabled(address)` and selector `0xf2b863ce`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isIrmEnabled", abi = "isIrmEnabled(address)")]
    pub struct IsIrmEnabledCall {
        pub irm: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isLltvEnabled` function with signature `isLltvEnabled(uint256)` and selector `0xb485f3b8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isLltvEnabled", abi = "isLltvEnabled(uint256)")]
    pub struct IsLltvEnabledCall {
        pub lltv: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate((address,address,address,address,uint256),address,uint256,uint256,bytes)` and selector `0xd8eabcb8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "liquidate",
        abi = "liquidate((address,address,address,address,uint256),address,uint256,uint256,bytes)"
    )]
    pub struct LiquidateCall {
        pub market_params: MarketParams,
        pub borrower: ::ethers::core::types::Address,
        pub seized_assets: ::ethers::core::types::U256,
        pub repaid_shares: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `market` function with signature `market(bytes32)` and selector `0x5c60e39a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "market", abi = "market(bytes32)")]
    pub struct MarketCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce(address)` and selector `0x70ae92d2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nonce", abi = "nonce(address)")]
    pub struct NonceCall {
        pub authorizer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `position` function with signature `position(bytes32,address)` and selector `0x93c52062`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "position", abi = "position(bytes32,address)")]
    pub struct PositionCall {
        pub id: [u8; 32],
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `repay` function with signature `repay((address,address,address,address,uint256),uint256,uint256,address,bytes)` and selector `0x20b76e81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "repay",
        abi = "repay((address,address,address,address,uint256),uint256,uint256,address,bytes)"
    )]
    pub struct RepayCall {
        pub market_params: MarketParams,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
        pub on_behalf: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setAuthorization` function with signature `setAuthorization(address,bool)` and selector `0xeecea000`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setAuthorization", abi = "setAuthorization(address,bool)")]
    pub struct SetAuthorizationCall {
        pub authorized: ::ethers::core::types::Address,
        pub new_is_authorized: bool,
    }
    ///Container type for all input parameters for the `setAuthorizationWithSig` function with signature `setAuthorizationWithSig((address,address,bool,uint256,uint256),(uint8,bytes32,bytes32))` and selector `0x8069218f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setAuthorizationWithSig",
        abi = "setAuthorizationWithSig((address,address,bool,uint256,uint256),(uint8,bytes32,bytes32))"
    )]
    pub struct SetAuthorizationWithSigCall {
        pub authorization: Authorization,
        pub signature: Signature,
    }
    ///Container type for all input parameters for the `setFee` function with signature `setFee((address,address,address,address,uint256),uint256)` and selector `0x2b4f013c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFee", abi = "setFee((address,address,address,address,uint256),uint256)")]
    pub struct SetFeeCall {
        pub market_params: MarketParams,
        pub new_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFeeRecipient` function with signature `setFeeRecipient(address)` and selector `0xe74b981b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFeeRecipient", abi = "setFeeRecipient(address)")]
    pub struct SetFeeRecipientCall {
        pub new_fee_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `0x13af4035`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supply` function with signature `supply((address,address,address,address,uint256),uint256,uint256,address,bytes)` and selector `0xa99aad89`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "supply",
        abi = "supply((address,address,address,address,uint256),uint256,uint256,address,bytes)"
    )]
    pub struct SupplyCall {
        pub market_params: MarketParams,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
        pub on_behalf: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `supplyCollateral` function with signature `supplyCollateral((address,address,address,address,uint256),uint256,address,bytes)` and selector `0x238d6579`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "supplyCollateral",
        abi = "supplyCollateral((address,address,address,address,uint256),uint256,address,bytes)"
    )]
    pub struct SupplyCollateralCall {
        pub market_params: MarketParams,
        pub assets: ::ethers::core::types::U256,
        pub on_behalf: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw((address,address,address,address,uint256),uint256,uint256,address,address)` and selector `0x5c2bea49`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "withdraw",
        abi = "withdraw((address,address,address,address,uint256),uint256,uint256,address,address)"
    )]
    pub struct WithdrawCall {
        pub market_params: MarketParams,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
        pub on_behalf: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawCollateral` function with signature `withdrawCollateral((address,address,address,address,uint256),uint256,address,address)` and selector `0x8720316d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "withdrawCollateral",
        abi = "withdrawCollateral((address,address,address,address,uint256),uint256,address,address)"
    )]
    pub struct WithdrawCollateralCall {
        pub market_params: MarketParams,
        pub assets: ::ethers::core::types::U256,
        pub on_behalf: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
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
    pub enum IMorphoCalls {
        DomainSeparator(DomainSeparatorCall),
        AccrueInterest(AccrueInterestCall),
        Borrow(BorrowCall),
        CreateMarket(CreateMarketCall),
        EnableIrm(EnableIrmCall),
        EnableLltv(EnableLltvCall),
        ExtSloads(ExtSloadsCall),
        FeeRecipient(FeeRecipientCall),
        FlashLoan(FlashLoanCall),
        IdToMarketParams(IdToMarketParamsCall),
        IsAuthorized(IsAuthorizedCall),
        IsIrmEnabled(IsIrmEnabledCall),
        IsLltvEnabled(IsLltvEnabledCall),
        Liquidate(LiquidateCall),
        Market(MarketCall),
        Nonce(NonceCall),
        Owner(OwnerCall),
        Position(PositionCall),
        Repay(RepayCall),
        SetAuthorization(SetAuthorizationCall),
        SetAuthorizationWithSig(SetAuthorizationWithSigCall),
        SetFee(SetFeeCall),
        SetFeeRecipient(SetFeeRecipientCall),
        SetOwner(SetOwnerCall),
        Supply(SupplyCall),
        SupplyCollateral(SupplyCollateralCall),
        Withdraw(WithdrawCall),
        WithdrawCollateral(WithdrawCollateralCall),
    }
    impl ::ethers::core::abi::AbiDecode for IMorphoCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <AccrueInterestCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AccrueInterest(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Borrow(decoded));
            }
            if let Ok(decoded) = <CreateMarketCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateMarket(decoded));
            }
            if let Ok(decoded) = <EnableIrmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableIrm(decoded));
            }
            if let Ok(decoded) = <EnableLltvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EnableLltv(decoded));
            }
            if let Ok(decoded) = <ExtSloadsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExtSloads(decoded));
            }
            if let Ok(decoded) = <FeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FeeRecipient(decoded));
            }
            if let Ok(decoded) = <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <IdToMarketParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IdToMarketParams(decoded));
            }
            if let Ok(decoded) = <IsAuthorizedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsAuthorized(decoded));
            }
            if let Ok(decoded) = <IsIrmEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsIrmEnabled(decoded));
            }
            if let Ok(decoded) = <IsLltvEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsLltvEnabled(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <MarketCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Market(decoded));
            }
            if let Ok(decoded) = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PositionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Position(decoded));
            }
            if let Ok(decoded) = <RepayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Repay(decoded));
            }
            if let Ok(decoded) =
                <SetAuthorizationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAuthorization(decoded));
            }
            if let Ok(decoded) =
                <SetAuthorizationWithSigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAuthorizationWithSig(decoded));
            }
            if let Ok(decoded) = <SetFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetFeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetFeeRecipient(decoded));
            }
            if let Ok(decoded) = <SetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOwner(decoded));
            }
            if let Ok(decoded) = <SupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Supply(decoded));
            }
            if let Ok(decoded) =
                <SupplyCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupplyCollateral(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawCollateral(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IMorphoCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AccrueInterest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Borrow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateMarket(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnableIrm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnableLltv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExtSloads(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeRecipient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FlashLoan(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IdToMarketParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsAuthorized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsIrmEnabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsLltvEnabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Liquidate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Market(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Position(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Repay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAuthorization(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAuthorizationWithSig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeRecipient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Supply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupplyCollateral(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IMorphoCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccrueInterest(element) => ::core::fmt::Display::fmt(element, f),
                Self::Borrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateMarket(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableIrm(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableLltv(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtSloads(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::IdToMarketParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAuthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsIrmEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsLltvEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Market(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Position(element) => ::core::fmt::Display::fmt(element, f),
                Self::Repay(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthorization(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthorizationWithSig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Supply(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupplyCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawCollateral(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for IMorphoCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AccrueInterestCall> for IMorphoCalls {
        fn from(value: AccrueInterestCall) -> Self {
            Self::AccrueInterest(value)
        }
    }
    impl ::core::convert::From<BorrowCall> for IMorphoCalls {
        fn from(value: BorrowCall) -> Self {
            Self::Borrow(value)
        }
    }
    impl ::core::convert::From<CreateMarketCall> for IMorphoCalls {
        fn from(value: CreateMarketCall) -> Self {
            Self::CreateMarket(value)
        }
    }
    impl ::core::convert::From<EnableIrmCall> for IMorphoCalls {
        fn from(value: EnableIrmCall) -> Self {
            Self::EnableIrm(value)
        }
    }
    impl ::core::convert::From<EnableLltvCall> for IMorphoCalls {
        fn from(value: EnableLltvCall) -> Self {
            Self::EnableLltv(value)
        }
    }
    impl ::core::convert::From<ExtSloadsCall> for IMorphoCalls {
        fn from(value: ExtSloadsCall) -> Self {
            Self::ExtSloads(value)
        }
    }
    impl ::core::convert::From<FeeRecipientCall> for IMorphoCalls {
        fn from(value: FeeRecipientCall) -> Self {
            Self::FeeRecipient(value)
        }
    }
    impl ::core::convert::From<FlashLoanCall> for IMorphoCalls {
        fn from(value: FlashLoanCall) -> Self {
            Self::FlashLoan(value)
        }
    }
    impl ::core::convert::From<IdToMarketParamsCall> for IMorphoCalls {
        fn from(value: IdToMarketParamsCall) -> Self {
            Self::IdToMarketParams(value)
        }
    }
    impl ::core::convert::From<IsAuthorizedCall> for IMorphoCalls {
        fn from(value: IsAuthorizedCall) -> Self {
            Self::IsAuthorized(value)
        }
    }
    impl ::core::convert::From<IsIrmEnabledCall> for IMorphoCalls {
        fn from(value: IsIrmEnabledCall) -> Self {
            Self::IsIrmEnabled(value)
        }
    }
    impl ::core::convert::From<IsLltvEnabledCall> for IMorphoCalls {
        fn from(value: IsLltvEnabledCall) -> Self {
            Self::IsLltvEnabled(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for IMorphoCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<MarketCall> for IMorphoCalls {
        fn from(value: MarketCall) -> Self {
            Self::Market(value)
        }
    }
    impl ::core::convert::From<NonceCall> for IMorphoCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for IMorphoCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PositionCall> for IMorphoCalls {
        fn from(value: PositionCall) -> Self {
            Self::Position(value)
        }
    }
    impl ::core::convert::From<RepayCall> for IMorphoCalls {
        fn from(value: RepayCall) -> Self {
            Self::Repay(value)
        }
    }
    impl ::core::convert::From<SetAuthorizationCall> for IMorphoCalls {
        fn from(value: SetAuthorizationCall) -> Self {
            Self::SetAuthorization(value)
        }
    }
    impl ::core::convert::From<SetAuthorizationWithSigCall> for IMorphoCalls {
        fn from(value: SetAuthorizationWithSigCall) -> Self {
            Self::SetAuthorizationWithSig(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for IMorphoCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    impl ::core::convert::From<SetFeeRecipientCall> for IMorphoCalls {
        fn from(value: SetFeeRecipientCall) -> Self {
            Self::SetFeeRecipient(value)
        }
    }
    impl ::core::convert::From<SetOwnerCall> for IMorphoCalls {
        fn from(value: SetOwnerCall) -> Self {
            Self::SetOwner(value)
        }
    }
    impl ::core::convert::From<SupplyCall> for IMorphoCalls {
        fn from(value: SupplyCall) -> Self {
            Self::Supply(value)
        }
    }
    impl ::core::convert::From<SupplyCollateralCall> for IMorphoCalls {
        fn from(value: SupplyCollateralCall) -> Self {
            Self::SupplyCollateral(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for IMorphoCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawCollateralCall> for IMorphoCalls {
        fn from(value: WithdrawCollateralCall) -> Self {
            Self::WithdrawCollateral(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `borrow` function with signature `borrow((address,address,address,address,uint256),uint256,uint256,address,address)` and selector `0x50d8cd4b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BorrowReturn {
        pub assets_borrowed: ::ethers::core::types::U256,
        pub shares_borrowed: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `extSloads` function with signature `extSloads(bytes32[])` and selector `0x7784c685`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExtSloadsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `feeRecipient` function with signature `feeRecipient()` and selector `0x46904840`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `idToMarketParams` function with signature `idToMarketParams(bytes32)` and selector `0x2c3c9157`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IdToMarketParamsReturn(pub MarketParams);
    ///Container type for all return fields from the `isAuthorized` function with signature `isAuthorized(address,address)` and selector `0x65e4ad9e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsAuthorizedReturn(pub bool);
    ///Container type for all return fields from the `isIrmEnabled` function with signature `isIrmEnabled(address)` and selector `0xf2b863ce`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsIrmEnabledReturn(pub bool);
    ///Container type for all return fields from the `isLltvEnabled` function with signature `isLltvEnabled(uint256)` and selector `0xb485f3b8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsLltvEnabledReturn(pub bool);
    ///Container type for all return fields from the `liquidate` function with signature `liquidate((address,address,address,address,uint256),address,uint256,uint256,bytes)` and selector `0xd8eabcb8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LiquidateReturn(pub ::ethers::core::types::U256, pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `market` function with signature `market(bytes32)` and selector `0x5c60e39a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MarketReturn {
        pub m: Market,
    }
    ///Container type for all return fields from the `nonce` function with signature `nonce(address)` and selector `0x70ae92d2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `position` function with signature `position(bytes32,address)` and selector `0x93c52062`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PositionReturn {
        pub p: Position,
    }
    ///Container type for all return fields from the `repay` function with signature `repay((address,address,address,address,uint256),uint256,uint256,address,bytes)` and selector `0x20b76e81`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RepayReturn {
        pub assets_repaid: ::ethers::core::types::U256,
        pub shares_repaid: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `supply` function with signature `supply((address,address,address,address,uint256),uint256,uint256,address,bytes)` and selector `0xa99aad89`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SupplyReturn {
        pub assets_supplied: ::ethers::core::types::U256,
        pub shares_supplied: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `withdraw` function with signature `withdraw((address,address,address,address,uint256),uint256,uint256,address,address)` and selector `0x5c2bea49`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WithdrawReturn {
        pub assets_withdrawn: ::ethers::core::types::U256,
        pub shares_withdrawn: ::ethers::core::types::U256,
    }
    ///`Position(uint256,uint128,uint128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Position {
        pub supply_shares: ::ethers::core::types::U256,
        pub borrow_shares: u128,
        pub collateral: u128,
    }
}
