use solana_sdk::signature::Signer;  // To access pubkey from keypair.

fn main() {
    let client_keypair_file_path = std::env::args().nth(1).unwrap();
    let program_keypair_file_path = std::env::args().nth(2).unwrap();
    let cluster_url = std::env::args().nth(3).unwrap();

    let client_keypair = solana_sdk::signer::keypair::read_keypair_file(
        client_keypair_file_path
    )
    .unwrap();

    // XXX We do not strictly need the program's keypair, but only its pubkey.
    //     However, during experimentation, the program's keys will likely
    //     change many times and so it is easier to simply lookup the pubkey
    //     from the keypair found in the default location:
    //     target/deploy/program-keypair.json
    let program_keypair = solana_sdk::signer::keypair::read_keypair_file(
        program_keypair_file_path
    )
    .unwrap();

    let client_id = client_keypair.pubkey();
    let program_id = program_keypair.pubkey();
    let accounts = vec![];
    let instruction_data = &[0];
    eprintln!(">>> client_id: {:?}", client_id);
    eprintln!(">>> program_id: {:?}", program_id);
    eprintln!(">>> accounts: {:?}", accounts);
    eprintln!(">>> instruction_data: {:?}", instruction_data);

    let client = solana_client::rpc_client::RpcClient::new(cluster_url);
    let ix = solana_sdk::instruction::Instruction::new_with_bytes(
        program_id,
        instruction_data,
        accounts,
    );
    let tx = solana_sdk::transaction::Transaction::new(
        &[&client_keypair],
        solana_sdk::message::Message::new(&[ix], Some(&client_id)),
        client.get_latest_blockhash().unwrap(),
    );
    let signature = client.send_and_confirm_transaction(&tx).unwrap();
    eprintln!(">>> tx sent and confirmed. sig: {:?}", signature);
}
