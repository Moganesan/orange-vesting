use std::{convert::TryInto, task::Context, vec};

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
    pub strategic_investors: u8,
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

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo],
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
    _team_whitelist: &[AccountInfo<'_>], // Change to slice
    _reseach_whitelist: &[AccountInfo],
    _marketing_whitelist: &[AccountInfo],
    _partners_whitelist: &[AccountInfo],
    _staking_reward_whitelist: &[AccountInfo],
    _ecosystem_reward_whitelist: &[AccountInfo],
    _airdrop_whitelist: &[AccountInfo],
    _private_sale_white_list: &[AccountInfo],
    _pre_sale_1_whitelist: &[AccountInfo],
    _pre_sale_2_whitelist: &[AccountInfo],
    _investors_whitelist: &[AccountInfo],
) -> ProgramResult {
    let mut team_whitelist: Vec<AccountInfo> = Vec::new();
    team_whitelist.extend_from_slice(&_team_whitelist);

    let mut research_whitelist: Vec<AccountInfo> = Vec::new();
    research_whitelist.extend_from_slice(&_reseach_whitelist);

    let mut marketing_whitelist: Vec<AccountInfo> = Vec::new();
    marketing_whitelist.extend_from_slice(&_marketing_whitelist);

    let mut partners_whitelist: Vec<AccountInfo> = Vec::new();
    partners_whitelist.extend_from_slice(&_partners_whitelist);

    let mut staking_whitelist: Vec<AccountInfo> = Vec::new();
    staking_whitelist.extend_from_slice(&_staking_reward_whitelist);

    let mut ecosystem_reward_whitelist: Vec<AccountInfo> = Vec::new();
    ecosystem_reward_whitelist.extend_from_slice(&_ecosystem_reward_whitelist);

    let airdrop_whitelist: Vec<AccountInfo> = Vec::new();
    airdrop_whitelist.extend_from_slice(&_airdrop_whitelist);

    let private_sale_whitelist: Vec<AccountInfo> = Vec::new();
    private_sale_whitelist.extend_from_slice(&_private_sale_white_list);

    let pre_sale_1_whitelist: Vec<AccountInfo> = Vec::new();
    pre_sale_1_whitelist.extend_from_slice(&pre_sale_1_whitelist);

    let pre_sale_2_whitelist: Vec<AccountInfo> = Vec::new();
    pre_sale_1_whitelist.extend_from_slice(&_pre_sale_1_whitelist);

    let investors_whitelist: Vec<AccountInfo> = Vec::new();
    investors_whitelist.extend_from_slice(&_investors_whitelist);

    let token_allocation = TokenAllocation {
        team: 8,
        research: 7,
        liquidity: Liquidity {
            centralize_exchange: 15,
            decentralized_exchange: 6,
        },
        marketing: 5,
        partners: 4,
        staking_reward: 10,
        ecosystem_reward: 7,
        airdrop: 4,
        private_sale: 6,
        pre_sale_1: 7,
        pre_sale_2: 8,
        strategic_investors: 4,
        ido: 9,
    };

    let seconds_per_year = SECONDS_PER_DAY;
    let vesting_schedule: VestingSchedule = VestingSchedule {
        team: Clock::get()?.unix_timestamp + 90 * seconds_per_year as i64,
        ecosystem_reward: Clock::get()?.unix_timestamp + 300 * seconds_per_year as i64,
    };

    if _instruction_data.is_empty() {
        // Handle empty instruction data
        return Err(ProgramError::InvalidInstructionData);
    }

    msg!("Token Vesting Contract");
    Ok(())
}
