pub mod consts;
pub mod error;
pub mod instruction;
pub mod loaders;
pub mod state;

pub(crate) use ore_utils as utils;

use solana_program::declare_id;

// TODO
// declare_id!("stakeHF5r6S7HyD9SppBfVMXMavDkJsxwGesEvxZr2A");
declare_id!("2zH7PoRGwdybmN59jN5nmaXXfJ6C8sejqyT4h7xRARUd");
