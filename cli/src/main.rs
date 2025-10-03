use clap::{Parser, Subcommand};

use identity_core::commands::insert_user;
use identity_core::inputs::RegisterInput;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    CreateUser {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
        #[arg(short, long)]
        full_name: String,
        #[arg(short, long)]
        birthdate: String,
        #[arg(short, long)]
        country: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        CliCommand::CreateUser {
            username,
            email,
            password,
            full_name,
            birthdate,
            country,
        } => {
            let result = insert_user(&RegisterInput {
                username: username.clone(),
                email: email.clone(),
                password: password.clone(),
                full_name: full_name.clone(),
                birthdate: birthdate.clone(),
                country_alpha2: country.clone(),
            })
            .await;

            match result {
                Ok(_) => println!("User created successfully."),
                Err(err) => println!("Failed to create user.\n\n{err}"),
            }
        }
    }
}
