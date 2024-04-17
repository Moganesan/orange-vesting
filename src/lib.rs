use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_sdk::{
    account::{self, Account},
    clock::{Clock, SECONDS_PER_DAY},
    sysvar::Sysvar,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenAllocation {
    pub team: u8,
    pub research: u8,
    pub marketing: u8,
    pub partners: u8,
    pub staking_reward: u8,
    pub ecosystem_reward: u8,
    pub airdrop: u8,
    pub private_sale: u8,
    pub pre_sale_1: u8,
    pub pre_sale_2: u8,
    pub investors: u8,
    pub ido: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Liquidity {
    pub centralize_exchange: u8,
    pub decentralized_exchange: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VestingSchedule {
    pub team: i64,
    pub ecosystem_reward: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Whitelist {
    pub team: Vec<Pubkey>,
    pub research: Vec<Pubkey>,
    pub marketing: Vec<Pubkey>,
    pub partners: Vec<Pubkey>,
    pub staking_reward: Vec<Pubkey>,
    pub ecosystem_reward: Vec<Pubkey>,
    pub airdrop: Vec<Pubkey>,
    pub private_sale: Vec<Pubkey>,
    pub investors: Vec<Pubkey>,
}
// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo],
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;

    if writing_account.owner != program_id {
        msg!("writing_account isn't owned by the program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if _instruction_data.is_empty() {
        // Handle empty instruction data
        return Err(ProgramError::InvalidInstructionData);
    }

    if _instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let whitelist = match Whitelist::try_from_slice(_instruction_data) {
        Ok(whitelist) => whitelist,
        Err(_) => return Err(ProgramError::InvalidInstructionData),
    };

    let mut team_whitelist: &Vec<Pubkey> = &whitelist.team;

    let mut research_whitelis: &Vec<Pubkey> = &whitelist.research;

    let mut marketing_whitelist: &Vec<Pubkey> = &whitelist.marketing;

    let mut partners_whitelist: &Vec<Pubkey> = &whitelist.partners;

    let mut staking_reward_whitelist: &Vec<Pubkey> = &whitelist.staking_reward;

    let mut ecosystem_reward_whitelist: &Vec<Pubkey> = &whitelist.ecosystem_reward;

    let mut airdrop_whitelist: &Vec<Pubkey> = &whitelist.airdrop;

    let mut private_sale_whitelist: &Vec<Pubkey> = &whitelist.private_sale;

    let mut investors_whitelist: &Vec<Pubkey> = &whitelist.investors;

    let token_allocation = TokenAllocation {
        team: 8,
        research: 7,
        marketing: 5,
        partners: 4,
        staking_reward: 10,
        ecosystem_reward: 7,
        airdrop: 4,
        private_sale: 6,
        pre_sale_1: 7,
        pre_sale_2: 8,
        investors: 4,
        ido: 9,
    };

    let seconds_per_year = SECONDS_PER_DAY;
    let vesting_schedule: VestingSchedule = VestingSchedule {
        team: Clock::get()?.unix_timestamp + 90 * seconds_per_year as i64,
        ecosystem_reward: Clock::get()?.unix_timestamp + 300 * seconds_per_year as i64,
    };

    Ok(())
}
