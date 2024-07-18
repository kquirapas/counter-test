use crate::instruction::{
    accounts::{Context, IncrementAccounts, InitializeAccounts},
    CounterTestInstruction,
};
use crate::{
    pda::{create_counter_pda, find_counter_pda},
    state::Counter,
};
use borsh::BorshDeserialize;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey,
    rent::Rent, system_instruction,
};

/// Program state processor
pub struct Processor {}

impl<'a> Processor {
    /// Process the transaction
    ///
    /// - Deserializes the instruction data
    /// - Routes transaction data to the proper handler
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // get instruction
        let instruction = CounterTestInstruction::try_from_slice(instruction_data)?;
        match instruction {
            CounterTestInstruction::Initialize => {
                process_initialize(program_id, InitializeAccounts::context(accounts)?)?;
            }

            CounterTestInstruction::Increment => {
                process_increment(program_id, IncrementAccounts::context(accounts)?)?;
            }
        }

        Ok(())
    }
}

/// Initialize the Counter PDA
///
/// - Create the Counter PDA
/// - Set Counter count to 0
/// - Set Counter authority
///
/// Accounts
/// 0. `[WRITE]`    `Counter` account PDA
/// 1. `[SIGNER]`   `Authority` authority, fee payer
///
/// Instruction Data:
/// - (None)
pub fn process_initialize(program_id: &Pubkey, ctx: Context<InitializeAccounts>) -> ProgramResult {
    // ensure authority is signer
    assert!(ctx.accounts.authority.is_signer);

    // derive counter PDA
    let (counter_pda, counter_canonical_bump) =
        find_counter_pda(program_id, ctx.accounts.authority.key);

    // ensure canonical bump was used for counter PDA
    assert_eq!(*ctx.accounts.counter.key, counter_pda);

    // retrieve counter account data slice
    let counter_data = ctx.accounts.counter.try_borrow_mut_data()?;

    // ensure counter data size is what we expect
    assert_eq!(counter_data.len(), Counter::LEN);

    // load rent sysvar directly from runtime
    let rent_sysvar = Rent::get()?;

    invoke_signed(
        &system_instruction::create_account(
            ctx.accounts.authority.key,
            ctx.accounts.counter.key,
            rent_sysvar.minimum_balance(Counter::LEN),
            Counter::LEN as u64,
            program_id,
        ),
        &[ctx.accounts.authority.clone(), ctx.accounts.counter.clone()],
        &[&[
            b"counter",
            ctx.accounts.authority.key.as_ref(),
            &[counter_canonical_bump],
        ]],
    )?;

    // initialize counter
    let mut counter_account = Counter::try_from_slice(&counter_data)?;
    counter_account.count = 0;
    counter_account.authority = *ctx.accounts.authority.key;
    counter_account.bump = counter_canonical_bump;

    Ok(())
}

/// Increment the Counter
///
/// - Add 1 to counter
///
/// Accounts
/// 0. `[WRITE]`    `Counter` account PDA
/// 1. `[SIGNER]`   `Authority` authority, fee payer
///
/// Instruction Data:
/// - (None)
pub fn process_increment(program_id: &Pubkey, ctx: Context<IncrementAccounts>) -> ProgramResult {
    // ensure authority is signer
    assert!(ctx.accounts.authority.is_signer);

    // retrieve counter account data slice
    let counter_data = ctx.accounts.counter.try_borrow_mut_data()?;

    // ensure counter data size is what we expect
    assert_eq!(counter_data.len(), Counter::LEN);

    let mut counter_account = Counter::try_from_slice(&counter_data)?;

    // derive counter PDA
    let counter_pda =
        create_counter_pda(program_id, ctx.accounts.authority.key, counter_account.bump);

    // ensure the right counter account for the right authority
    assert_eq!(*ctx.accounts.counter.key, counter_pda);

    // increment counter
    counter_account.count += 1;

    Ok(())
}
