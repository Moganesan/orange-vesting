use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
};
use solana_sdk::clock;
use solana_sdk::{
    account::{self, Account},
    clock::{Clock, SECONDS_PER_DAY},
    sysvar::Sysvar,
};
use spl_token::instruction::transfer;
use spl_token::state::Account as TokenAccount;

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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenClaim {
    pub total_allocation: u64,
    pub claimed_token: u64,
}

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

    let total_supply: u64;
    let mut team_allocation_per_wallet: u64;

    if _instruction_data[0] == 0 {
        return claim_team_tokens(
            program_id,
            accounts,
            &_instruction_data[1.._instruction_data.len()],
            &team_whitelist,
            &token_allocation.team,
            vesting_schedule.team,
        );
    }

    Ok(())
}

fn claim_team_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
    team_whitelist: &Vec<Pubkey>,
    token_allocation: &u8,
    vesting_schedule: i64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let writing_account = next_account_info(accounts_iter)?;
    let token_program_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let receiver_account = next_account_info(accounts_iter)?;

    if team_whitelist
        .iter()
        .any(|account| account != receiver_account.key)
    {
        msg!("The account is't found in the whitelist.");
        return Err(ProgramError::Custom(1));
    }

    if writing_account.owner != program_id {
        msg!("writing_account is't owned by the program.");
        return Err(ProgramError::IncorrectProgramId);
    }

    if Clock::get()?.unix_timestamp < vesting_schedule {
        msg!("Team token hasn't reached the realease period.");
        return Err(ProgramError::Custom(1));
    }

    Ok(())
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
