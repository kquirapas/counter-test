#![cfg(feature = "test-sbf")]

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use {
        assert_matches::*,
        // solana_program::{program_pack::Pack, rent::Rent, system_instruction},
        solana_program_test::*,
        solana_sdk::{
            instruction::{AccountMeta, Instruction},
            message::Message,
            msg,
            program_pack::Pack,
            pubkey::Pubkey,
            rent::Rent,
            signature::Keypair,
            signature::Signer,
            system_instruction,
            system_program::ID as SYSTEM_PROGRAM_ID,
            sysvar::rent::ID as RENT_SYSVAR_ID,
            transaction::Transaction,
        },
    };

    #[tokio::test]
    async fn test_sanity() {
        assert_eq!(true, true)
    }

    #[tokio::test]
    async fn test_initialize() {
        // show program logs when testing
        solana_logger::setup_with_default("solana_program::message=debug");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new(
            // .so fixture is  retrieved from /target/deploy
            "counter_test",
            program_id,
            // shank is incompatible with instantiating the BuiltInFunction
            None,
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // create counter
        let (counter_pda, counter_canonical_bump) =
            pda::find_counter_pda(&program_id, &payer.pubkey());

        // create Initialize instruction
        let initialize_ix = instruction::CounterTestInstruction::Initialize;
        let mut initialize_ix_data = Vec::new();
        initialize_ix.serialize(&mut initialize_ix_data).unwrap();

        // create transaction
        let transaction = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(counter_pda, false),
                    AccountMeta::new(payer.pubkey(), true),
                ],
                data: initialize_ix_data,
            }],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        // send tx
        banks_client.process_transaction(transaction).await.unwrap();

        // confirm state
        let counter_account_info = banks_client
            .get_account(counter_pda)
            .await
            .unwrap()
            .unwrap();

        let counter = state::Counter::try_from_slice(&counter_account_info.data).unwrap();

        // check right authority
        assert_eq!(counter.authority, payer.pubkey());
        // check counter is 0
        assert_eq!(counter.count, 0);
        // check canonical bump is stored
        assert_eq!(counter.bump, counter_canonical_bump);
    }
}
