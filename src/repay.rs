use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction::transfer;

pub fn process(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());

    let account_info_iter = &mut accounts.iter();
    let user_account = next_account_info(account_info_iter)?;
    let protocol_vault = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?; 
    let authority_account = next_account_info(account_info_iter)?; // Program's PDA authority

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let transfer_instruction = transfer(
        token_program.key,
        user_account.key,
        protocol_vault.key,
        user_account.key,
        &[],
        amount,
    )?;

    invoke_signed(
        &transfer_instruction,
        &[
            user_account.clone(),
            protocol_vault.clone(),
            token_program.clone(),
        ],
        &[],
    )?;

    Ok(())
}
