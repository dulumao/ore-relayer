use std::mem::size_of;

use ore_relay_api::{consts::*, instruction::OpenRelayerArgs, loaders::*, state::Relayer};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    system_program,
};
use utils::{create_pda, AccountDeserialize, Discriminator};

/// Opens a new relay account.
pub fn process_open_relayer<'a, 'info>(
    accounts: &'a [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    // Parse args
    let args = OpenRelayerArgs::try_from_bytes(data)?;

    // Load accounts.
    let [signer, relayer_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    load_signer(signer)?;
    load_uninitialized_pda(
        relayer_info,
        &[RELAYER, signer.key.as_ref()],
        args.bump,
        &ore_relay_api::id(),
    )?;
    load_program(system_program, system_program::id())?;

    // Initialize relay account.
    create_pda(
        relayer_info,
        &ore_relay_api::id(),
        8 + size_of::<Relayer>(),
        &[RELAYER, signer.key.as_ref(), &[args.bump]],
        system_program,
        signer,
    )?;
    let mut relayer_data = relayer_info.data.borrow_mut();
    relayer_data[0] = Relayer::discriminator() as u8;
    let relayer = Relayer::try_from_bytes_mut(&mut relayer_data)?;
    relayer.authority = *signer.key;
    relayer.bump = args.bump as u64;

    Ok(())
}
