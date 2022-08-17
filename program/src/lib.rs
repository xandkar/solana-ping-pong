use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info,
    entrypoint,
    entrypoint::ProgramResult,
    //msg,
};

pub fn process_instruction(
    _program_id: &solana_program::pubkey::Pubkey,
    accounts: &[solana_program::account_info::AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let req = protocol::client::Request::try_from_slice(data)?;
    let accounts = &mut accounts.iter();
    let buffer_account = next_account_info(accounts)?;
    // XXX Official example adds a manual validation that
    //         buffer_account.owner == program_id
    //     but my experiments seem to show it isn't necessary, since incorrect
    //     ownerships result in instruction execution failure anyway.
    let buf = &mut &mut buffer_account.data.borrow_mut()[..];
    protocol::program::response(&req).serialize(buf)?;
    Ok(())
}

entrypoint!(process_instruction);
