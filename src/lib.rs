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
    pub staking_reward: u8,
    pub ecosystem_reward: u8,
    pub airdrop: u8,
    pub private_sale: u8,
    pub pre_sale_1: u8,
    pub pre_sale_2: u8,
    pub strategic_investors: u8,
    pub ido: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Liquidity {
    pub centralize_exchange: u8,
    pub decentralized_exchange: u8,
}

pub struct VestingSchedule {
    pub team: Vec<(u64, u64)>,
    pub research: Vec<(u64, u64)>,
    pub marketing: Vec<(u64, u64)>,
    pub partners: Vec<(u64, u64)>,
    pub staking_reward: Vec<(u64, u64)>,
    pub ecosystem_reward: Vec<(u64, u64)>,
    pub airdrop: Vec<(u64, u64)>,
    pub private_sale: Vec<(u64, u64)>,
    pub pre_sale_1: Vec<(u64, u64)>,
    pub pre_sale_2: Vec<(u64, u64)>,
    pub strategic_investors: Vec<(u64, u64)>,
    pub ido: Vec<(u64, u64)>,
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
