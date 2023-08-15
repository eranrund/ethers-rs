#![doc = include_str!("../README.md")]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(target_arch = "wasm32"), deny(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

pub mod types;

pub mod abi;

/// Various utilities
pub mod utils;

#[cfg(feature = "macros")]
pub mod macros;

// re-export rand to avoid potential confusion when there's rand version mismatches
pub use rand;

// re-export k256
pub use k256;
