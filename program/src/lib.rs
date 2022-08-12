use solana_program::{entrypoint, msg};

pub fn process_instruction(
    program_id: &solana_program::pubkey::Pubkey,
    accounts: &[solana_program::account_info::AccountInfo],
    instruction_data: &[u8],
) -> solana_program::entrypoint::ProgramResult {
    msg!(">>> Hello, world!");
    msg!(">>> program_id: {:?}", program_id);
    msg!(">>> accounts: {:?}", accounts);
    msg!(">>> instruction_data: {:?}", instruction_data);
    Ok(())
}

entrypoint!(process_instruction);
