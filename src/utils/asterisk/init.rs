use asterisk_ami::{AmiConnection, Packet};
use dotenv::dotenv;
use std::{env, collections::HashMap};
use std::error::Error;

pub async fn init() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let ami_server = env::var("AMI_SERVER").expect("AMI_SERVER should be set in .env");
    let ami_username = env::var("AMI_USERNAME").expect("AMI_USERNAME should be set in .env");
    let ami_secret = env::var("AMI_SECRET").expect("AMI_SECRET should be set in .env");

    println!("Server: http://{}", &ami_server);
    let mut ami = AmiConnection::connect(ami_server).await?;

    // Login
    let mut login_params = HashMap::new();
    login_params.insert("Action".to_string(), "Login".to_string());
    login_params.insert("Username".to_string(), ami_username.to_string());
    login_params.insert("Secret".to_string(), ami_secret.to_string());
    let login_packet = Packet::from(login_params);

    let login_response = ami.send(login_packet).await;

    if let Some(responses) = login_response {
        let first_response = responses.get(0).unwrap();
        if first_response.get("Response").unwrap() != "Success" {
            eprintln!("Login failed: {:?}", first_response);
            return Err("AMI Login failed".into());
        } else {
            println!("Login successful");
        }
    } else {
        eprintln!("Login failed: No response");
        return Err("AMI Login failed: No response".into());
    }

    // Ping
    let mut ping_params = HashMap::new();
    ping_params.insert("Action".to_string(), "Ping".to_string());
    let ping_packet = Packet::from(ping_params);

    match ami.send(ping_packet).await {
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
