<!-- cargo-sync-readme start -->

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE-MIT)
[![Apache License 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)
[![docs.rs](https://docs.rs/oid-registry/badge.svg)](https://docs.rs/oid-registry)
[![crates.io](https://img.shields.io/crates/v/oid-registry.svg)](https://crates.io/crates/oid-registry)
[![Github CI](https://github.com/rusticata/oid-registry/workflows/Continuous%20integration/badge.svg)](https://github.com/rusticata/oid-registry/actions)
# OID Registry

This crate is a helper crate, containing a database of OID objects. These objects are intended
for use when manipulating ASN.1 grammars and BER/DER encodings, for example.

This crate provides only a simple registry (similar to a `HashMap`) by default. This object can
be used to get names and descriptions from OID.

By default, the registry is provided empty.
This crate can provide default lists of known OIDs, that can be selected using the build
features.
<!-- cargo-sync-readme end -->

# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
