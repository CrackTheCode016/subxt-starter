#![allow(missing_docs)]
use crate::{ config::{ SubXtResult, paseo } };

mod remark;
mod config;

#[tokio::main]
async fn main() -> SubXtResult<()> {
    use crate::remark::{ fetch_account_info, remark, create_signer };
    use subxt_signer::sr25519::dev;

    // Fetch account info for Alice
    let alice = dev::alice();
    let address = alice.public_key().to_account_id();
    let info = fetch_account_info(address).await?;
    println!("Account info for ALICE: {:?}", info);

    // Create a signer from a mnemonic (this is just Alice; replace with your own account's phrase!)
    let mnemonic = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    let signer = create_signer(mnemonic)?;
    println!("Signer SS58: {:?}", signer.public_key().to_account_id().to_string());

    // Send a remark and watch for the event
    let events = remark(&alice, "Hello from remark.rs!").await?;
    // Note: this just finds the first "remark_with_event" extrinsic
    let remark_event = events.find_first::<paseo::system::events::Remarked>()?;
    if let Some(event) = remark_event {
        println!("Remark success: {event:?}");
    } else {
        println!("No Remarked event found.");
    }

    Ok(())
}
