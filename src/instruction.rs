use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Debug, ShankContext, ShankInstruction)]
pub enum CounterTestInstruction {
    #[account(
        0,
        writable,
        name = "counter",
        desc = "Counter account. Seeds ['counter', `authority.key`]"
    )]
    #[account(1, name = "authority", desc = "Counter authority")]
    #[account(2, name = "rent_sysvar", desc = "Rent Sysvar")]
    #[account(3, name = "system_program", desc = "System Program")]
    Initialize,

    #[account(
        0,
        writable,
        name = "counter",
        desc = "Counter account. Seeds ['counter', `authority.key`]"
    )]
    #[account(1, name = "authority", desc = "Counter authority")]
    Increment,
}
