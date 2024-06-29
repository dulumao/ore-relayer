use std::mem::size_of;

use ore_api::consts::*;
use ore_stake_api::{consts::*, instruction::InitializeArgs, loaders::*, state::Stake};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    system_program,
    sysvar::{self},
};

use crate::utils::{create_pda, AccountDeserialize, Discriminator};

/// Initializes a new stake account.
pub fn process_initialize<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args.
    let args = InitializeArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, miner_info, mint_info, proof_info, stake_info, stake_tokens_info, system_program, token_program, associated_token_program, slot_hashes_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_system_account(miner_info, false)?;
    load_mint(mint_info, MINT_ADDRESS, false)?;
    load_uninitialized_pda(
        proof_info,
        &[PROOF, stake_info.key.as_ref()],
        args.proof_bump,
        &ore_api::id(),
    )?;
    load_uninitialized_pda(
        stake_info,
        &[STAKE, signer.key.as_ref()],
        args.stake_bump,
        &ore_api::id(),
    )?;
    load_system_account(stake_tokens_info, true)?;
    load_program(system_program, system_program::id())?;
    load_program(token_program, spl_token::id())?;
    load_program(associated_token_program, spl_associated_token_account::id())?;
    load_sysvar(slot_hashes_info, sysvar::slot_hashes::id())?;

    // Initialize the stake account.
    create_pda(
        stake_info,
        &ore_stake_api::id(),
        8 + size_of::<Stake>(),
        &[STAKE, signer.key.as_ref(), &[args.stake_bump]],
        system_program,
        signer,
    )?;
    let mut stake_data = stake_info.data.borrow_mut();
    stake_data[0] = Stake::discriminator() as u8;
    let stake = Stake::try_from_bytes_mut(&mut stake_data)?;
    stake.authority = *signer.key;
    stake.bump = args.stake_bump as u64;
    stake.is_liquid = 0;
    stake.is_open = 0;
    drop(stake_data);

    // Initialize a token account to escrow stake.
    solana_program::program::invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            signer.key,
            stake_info.key,
            mint_info.key,
            &spl_token::id(),
        ),
        &[
            associated_token_program.clone(),
            signer.clone(),
            stake_tokens_info.clone(),
            stake_info.clone(),
            mint_info.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // Open a proof account for mining.
    solana_program::program::invoke_signed(
        &ore_api::instruction::open(*stake_info.key, *miner_info.key),
        &[
            stake_info.clone(),
            miner_info.clone(),
            proof_info.clone(),
            system_program.clone(),
            slot_hashes_info.clone(),
        ],
        &[&[STAKE, signer.key.as_ref(), &[args.stake_bump]]],
    )?;

    Ok(())
}
