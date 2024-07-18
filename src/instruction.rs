use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Debug, ShankContext, ShankInstruction)]
pub enum CounterTestInstruction {
    /// Initialize the Counter PDA
    ///
    /// - Create the Counter PDA
    /// - Set Counter count to 0
    /// - Set Counter authority
    ///
    #[account(
        0,
        writable,
        name = "counter",
        desc = "Counter account. Seeds ['counter', `authority.key`]"
    )]
    #[account(1, signer, name = "authority", desc = "Counter authority")]
    Initialize,

    /// Increment the Counter
    ///
    /// - Add 1 to counter
    ///
    #[account(
        0,
        writable,
        name = "counter",
        desc = "Counter account. Seeds ['counter', `authority.key`]"
    )]
    #[account(1, signer, name = "authority", desc = "Counter authority")]
    Increment,
}
