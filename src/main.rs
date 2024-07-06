mod wallet;

use wallet::Wallet;
use web3::transports::Http;
use web3::Web3;



#[tokio::main]
async fn main() {
    // Initialize the HTTP transport for connecting to the Ethereum network
    let http = Http::new("https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID").unwrap();
    // Create a new Web3 instance with the HTTP transport
    let web3 = Web3::new(http);

    // Load the wallet from the JSON file
    let wallet = Wallet::load_from_file("wallet.json");
    println!("Address: {}", wallet.address);

    // Fetch and display the wallet's balance
    let balance = wallet.get_balance(&web3).await;
    println!("Balance: {}", balance);

    // Define the recipient address and the amount to send (1 ETH in wei)
    let to_address = "0xRecipientAddressHere";
    let value = U256::from_dec_str("1000000000000000000").unwrap(); // 1 ETH in wei

    // Send the tokens and display the transaction hash
    let tx_hash = wallet.send_token(&web3, to_address, value).await.unwrap();
    println!("Transaction sent with hash: {:?}", tx_hash);
}

