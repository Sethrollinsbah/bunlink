use asterisk_ami::{AmiConnection, Packet};
use dotenv::dotenv;
use std::{env, collections::HashMap};
use std::error::Error;

pub async fn init() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let ami_server = env::var("AMI_SERVER").expect("AMI_SERVER should be set in .env");

    println!("  Server: http://{}", &ami_server);
    let mut ami = AmiConnection::connect(ami_server).await?;
    // Create a HashMap to hold action parameters
    let mut params = HashMap::new();
    params.insert("Action".to_string(), "Ping".to_string());

    // Create a Packet with the action and parameters
    let packet = Packet::new();

    // Send the Packet to the Asterisk server
    match ami.send(packet).await {
        Some(response) => {
            for pkt in response {
                println!("Response Packet : {:?}", pkt);
            }
        }
        None => {
            eprintln!("Failed to receive a response or connection error occurred.");
        }
    }
    Ok(())
}
