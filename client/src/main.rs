use rand::Rng; // access gen()

use borsh::BorshDeserialize;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signature::Signer; // To access pubkey from keypair.

fn keys_gen() -> solana_sdk::signature::Keypair {
    let seed = rand::thread_rng().gen::<[u8; 32]>();
    solana_sdk::signer::keypair::keypair_from_seed(&seed).unwrap()
}

fn airdrop(
    client: &solana_client::rpc_client::RpcClient,
    dst: &solana_sdk::pubkey::Pubkey,
    amount: u64,
) {
    let balance_init = client.get_balance(&dst).unwrap();
    let balance_target = balance_init + amount;

    eprint!("airdrop requesting");
    client.request_airdrop(&dst, amount).unwrap();
    eprintln!(".");

    eprint!("airdrop confirming ");
    let mut backoff = 1;
    while client.get_balance(&dst).unwrap() < balance_target {
        eprint!(".");
        std::thread::sleep(std::time::Duration::from_secs(backoff));
        backoff *= 2;
    }
    eprintln!(".");
    eprintln!("airdrop done");
}

fn account_create(
    client: &solana_client::rpc_client::RpcClient,
    payer_keys: &solana_sdk::signer::keypair::Keypair,
    account_keys: &solana_sdk::signer::keypair::Keypair,
    account_owner: &solana_sdk::pubkey::Pubkey, // Who has write access?
    account_data_len: usize, // How much buffer space to allocate?
) {
    let ix = solana_program::system_instruction::create_account(
        &payer_keys.pubkey(),
        &account_keys.pubkey(),
        client
            .get_minimum_balance_for_rent_exemption(account_data_len)
            .unwrap(),
        account_data_len as u64,
        account_owner,
    );
    let signers = [payer_keys, account_keys];
    let tx = solana_sdk::transaction::Transaction::new(
        &signers,
        solana_sdk::message::Message::new(&[ix], Some(&payer_keys.pubkey())),
        client.get_latest_blockhash().unwrap(),
    );
    let _ = client.send_and_confirm_transaction(&tx).unwrap();
}

fn play_ping_pong(
    client: &solana_client::rpc_client::RpcClient,
    program_id: solana_sdk::pubkey::Pubkey,
    buf_id: solana_sdk::pubkey::Pubkey,
    payer_keys: &solana_sdk::signer::keypair::Keypair,
) {
    let tx = |req: &protocol::client::Request| {
        let ix = solana_sdk::instruction::Instruction::new_with_borsh(
            program_id,
            req,
            {
                let is_signer = false; // Buffer doesn't need to sign the tx.
                vec![AccountMeta::new(buf_id, is_signer)]
            },
        );
        let signers = [payer_keys];
        let payer_id = payer_keys.pubkey();
        solana_sdk::transaction::Transaction::new(
            &signers,
            solana_sdk::message::Message::new(&[ix], Some(&payer_id)),
            client.get_latest_blockhash().unwrap(),
        )
    };
    let mut req = protocol::client::request_init();
    loop {
        eprint!("{:?} >", req);
        let _ = client.send_and_confirm_transaction(&tx(&req)).unwrap();
        let buf = client.get_account(&buf_id).unwrap().data;
        let resp = protocol::program::Response::try_from_slice(&buf).unwrap();
        eprintln!(" < {:?}", resp);
        req = protocol::client::request_next(&resp);
    }
}

fn main() {
    let program_keypair_file_path = std::env::args().nth(1).unwrap();
    let cluster_url = std::env::args().nth(2).unwrap();

    // XXX We do not strictly need the program's keypair, but only its pubkey.
    //     However, during experimentation, the program's keys will likely
    //     change many times and so it is easier to simply lookup the pubkey
    //     from the keypair found in the default location:
    //     target/deploy/program-keypair.json
    let program_id = solana_sdk::signer::keypair::read_keypair_file(
        program_keypair_file_path,
    )
    .unwrap()
    .pubkey();

    let payer_keys = keys_gen();
    let buf_keys = keys_gen();

    let client = solana_client::rpc_client::RpcClient::new(cluster_url);

    {
        // Payer could be a CLI arg, but it doesn't really matter for this
        // purpose, so generating a new one is simplest. As a bonus, this also
        // illustrates the point that a payer doesn't need an account, but only
        // a ledger entry with sufficient funds.
        // XXX airdrop caps:
        //     - devnet  : 2 sol
        //     - testnet : 1 sol
        let sol = 1;
        let lamports = sol * 1_000_000_000;
        airdrop(&client, &payer_keys.pubkey(), lamports);
    }

    {
        // Buffer account, which:
        // - program writes to;
        // - client reads from.
        // XXX Buffer account's "data" field MUST be allocated PRECISELY to the
        //     size of the data which we expect to write and read. Any
        //     deviation and borsh deserialization will fail.
        eprintln!("buffer account creating");
        let data_len = protocol::program::response_data_size();
        account_create(&client, &payer_keys, &buf_keys, &program_id, data_len);
        eprintln!("buffer account done");
    }

    play_ping_pong(&client, program_id, buf_keys.pubkey(), &payer_keys);
}
