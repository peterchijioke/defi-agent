
mod deposit;
mod borrow;
mod repay;

use borsh::{BorshDeserialize, BorshSerialize};
use  solana_program::{entrypoint, msg};
use solana_program::{
    program_error::ProgramError::InvalidInstructionData,
account_info::AccountInfo,entrypoint::ProgramResult,pubkey::Pubkey
};



entrypoint!(process_instruction);


#[derive(BorshSerialize,BorshDeserialize,Debug)]
enum Initializer {
    HandleDeposit(Vec<u8>),
    HandleBorrow(Vec<u8>),
    HandleRepay(Vec<u8>)
}

pub fn process_instruction(program_id:&Pubkey,accounts:&[AccountInfo],instruction_data:&[u8]) ->ProgramResult{


    let instruction = Initializer::try_from_slice(instruction_data).map_err(|error|{
        msg!("Program error {:?}",error);
        InvalidInstructionData
    })?;

    match instruction {
        Initializer::HandleDeposit(data)=>{
            deposit::process( accounts, &data)
        }
        Initializer::HandleBorrow(data)=>{
            borrow::process( accounts, &data)
        }
        Initializer::HandleRepay(data)=>{
            repay::process(accounts, &data)
        }
    }
}
