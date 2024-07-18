#![cfg(feature = "test-sbf")]

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use borsh::{BorshDeserialize, BorshSerialize};
    use spl_token::{
        id, instruction,
        state::{Account, Mint},
        ID,
    };
    use token_sale::{
        entrypoint,
        instruction::TokenSaleInstruction,
        state::{pda::find_token_base_pda, TokenBase},
    };
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
    async fn test_open_sale() {
        solana_logger::setup_with_default("solana_program::message=debug");

        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("token_sale", program_id, None);
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // create mint
        let mint = Keypair::new();
        let rent = banks_client.get_rent().await.unwrap();
        let decimals = 0;

        // Setup the mint
        let transaction = Transaction::new_signed_with_payer(
            &[
                system_instruction::create_account(
                    &payer.pubkey(),
                    &mint.pubkey(),
                    rent.minimum_balance(Mint::LEN),
                    Mint::LEN as u64,
                    &spl_token::id(),
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    &mint.pubkey(),
                    &payer.pubkey(),
                    None,
                    decimals,
                )
                .unwrap(),
            ],
            Some(&payer.pubkey()),
            &[&payer, &mint],
            recent_blockhash,
        );
        banks_client.process_transaction(transaction).await.unwrap();

        // create token_base
        let (token_base_pda, _) = find_token_base_pda(&program_id, &payer.pubkey(), &mint.pubkey());

        let vault = Keypair::new();

        let price = 100000000000;
        let default_purchase_limit = 100;
        let instruction = TokenSaleInstruction::OpenSale {
            price,
            purchase_limit: default_purchase_limit,
            whitelist_root: [0u8; 32],
        };

        let mut instruction_data = Vec::new();
        instruction.serialize(&mut instruction_data).unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(token_base_pda, false),
                    AccountMeta::new_readonly(mint.pubkey(), false),
                    AccountMeta::new_readonly(vault.pubkey(), false),
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new_readonly(RENT_SYSVAR_ID, false),
                    AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
                ],
                data: instruction_data,
            }],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // instruction went through
        assert_eq!(true, true);
    }
}
