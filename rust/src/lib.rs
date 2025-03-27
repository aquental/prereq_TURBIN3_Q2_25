mod programs;

pub const TURBIN3_WALLET: &str = "turbin3-wallet.json";
pub const TURBIN3_PUB_KEY: &str = "Dn2ucNUVe5ptVueYRKf6m6effxs13RJpjJEMfEL9yMzG";
pub const DEV_WALLET: &str = "dev-wallet.json";
pub const GITHUB_USER: &[u8; 8] = b"aquental";
pub const RPC_DEVNET: &str = "https://api.devnet.solana.com";
//pub const RPC_DEVNET: &str = "https://polished-warmhearted-frog.solana-devnet.quiknode.pro/9bc0c3437243817577c59c3690d3bcde03fe8b6f/";
pub const PREREQ_SEED: &[u8; 6] = b"prereq";

#[cfg(test)]
mod tests {
    use crate::{
        DEV_WALLET, PUB_KEY, RPC_DEVNET,
        programs::Turbin3_prereq::{CompleteArgs, TurbinePrereqProgram, UpdateArgs},
    };
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{Keypair, Signer, read_keypair_file},
        system_program,
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    const RPC_URL: &str = RPC_DEVNET;

    /// Generate a new Solana keypair, print it to the console and output the corresponding JSON data.
    /// The user should copy and paste this into a JSON file to save the wallet.
    #[test]
    fn keygen() {
        // Let's create a new Keypair
        let kp = Keypair::new();
        // Print newly created keypair to the console
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("\nTo save your wallet, Copy and Paste the following into a JSON file!");
        println!("{:?}", kp.to_bytes());
    }

    /// Claims 2 SOL from the Solana Devnet faucet, given a path to a valid keypair file.
    /// Prints the TX signature to the console upon success.
    #[test]
    fn airdrop() {
        // let's read our keypair file
        let keypair = read_keypair_file(DEV_WALLET).expect("Couldn't find wallet file");
        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);
        // Now, we're gonna claim 2 SOL
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Airdrop success! Check out your TX here");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, Something went wrong: {}", e.to_string()),
        };
    }

    /// Transfer all the SOL from the dev wallet to the Turbin3 wallet, minus the fee that will be
    /// incurred when sending the transaction. This is done to empty the devnet wallet of SOL.
    #[test]
    fn transfer_sol() {
        // Let's get our dev-wallet.json file
        let keypair = read_keypair_file(DEV_WALLET).expect("Couldn't find wallet file");
        // Define our Turbin3 public key
        let turbin3_pubkey = Pubkey::from_str(TURBIN3_PUB_KEY).unwrap();
        // Create a solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        // To sign transactions, we're gonna need recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        // We're gonna transfer 0.1 SOL from our dev wallet to Turbin3 wallet
        //let transaction = Transaction::new_signed_with_payer(&[transfer(&keypair.pubkey(), &turbin3_pubkey, 100_000_000)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);
        // Let's submit transaction and grab the tx Signature
        //let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        // Print and grab transaction signature on success
        //println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

        // ATTEMPTING TO EMPTY THE DEVNET WALLET
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        // creating test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &turbin3_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        // Now, we ask the Rpc client what the fee for this mock transaction gonna be
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        // Let's transact with lamports of : balance -fee
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &turbin3_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        // let's submit and grab the tx signature
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    /// Enrolls a user by creating a Solana Devnet connection and executing a transaction.
    /// The function reads the signer's keypair from a wallet file, derives the program address
    /// for the prerequisite account, and sets up the GitHub account using provided arguments.
    /// It retrieves the latest blockhash and invokes the `complete` function of the
    /// TurbinePrereqProgram to create the transaction.
    /// Finally, it sends and confirms the transaction, printing the transaction signature
    /// upon success.
    #[test]
    fn enroll() {
        // Let's create a Solana Devnet Connection
        let rpc_client = RpcClient::new(RPC_URL);
        // Let's define our accounts
        let signer = read_keypair_file(TURBIN3_WALLET).expect("Couldn't find wallet file");
        // Creating first PDA
        let prereq = TurbinePrereqProgram::derive_program_address(&[
            PREREQ_SEED,
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Let's set our github account
        let args = CompleteArgs {
            github: GITHUB_USER.to_vec(),
        };

        // Need Recent blockhash to publish our transaction
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Now, we need to invoke our complete function
        let transaction = TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // Let's publish our transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        // Print tx hash upon success
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

    /// Converts a user-provided base58 private key to a wallet file byte array.
    /// The program reads the user's private key as base58, decodes it, and prints
    /// the wallet file as a byte array.
    #[test]
    fn base58_to_wallet() {
        // Get User's private key as base58
        println!("Input your private key as base58: ");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!(" Your wallet file is: ");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    /// Converts a user-provided wallet file byte array to a base58 private key.
    /// The program reads the user's wallet file as a byte array, encodes it, and prints
    /// the base58 private key.
    #[test]
    fn wallet_to_base58() {
        // Let's Get user's private key as bytes array
        println!("Input your private key as a wallet file byte array: ");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(",")
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is: ");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
