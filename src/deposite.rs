
use solana_program::{
account_info::AccountInfo,entrypoint::ProgramResult,pubkey::Pubkey
};

pub fn process(
  program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
)->ProgramResult {
  Ok(())
}
