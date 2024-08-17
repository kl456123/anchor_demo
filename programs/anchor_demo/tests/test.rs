#![cfg(feature = "test-sbf")]

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};
use solana_sdk::{
    pubkey::Pubkey,
    signature::Signer,
    signer::keypair::Keypair,
    transaction::{Transaction, TransactionError},
};

pub fn fixed_entry(
    program_id: &Pubkey,
    accounts: &[anchor_lang::prelude::AccountInfo],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    let extended_lifetime_accs =
        unsafe { core::mem::transmute::<_, &[anchor_lang::prelude::AccountInfo<'_>]>(accounts) };
    anchor_demo::entry(program_id, extended_lifetime_accs, data)
}

#[tokio::test]
async fn test_initialize() {
    let program_test = ProgramTest::new("anchor_demo", anchor_demo::id(), processor!(fixed_entry));
    let mut context = program_test.start_with_context().await;

    let num: u64 = 10;
    let new_account_kp = Keypair::new();
    // check before tx
    assert!(context
        .banks_client
        .get_account(new_account_kp.pubkey())
        .await
        .unwrap()
        .is_none());

    let accounts = anchor_demo::accounts::Initialize {
        new_account: new_account_kp.pubkey(),
        signer: context.payer.pubkey(),
        system_program: System::id(),
    };
    let instruction = anchor_demo::instruction::Initialize { num };

    let instruction = anchor_lang::solana_program::instruction::Instruction {
        program_id: anchor_demo::id(),
        data: anchor_lang::InstructionData::data(&instruction),
        accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
    };

    // let tx = Transaction::new_signed_with_payer(
    // &[instruction],
    // Some(&context.payer.pubkey()),
    // &[&context.payer, &new_account_kp],
    // context.last_blockhash,
    // );
    // context.banks_client.process_transaction(tx).await.unwrap();
    // context.banks_client.process_transaction(&vec![instruction], Some(&signers[..])).await.unwrap();
    let all_signers = vec![&context.payer, &new_account_kp];
    let mut transaction =
        Transaction::new_with_payer(&vec![instruction], Some(&context.payer.pubkey()));
    transaction.sign(&all_signers, context.last_blockhash);
    context
        .banks_client
        .process_transaction_with_commitment(
            transaction,
            solana_sdk::commitment_config::CommitmentLevel::Processed,
        )
        .await;

    // check data
    let new_account = context
        .banks_client
        .get_account(new_account_kp.pubkey())
        .await
        .unwrap()
        .unwrap();
    let new_account_data =
        anchor_demo::NewAccount::try_deserialize(&mut new_account.data.as_ref()).unwrap();
    assert_eq!(new_account_data.num, num);
}
