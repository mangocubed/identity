use clap::{Parser, Subcommand};

use identity_core::commands::{
    delete_application, get_all_applications, get_application_by_id, insert_application, insert_user,
};
use identity_core::inputs::{ApplicationInput, RegisterInput};
use identity_core::models::Application;
use uuid::Uuid;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    ApplicationsList,
    CreateApplication {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        redirect_url: String,
    },
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
    DeleteApplication {
        #[arg(short, long)]
        id: Uuid,
    },
}

fn print_application(application: &Application) {
    println!(
        "\nID: {}\nName: {}\nRedirect URL: {}\nCreated at: {}",
        application.id, application.name, application.redirect_url, application.created_at
    );
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        CliCommand::ApplicationsList => {
            let applications = get_all_applications().await;

            match applications {
                Ok(applications) => {
                    let applications_len = applications.len();

                    println!(
                        "{} application{} found.",
                        applications_len,
                        if applications_len != 1 { "s" } else { "" }
                    );

                    for application in applications {
                        print_application(&application);
                    }
                }
                Err(err) => println!("Failed to get applications.\n\n{err}"),
            }
        }
        CliCommand::CreateApplication { name, redirect_url } => {
            let result = insert_application(&ApplicationInput {
                name: name.clone(),
                redirect_url: redirect_url.clone(),
            })
            .await;

            match result {
                Ok((application, secret)) => {
                    println!("Application created successfully.");
                    print_application(&application);
                    println!("\nSecret: {}", secret);
                }
                Err(err) => println!("Failed to create application.\n\n{err}"),
            }
        }
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
        CliCommand::DeleteApplication { id } => {
            let application = get_application_by_id(*id).await.expect("Could not get application");
            let result = delete_application(application).await;

            match result {
                Ok(_) => println!("Application deleted successfully."),
                Err(err) => println!("Failed to delete application.\n\n{err}"),
            }
        }
    }
}
