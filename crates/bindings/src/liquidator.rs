pub use liquidator::*;
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
pub mod liquidator {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("liquidateUser"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidateUser"),
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
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapper"),
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
                                name: ::std::borrow::ToOwned::to_owned("swapData"),
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
                    ::std::borrow::ToOwned::to_owned("onMorphoLiquidate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onMorphoLiquidate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("repaidAssets"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("target"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[Pa\x0C7\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80c\xC4\\\xAF=\x14a\08W\x80c\xCF~\xA1\x96\x14a\0MW[_\x80\xFD[a\0Ka\0F6`\x04a\x08\x7FV[a\0`V[\0[a\0Ka\0[6`\x04a\tQV[a\x02\x95V[s\xBB\xBB\xBB\xBB\xBB\x9C\xC5\xE9\x0E;:\xF6K\xDA\xF6,7\xEE\xFF\xCB`\x01`\x01`\xA0\x1B\x03\x16c\xD8\xEA\xBC\xB8\x87\x87\x86_`@Q\x80`\x80\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP`@Qa\x01\r\x91\x90` \x01a\t\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01<\x95\x94\x93\x92\x91\x90a\n\x15V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x01WW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01{\x91\x90a\n\x88V[PP` \x86\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xEA\x91\x90a\n\xAAV[\x90P\x80\x15a\x02\x06Wa\x02\x06`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a\x03\xD6V[\x87Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02JW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02n\x91\x90a\n\xAAV[\x91P\x81\x15a\x02\x8AWa\x02\x8A`\x01`\x01`\xA0\x1B\x03\x82\x163\x84a\x03\xD6V[PPPPPPPPPV[s\xBB\xBB\xBB\xBB\xBB\x9C\xC5\xE9\x0E;:\xF6K\xDA\xF6,7\xEE\xFF\xCB3\x14a\x02\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FLiquidator: Invalid address\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\x03\n\x82\x84\x01\x84a\n\xC1V[` \x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92P_\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03WW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03{\x91\x90a\n\xAAV[\x82Q` \x84\x01Q\x91\x92Pa\x03\x99\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83a\x04:V[``\x82\x01Q\x82Qa\x03\xB5\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x04\xC7V[P`@\x82\x01Qa\x03\xCF\x90`\x01`\x01`\xA0\x1B\x03\x163\x87a\x04:V[PPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\x045\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\x04\xDDV[PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x87W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xAB\x91\x90a\n\xAAV[\x90Pa\x04\xC1\x84\x84a\x04\xBC\x85\x85a\x0B\xADV[a\x05>V[PPPPV[``a\x04\xD4\x83\x83_a\x05\xCDV[\x90P[\x92\x91PPV[_a\x04\xF1`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x04\xC7V[\x90P\x80Q_\x14\x15\x80\x15a\x05\x15WP\x80\x80` \x01\x90Q\x81\x01\x90a\x05\x13\x91\x90a\x0B\xCCV[\x15[\x15a\x045W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x02\xF4V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x05\x8F\x84\x82a\x06hV[a\x04\xC1W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R_`D\x83\x01Ra\x05\xC3\x91\x86\x91\x82\x16\x90c\t^\xA7\xB3\x90`d\x01a\x04\x03V[a\x04\xC1\x84\x82a\x04\xDDV[``\x81G\x10\x15a\x05\xF2W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x02\xF4V[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x06\r\x91\x90a\x0B\xEBV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x06GW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06LV[``\x91P[P\x91P\x91Pa\x06\\\x86\x83\x83a\x07\tV[\x92PPP[\x93\x92PPPV[_\x80_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x06\x83\x91\x90a\x0B\xEBV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x06\xBCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\xC1V[``\x91P[P\x91P\x91P\x81\x80\x15a\x06\xEBWP\x80Q\x15\x80a\x06\xEBWP\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xEB\x91\x90a\x0B\xCCV[\x80\x15a\x07\0WP_\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[``\x82a\x07\x1EWa\x07\x19\x82a\x07eV[a\x06aV[\x81Q\x15\x80\x15a\x075WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x07^W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x02\xF4V[P\x80a\x06aV[\x80Q\x15a\x07uW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xC5Wa\x07\xC5a\x07\x8EV[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xC5Wa\x07\xC5a\x07\x8EV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\x17Wa\x08\x17a\x07\x8EV[`@R\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x085W_\x80\xFD[\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12a\x08JW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08aW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08xW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80\x86\x88\x03a\x01 \x81\x12\x15a\x08\x96W_\x80\xFD[`\xA0\x81\x12\x15a\x08\xA3W_\x80\xFD[Pa\x08\xACa\x07\xA2V[a\x08\xB5\x88a\x08\x1FV[\x81Ra\x08\xC3` \x89\x01a\x08\x1FV[` \x82\x01Ra\x08\xD4`@\x89\x01a\x08\x1FV[`@\x82\x01Ra\x08\xE5``\x89\x01a\x08\x1FV[``\x82\x01R`\x80\x88\x81\x015\x90\x82\x01R\x95Pa\t\x02`\xA0\x88\x01a\x08\x1FV[\x94Pa\t\x10`\xC0\x88\x01a\x08\x1FV[\x93P`\xE0\x87\x015\x92Pa\x01\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t3W_\x80\xFD[a\t?\x89\x82\x8A\x01a\x08:V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_\x80_`@\x84\x86\x03\x12\x15a\tcW_\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x80W_\x80\xFD[a\t\x8C\x86\x82\x87\x01a\x08:V[\x94\x97\x90\x96P\x93\x94PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_`\x01\x80`\xA0\x1B\x03\x80\x84Q\x16` \x84\x01R\x80` \x85\x01Q\x16`@\x84\x01R\x80`@\x85\x01Q\x16``\x84\x01RP``\x83\x01Q`\x80\x80\x84\x01Ra\n\r`\xA0\x84\x01\x82a\t\x99V[\x94\x93PPPPV[_a\x01 `\x01\x80`\xA0\x1B\x03\x80\x89Q\x16\x84R\x80` \x8A\x01Q\x16` \x85\x01R\x80`@\x8A\x01Q\x16`@\x85\x01R\x80``\x8A\x01Q\x16``\x85\x01R`\x80\x89\x01Q`\x80\x85\x01R\x80\x88\x16`\xA0\x85\x01RP\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra\n|\x81\x84\x01\x85a\t\x99V[\x98\x97PPPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\n\x99W_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[_` \x82\x84\x03\x12\x15a\n\xBAW_\x80\xFD[PQ\x91\x90PV[_` \x80\x83\x85\x03\x12\x15a\n\xD2W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xE9W_\x80\xFD[\x90\x84\x01\x90`\x80\x82\x87\x03\x12\x15a\n\xFCW_\x80\xFD[a\x0B\x04a\x07\xCBV[a\x0B\r\x83a\x08\x1FV[\x81Ra\x0B\x1A\x84\x84\x01a\x08\x1FV[\x84\x82\x01Ra\x0B*`@\x84\x01a\x08\x1FV[`@\x82\x01R``\x83\x015\x82\x81\x11\x15a\x0B@W_\x80\xFD[\x80\x84\x01\x93PP\x86`\x1F\x84\x01\x12a\x0BTW_\x80\xFD[\x825\x82\x81\x11\x15a\x0BfWa\x0Bfa\x07\x8EV[a\x0Bx`\x1F\x82\x01`\x1F\x19\x16\x86\x01a\x07\xEEV[\x92P\x80\x83R\x87\x85\x82\x86\x01\x01\x11\x15a\x0B\x8DW_\x80\xFD[\x80\x85\x85\x01\x86\x85\x017_\x90\x83\x01\x90\x94\x01\x93\x90\x93R``\x83\x01RP\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x04\xD7WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x0B\xDCW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06aW_\x80\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \xF8\xEEl\x1Bf\xF6)q\xAF\xED\xC0\x95a\0$\x85\xE5\n\xEE\x14\\?\xC4\xE4by\xB1v\xF3N\x80\xA9dsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80c\xC4\\\xAF=\x14a\08W\x80c\xCF~\xA1\x96\x14a\0MW[_\x80\xFD[a\0Ka\0F6`\x04a\x08\x7FV[a\0`V[\0[a\0Ka\0[6`\x04a\tQV[a\x02\x95V[s\xBB\xBB\xBB\xBB\xBB\x9C\xC5\xE9\x0E;:\xF6K\xDA\xF6,7\xEE\xFF\xCB`\x01`\x01`\xA0\x1B\x03\x16c\xD8\xEA\xBC\xB8\x87\x87\x86_`@Q\x80`\x80\x01`@R\x80\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP`@Qa\x01\r\x91\x90` \x01a\t\xC7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01<\x95\x94\x93\x92\x91\x90a\n\x15V[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x01WW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01{\x91\x90a\n\x88V[PP` \x86\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xEA\x91\x90a\n\xAAV[\x90P\x80\x15a\x02\x06Wa\x02\x06`\x01`\x01`\xA0\x1B\x03\x83\x163\x83a\x03\xD6V[\x87Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02JW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02n\x91\x90a\n\xAAV[\x91P\x81\x15a\x02\x8AWa\x02\x8A`\x01`\x01`\xA0\x1B\x03\x82\x163\x84a\x03\xD6V[PPPPPPPPPV[s\xBB\xBB\xBB\xBB\xBB\x9C\xC5\xE9\x0E;:\xF6K\xDA\xF6,7\xEE\xFF\xCB3\x14a\x02\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FLiquidator: Invalid address\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\x03\n\x82\x84\x01\x84a\n\xC1V[` \x81\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92P_\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03WW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03{\x91\x90a\n\xAAV[\x82Q` \x84\x01Q\x91\x92Pa\x03\x99\x91`\x01`\x01`\xA0\x1B\x03\x16\x90\x83a\x04:V[``\x82\x01Q\x82Qa\x03\xB5\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90a\x04\xC7V[P`@\x82\x01Qa\x03\xCF\x90`\x01`\x01`\xA0\x1B\x03\x163\x87a\x04:V[PPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`D\x82\x01\x83\x90Ra\x045\x91\x85\x91\x82\x16\x90c\xA9\x05\x9C\xBB\x90`d\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P`\xE0\x1B` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x83\x81\x83\x16\x17\x83RPPPPa\x04\xDDV[PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x87W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xAB\x91\x90a\n\xAAV[\x90Pa\x04\xC1\x84\x84a\x04\xBC\x85\x85a\x0B\xADV[a\x05>V[PPPPV[``a\x04\xD4\x83\x83_a\x05\xCDV[\x90P[\x92\x91PPV[_a\x04\xF1`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x04\xC7V[\x90P\x80Q_\x14\x15\x80\x15a\x05\x15WP\x80\x80` \x01\x90Q\x81\x01\x90a\x05\x13\x91\x90a\x0B\xCCV[\x15[\x15a\x045W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x02\xF4V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x05\x8F\x84\x82a\x06hV[a\x04\xC1W`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R_`D\x83\x01Ra\x05\xC3\x91\x86\x91\x82\x16\x90c\t^\xA7\xB3\x90`d\x01a\x04\x03V[a\x04\xC1\x84\x82a\x04\xDDV[``\x81G\x10\x15a\x05\xF2W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x02\xF4V[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x06\r\x91\x90a\x0B\xEBV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x06GW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06LV[``\x91P[P\x91P\x91Pa\x06\\\x86\x83\x83a\x07\tV[\x92PPP[\x93\x92PPPV[_\x80_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x06\x83\x91\x90a\x0B\xEBV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x06\xBCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x06\xC1V[``\x91P[P\x91P\x91P\x81\x80\x15a\x06\xEBWP\x80Q\x15\x80a\x06\xEBWP\x80\x80` \x01\x90Q\x81\x01\x90a\x06\xEB\x91\x90a\x0B\xCCV[\x80\x15a\x07\0WP_\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[``\x82a\x07\x1EWa\x07\x19\x82a\x07eV[a\x06aV[\x81Q\x15\x80\x15a\x075WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x07^W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x02\xF4V[P\x80a\x06aV[\x80Q\x15a\x07uW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xC5Wa\x07\xC5a\x07\x8EV[`@R\x90V[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xC5Wa\x07\xC5a\x07\x8EV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08\x17Wa\x08\x17a\x07\x8EV[`@R\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x085W_\x80\xFD[\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12a\x08JW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08aW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08xW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80\x86\x88\x03a\x01 \x81\x12\x15a\x08\x96W_\x80\xFD[`\xA0\x81\x12\x15a\x08\xA3W_\x80\xFD[Pa\x08\xACa\x07\xA2V[a\x08\xB5\x88a\x08\x1FV[\x81Ra\x08\xC3` \x89\x01a\x08\x1FV[` \x82\x01Ra\x08\xD4`@\x89\x01a\x08\x1FV[`@\x82\x01Ra\x08\xE5``\x89\x01a\x08\x1FV[``\x82\x01R`\x80\x88\x81\x015\x90\x82\x01R\x95Pa\t\x02`\xA0\x88\x01a\x08\x1FV[\x94Pa\t\x10`\xC0\x88\x01a\x08\x1FV[\x93P`\xE0\x87\x015\x92Pa\x01\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t3W_\x80\xFD[a\t?\x89\x82\x8A\x01a\x08:V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_\x80_`@\x84\x86\x03\x12\x15a\tcW_\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x80W_\x80\xFD[a\t\x8C\x86\x82\x87\x01a\x08:V[\x94\x97\x90\x96P\x93\x94PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_`\x01\x80`\xA0\x1B\x03\x80\x84Q\x16` \x84\x01R\x80` \x85\x01Q\x16`@\x84\x01R\x80`@\x85\x01Q\x16``\x84\x01RP``\x83\x01Q`\x80\x80\x84\x01Ra\n\r`\xA0\x84\x01\x82a\t\x99V[\x94\x93PPPPV[_a\x01 `\x01\x80`\xA0\x1B\x03\x80\x89Q\x16\x84R\x80` \x8A\x01Q\x16` \x85\x01R\x80`@\x8A\x01Q\x16`@\x85\x01R\x80``\x8A\x01Q\x16``\x85\x01R`\x80\x89\x01Q`\x80\x85\x01R\x80\x88\x16`\xA0\x85\x01RP\x85`\xC0\x84\x01R\x84`\xE0\x84\x01R\x80a\x01\0\x84\x01Ra\n|\x81\x84\x01\x85a\t\x99V[\x98\x97PPPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a\n\x99W_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[_` \x82\x84\x03\x12\x15a\n\xBAW_\x80\xFD[PQ\x91\x90PV[_` \x80\x83\x85\x03\x12\x15a\n\xD2W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xE9W_\x80\xFD[\x90\x84\x01\x90`\x80\x82\x87\x03\x12\x15a\n\xFCW_\x80\xFD[a\x0B\x04a\x07\xCBV[a\x0B\r\x83a\x08\x1FV[\x81Ra\x0B\x1A\x84\x84\x01a\x08\x1FV[\x84\x82\x01Ra\x0B*`@\x84\x01a\x08\x1FV[`@\x82\x01R``\x83\x015\x82\x81\x11\x15a\x0B@W_\x80\xFD[\x80\x84\x01\x93PP\x86`\x1F\x84\x01\x12a\x0BTW_\x80\xFD[\x825\x82\x81\x11\x15a\x0BfWa\x0Bfa\x07\x8EV[a\x0Bx`\x1F\x82\x01`\x1F\x19\x16\x86\x01a\x07\xEEV[\x92P\x80\x83R\x87\x85\x82\x86\x01\x01\x11\x15a\x0B\x8DW_\x80\xFD[\x80\x85\x85\x01\x86\x85\x017_\x90\x83\x01\x90\x94\x01\x93\x90\x93R``\x83\x01RP\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x04\xD7WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x0B\xDCW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x06aW_\x80\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \xF8\xEEl\x1Bf\xF6)q\xAF\xED\xC0\x95a\0$\x85\xE5\n\xEE\x14\\?\xC4\xE4by\xB1v\xF3N\x80\xA9dsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), LIQUIDATOR_ABI.clone(), client))
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
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `liquidateUser` (0xc45caf3d) function
        pub fn liquidate_user(
            &self,
            market_params: MarketParams,
            user: ::ethers::core::types::Address,
            swapper: ::ethers::core::types::Address,
            seized_assets: ::ethers::core::types::U256,
            swap_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [196, 92, 175, 61],
                    (market_params, user, swapper, seized_assets, swap_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onMorphoLiquidate` (0xcf7ea196) function
        pub fn on_morpho_liquidate(
            &self,
            repaid_assets: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 126, 161, 150], (repaid_assets, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Liquidator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AddressInsufficientBalance", abi = "AddressInsufficientBalance(address)")]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SafeERC20FailedOperation", abi = "SafeERC20FailedOperation(address)")]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
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
    pub enum LiquidatorErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) =
                <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) =
                <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LiquidatorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LiquidatorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for LiquidatorErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for LiquidatorErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for LiquidatorErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for LiquidatorErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    ///Container type for all input parameters for the `liquidateUser` function with signature `liquidateUser((address,address,address,address,uint256),address,address,uint256,bytes)` and selector `0xc45caf3d`
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
        name = "liquidateUser",
        abi = "liquidateUser((address,address,address,address,uint256),address,address,uint256,bytes)"
    )]
    pub struct LiquidateUserCall {
        pub market_params: MarketParams,
        pub user: ::ethers::core::types::Address,
        pub swapper: ::ethers::core::types::Address,
        pub seized_assets: ::ethers::core::types::U256,
        pub swap_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `onMorphoLiquidate` function with signature `onMorphoLiquidate(uint256,bytes)` and selector `0xcf7ea196`
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
    #[ethcall(name = "onMorphoLiquidate", abi = "onMorphoLiquidate(uint256,bytes)")]
    pub struct OnMorphoLiquidateCall {
        pub repaid_assets: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum LiquidatorCalls {
        LiquidateUser(LiquidateUserCall),
        OnMorphoLiquidate(OnMorphoLiquidateCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LiquidateUserCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LiquidateUser(decoded));
            }
            if let Ok(decoded) =
                <OnMorphoLiquidateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnMorphoLiquidate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::LiquidateUser(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnMorphoLiquidate(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LiquidateUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnMorphoLiquidate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LiquidateUserCall> for LiquidatorCalls {
        fn from(value: LiquidateUserCall) -> Self {
            Self::LiquidateUser(value)
        }
    }
    impl ::core::convert::From<OnMorphoLiquidateCall> for LiquidatorCalls {
        fn from(value: OnMorphoLiquidateCall) -> Self {
            Self::OnMorphoLiquidate(value)
        }
    }
}
