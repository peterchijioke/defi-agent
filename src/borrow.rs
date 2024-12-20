
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process(
    accounts: &[AccountInfo],
    data: &[u8],
)->ProgramResult {
  if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());

    let account_info_iter = &mut accounts.iter();
    let collateral_account = next_account_info(account_info_iter)?;
    let borrow_account = next_account_info(account_info_iter)?;
    let protocol_vault = next_account_info(account_info_iter)?;
    let _protocol_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let authority_account = next_account_info(account_info_iter)?;

    if !collateral_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let max_borrow = 1_000_000;
    if amount > max_borrow {
        return Err(ProgramError::Custom(1));
    }

    let transfer_instruction = spl_token::instruction::transfer(
        token_program.key,
        protocol_vault.key,
        borrow_account.key,
        authority_account.key,
        &[],
        amount,
    )?;

    invoke_signed(
        &transfer_instruction,
        &[
            protocol_vault.clone(),
            borrow_account.clone(),
            token_program.clone(),
            authority_account.clone(),
        ],
        &[],
    )?;

    Ok(())
}
