use solana_program::{pubkey, pubkey::Pubkey};

/// Miner pubkey
pub const MINER_PUBKEY: Pubkey = pubkey!("F7coAFJKxeo1btofymv6f6KFmN5LUC9JEGRATRqwQCXL");

/// The seed of the escrow account PDA.
pub const ESCROW: &[u8] = b"escrow";

/// The ore commission the relayer is allowed to collect
pub const COMMISSION: u64 = 10_000;
