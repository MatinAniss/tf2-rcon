use async_std::io::{self};
use colored::Colorize;
use rcon::Connection;

const RCON_ADDRESS: &str = "localhost:42465";
const RCON_PASSWORD: &str = "password";

#[async_std::main]
async fn main() {
    println!(
        "{} {}",
        "Attempting to connect to TF2 RCON server:".yellow(),
        &RCON_ADDRESS
    );

    let conn = Connection::builder()
        .connect(RCON_ADDRESS, RCON_PASSWORD)
        .await;

    match conn {
        Ok(mut conn) => {
            println!("{}", "Successfully connected to TF2 RCON server".green());

            loop {
                let mut input = String::new();

                match io::stdin().read_line(&mut input).await {
                    Ok(_) => {
                        let trimmed_input = input.trim();

                        if !trimmed_input.is_empty() {
                            if let Err(err) = conn.cmd(&input).await {
                                eprintln!(
                                    "{} {}",
                                    "Failed to send command to TF2 RCON server:".red(),
                                    err
                                );
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("{} {}", "Failed to read from stdin:".red(), err);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("{} {}", "Failed to connect to TF2 RCON server:".red(), err);
        }
    }
}
