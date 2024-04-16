use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenAllocation {
    pub team: u8,
    pub research: u8,
    pub liquidity: Liquidity,
    pub marketing: u8,
    pub partners: u8,
    pub stakingReward: u8,
    pub ecosystemReward: u8,
    pub airdrop: u8,
    pub privateSale: u8,
    pub preSale1: u8,
    pub preSale2: u8,
    pub strategicInvestors: u8,
    pub ido: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Liquidity {
    pub centralizeExchange: u8,
    pub decentralizedExchange: u8,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Token Vesting Contract");
    Ok(())
}
