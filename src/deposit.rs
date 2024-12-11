
use solana_program::{
account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, program::invoke, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey
};
use spl_token::state::Account as TokenAccount;

pub fn process(
    accounts: &[AccountInfo],
    data: &[u8],
)->ProgramResult {
    if data.len() < 9 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());

    let account_info_iter = &mut accounts.iter();
    let user_wallet = next_account_info(account_info_iter)?; // User's SPL Token Account
    let vault_account = next_account_info(account_info_iter)?; // Protocol's SPL Token Vault
    let token_program = next_account_info(account_info_iter)?; // SPL Token Program
    let authority_account = next_account_info(account_info_iter)?; // Program Authority (PDA)

    // Verify account ownership and signers
    if !user_wallet.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let user_token_account = TokenAccount::unpack(&user_wallet.try_borrow_data()?)?;
    let _vault_token_account = TokenAccount::unpack(&vault_account.try_borrow_data()?)?;

    // Check token account ownership
    if user_token_account.owner != *authority_account.key {
        return Err(ProgramError::IllegalOwner);
    }

    if user_token_account.amount < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    let transfer_instruction = spl_token::instruction::transfer(
        token_program.key,
        user_wallet.key,
        vault_account.key,
        user_wallet.key,
        &[],
        amount,
    )?;

    invoke(
        &transfer_instruction,
        &[
            user_wallet.clone(),
            vault_account.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}
