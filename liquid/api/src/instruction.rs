use bytemuck::{Pod, Zeroable};
use num_enum::TryFromPrimitive;
use ore_api::consts::*;
use shank::ShankInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::{
    consts::*,
    utils::{impl_instruction_from_bytes, impl_to_bytes},
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, ShankInstruction, TryFromPrimitive)]
#[rustfmt::skip]
pub enum StakeInstruction {
    #[account(0, name = "stake_program", desc = "ORE stake program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "miner", desc = "Miner authority")]
    #[account(3, name = "pool", desc = "ORE pool account", writable)]
    #[account(4, name = "pool_tokens", desc = "ORE pool escrow account", writable)]
    #[account(5, name = "proof", desc = "ORE proof account", writable)]
    #[account(6, name = "system_program", desc = "Solana system program")]
    #[account(7, name = "token_program", desc = "SPL token program")]
    #[account(8, name = "associated_token_program", desc = "SPL associated token program")]
    #[account(9, name = "slot_hashes", desc = "Solana slot hashes sysvar")]
    Initialize = 0,

    #[account(0, name = "stake_program", desc = "ORE stake program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "delegate", desc = "ORE stake delegate account", writable)]
    #[account(3, name = "pool", desc = "ORE pool account", writable)]
    #[account(4, name = "pool_tokens", desc = "ORE pool escrow account", writable)]
    #[account(5, name = "proof", desc = "ORE proof account", writable)]
    #[account(6, name = "sender", desc = "Signer token account", writable)]
    #[account(7, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(8, name = "token_program", desc = "SPL token program")]
    Delegate = 2, // TODO Rename to Stake

    #[account(0, name = "stake_program", desc = "ORE stake program")]
    #[account(1, name = "signer", desc = "Signer", signer)]
    #[account(2, name = "beneficiary", desc = "Beneficiary token account", writable)]
    #[account(3, name = "delegate", desc = "ORE stake delegate account", writable)]
    #[account(4, name = "pool", desc = "ORE pool account", writable)]
    #[account(5, name = "pool_tokens", desc = "ORE pool escrow account", writable)]
    #[account(6, name = "proof", desc = "ORE proof account", writable)]
    #[account(7, name = "treasury_tokens", desc = "ORE treasury token account", writable)]
    #[account(8, name = "token_program", desc = "SPL token program")]
    Withdraw = 3, // TODO Rename to Unstake

    // TODO Update pool account
}

impl StakeInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InitializeArgs {
    pub mint_bump: u8,
    pub pool_bump: u8,
    pub proof_bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DelegateArgs {
    pub amount: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct WithdrawArgs {
    pub amount: u64,
}

// impl_to_bytes!(OpenArgs);
impl_to_bytes!(DelegateArgs);
impl_to_bytes!(InitializeArgs);
impl_to_bytes!(WithdrawArgs);

// impl_instruction_from_bytes!(OpenArgs);
impl_instruction_from_bytes!(DelegateArgs);
impl_instruction_from_bytes!(InitializeArgs);
impl_instruction_from_bytes!(WithdrawArgs);

/// Builds an initialize instruction.
pub fn initialize(signer: Pubkey, miner: Pubkey) -> Instruction {
    let pool_pda = Pubkey::find_program_address(&[POOL, signer.as_ref()], &crate::id());
    let proof_pda = Pubkey::find_program_address(&[PROOF, pool_pda.0.as_ref()], &ore_api::id());
    let mint_pda = Pubkey::find_program_address(&[MINT, pool_pda.0.as_ref()], &crate::id());
    let pool_tokens_address =
        spl_associated_token_account::get_associated_token_address(&pool_pda.0, &MINT_ADDRESS);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(miner, false),
            AccountMeta::new(mint_pda.0, false),
            AccountMeta::new_readonly(MINT_ADDRESS, false),
            AccountMeta::new(pool_pda.0, false),
            AccountMeta::new(pool_tokens_address, false),
            AccountMeta::new(proof_pda.0, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(sysvar::slot_hashes::id(), false),
        ],
        data: [
            StakeInstruction::Initialize.to_vec(),
            InitializeArgs {
                mint_bump: mint_pda.1,
                pool_bump: pool_pda.1,
                proof_bump: proof_pda.1,
            }
            .to_bytes()
            .to_vec(),
        ]
        .concat(),
    }
}
