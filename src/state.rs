use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount, SplDiscriminate)]
#[discriminator_hash_input("counter_test::state::counter")]
pub struct Counter {
    pub discriminator: [u8; 8],
    pub authority: Pubkey,
    pub count: u64,
}

impl Counter {
    /// Get known size of Counter
    pub const LEN: usize = std::mem::size_of::<Counter>();

    /// Is `true` if Counter is initialized
    pub fn is_initialized(&self) -> bool {
        self.discriminator.as_slice() == Counter::SPL_DISCRIMINATOR_SLICE
    }

    /// Is `true` if Counter is uninitialized
    pub fn is_uninitialized(&self) -> bool {
        self.discriminator.as_slice() == ArrayDiscriminator::UNINITIALIZED.as_slice()
    }
}
