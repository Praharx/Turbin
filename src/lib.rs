#[cfg(test)] 
mod tests { 

    use bs58;
    use std::io::{self, BufRead};
    use solana_sdk; 
    use solana_sdk::{ message::Message, signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
    use solana_program::{ pubkey::Pubkey, system_instruction::transfer};
    use solana_client::rpc_client::RpcClient;
    use std::str::FromStr;

    const RPC_URL:&str = "https://api.devnet.solana.com"; 


    #[test] 
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet address: {}", kp.pubkey().to_string());println!("");
        println!("Newly generated address:");
        println!("{:?}", kp.to_bytes());
    } 
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58.");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}",wallet);
    }
    #[test]
    fn wallet_to_base58() {
        println!("Input key as a wallet byte array:");
        let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',').map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();println!("{:?}",base58);
    }
    #[test] 
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldnt find a wallet");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success, your txn is here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }

            Err(e) => println!("Oops, something went wrong: {}",e.to_string())
        }
    } 
    #[test] 
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldnt find a wallet");
        let to_pubkey = Pubkey::from_str("ExyE8BT5a3LYZjymM2XsZUNhTWLj1xjKv9BecfY7J5xd").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get the recent blockhash");
        let balance = rpc_client 
        .get_balance(&keypair.pubkey()) 
        .expect("Failed to get balance"); 
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),  &recent_blockhash
        );
        // Calculating exact fee rate to transfer entire SOL amount out of account
        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to calculate the fee");
        // Deducting the feefrom lamports amount TX with correct balance
        let transaction = Transaction::new_signed_with_payer( &[transfer(
            &keypair.pubkey(),&to_pubkey, balance - fee,
        )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        println!("Success, your txn is here:");
        println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    } 
} 


    