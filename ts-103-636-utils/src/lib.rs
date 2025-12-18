//! <!--
//! SPDX-FileCopyrightText: Copyright Christian Amsüss <chrysn@fsfe.org>, Silano Systems
//! SPDX-License-Identifier: MIT OR Apache-2.0
//! -->
//! This crate contains protocol utilities ETSI TS 103-636 "DECT-2020 New Radio (NR)".
#![no_std]

pub mod mac_ie;
pub mod mac_pdu;

/// Something in the input data structure violated this crate's expectation of what specification
/// compliant input should look like.
///
/// As this is not an actionable error, it has no inner structure.
#[derive(Copy, Clone, Debug)]
pub struct ParsingError;

/// An input slice mismatches the length usable the requested data item.
#[derive(Copy, Clone, Debug)]
pub struct InputLengthError;
