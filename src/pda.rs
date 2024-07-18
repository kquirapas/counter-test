use solana_program::pubkey::Pubkey;

pub fn create_counter_pda(program_id: &Pubkey, authority: &Pubkey, bump: u8) -> Pubkey {
    // safe to unwrap if it fails something is
    // very very wrong
    Pubkey::create_program_address(&[b"counter", authority.as_ref(), &[bump]], program_id).unwrap()
}

pub fn find_counter_pda(program_id: &Pubkey, authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"counter", authority.as_ref()], program_id)
}
