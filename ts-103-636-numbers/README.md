# ts-103-636-numbers ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![ts-103-636-numbers on crates.io](https://img.shields.io/crates/v/ts-103-636-numbers)](https://crates.io/crates/ts-103-636-numbers) [![ts-103-636-numbers on docs.rs](https://docs.rs/ts-103-636-numbers/badge.svg)](https://docs.rs/ts-103-636-numbers) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/ariel-os/hophop/)

This crate contains numeric constants for ETSI TS 103-636 “DECT-2020 New Radio (NR)”.

This encompasses bitfield values from the specification itself as well as entries in the
extensible [DECT-2020 NR Endpoint Multiplexing Address Allocation][__link0] mapping.

This crate tries to strike a balance between not being opinionated in terms of types and being
practical: It uses newtypes of the minimal suitable bit length to express constants, which
provide Debug implementations that may include a lot of code (relying on dead code eliminiation
to not weigh down users needlessly).


 [__link0]: https://portal.etsi.org/PNNS/Protocol-Specification-Allocation/DECT-2020-NR-Endpoint-Multiplexing-Addresses
