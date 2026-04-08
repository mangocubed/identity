use clap::{Parser, Subcommand};

use chrono::NaiveDate;
use identity_core::commands;
use identity_core::models::{Application, ApplicationToken};
use identity_core::params::{ApplicationParams, ApplicationTokenParams, UserParams};
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
    ApplicationTokensList {
        #[arg(short, long)]
        application_id: Uuid,
    },
    CreateApplication {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        redirect_url: String,
    },
    CreateApplicationToken {
        #[arg(short, long)]
        application_id: Uuid,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        expires_at: Option<NaiveDate>,
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
        birthdate: NaiveDate,
        #[arg(short, long)]
        country_code: String,
    },
    DeleteApplication {
        #[arg(short, long)]
        id: Uuid,
    },
    RevoveApplicationToken {
        #[arg(short, long)]
        id: Uuid,
    },
    UpdateApplication {
        #[arg(short, long)]
        id: Uuid,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        redirect_url: Option<String>,
    },
}

fn print_application(application: &Application) {
    println!(
        "\nID: {}\nName: {}\nRedirect URL: {}\nCreated at: {}\nUpdated at: {}",
        application.id,
        application.name,
        application.redirect_url,
        application.created_at,
        application
            .updated_at
            .map(|updated_at| updated_at.to_string())
            .unwrap_or_else(|| "None".to_owned())
    );
}

fn print_application_token(application_token: &ApplicationToken) {
    println!(
        "\nID: {}\nApplication ID: {}\nName: {}\nCode: {}\nExpires At: {}\nCreated at: {}\nUpdated at: {}",
        application_token.id,
        application_token.application_id,
        application_token.name,
        application_token.code,
        application_token.expires_at,
        application_token.created_at,
        application_token
            .updated_at
            .map(|updated_at| updated_at.to_string())
            .unwrap_or_else(|| "None".to_owned())
    );
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        CliCommand::ApplicationsList => {
            let result = commands::all_applications().await;

            match result {
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
        CliCommand::ApplicationTokensList { application_id } => {
            let application = commands::get_application_by_id(*application_id)
                .await
                .expect("Could not get application");

            let result = commands::all_application_tokens(&application).await;

            match result {
                Ok(application_tokens) => {
                    let application_tokens_len = application_tokens.len();

                    println!(
                        "{} application token{} found.",
                        application_tokens_len,
                        if application_tokens_len != 1 { "s" } else { "" }
                    );

                    for application_token in application_tokens {
                        print_application_token(&application_token);
                    }
                }
                Err(err) => println!("Failed to get application_tokens.\n\n{err}"),
            }
        }
        CliCommand::CreateApplication { name, redirect_url } => {
            let result = commands::insert_application(ApplicationParams {
                name: name.clone(),
                redirect_url: redirect_url.clone(),
            })
            .await;

            match result {
                Ok(application) => {
                    println!("Application created successfully.");
                    print_application(&application);
                }
                Err(err) => println!("Failed to create application.\n\n{err}"),
            }
        }
        CliCommand::CreateApplicationToken {
            application_id,
            name,
            expires_at,
        } => {
            let application = commands::get_application_by_id(*application_id)
                .await
                .expect("Could not get application");

            let result = commands::insert_application_token(
                &application,
                ApplicationTokenParams {
                    name: name.clone(),
                    expires_at: *expires_at,
                },
            )
            .await;

            match result {
                Ok(application_token) => {
                    println!("Application token created successfully.");
                    print_application_token(&application_token);
                }
                Err(err) => println!("Failed to create application token.\n\n{err}"),
            }
        }
        CliCommand::CreateUser {
            username,
            email,
            password,
            full_name,
            birthdate,
            country_code,
        } => {
            let result = commands::insert_user(UserParams {
                username: username.clone(),
                email: email.clone(),
                password: password.clone(),
                full_name: full_name.clone(),
                birthdate: Some(*birthdate),
                country_code: country_code.clone(),
            })
            .await;

            match result {
                Ok(_) => println!("User created successfully."),
                Err(err) => println!("Failed to create user.\n\n{err}"),
            }
        }
        CliCommand::DeleteApplication { id } => {
            let application = commands::get_application_by_id(*id)
                .await
                .expect("Could not get application");
            let result = commands::delete_application(application).await;

            match result {
                Ok(_) => println!("Application deleted successfully."),
                Err(err) => println!("Failed to delete application.\n\n{err}"),
            }
        }
        CliCommand::RevoveApplicationToken { id } => {
            let application_token = commands::get_application_token_by_id(*id)
                .await
                .expect("Could not get application token");
            let result = commands::revoke_application_token(&application_token).await;

            match result {
                Ok(_) => println!("Application token revoked successfully."),
                Err(err) => println!("Failed to revoke application token.\n\n{err}"),
            }
        }
        CliCommand::UpdateApplication { id, name, redirect_url } => {
            if name.is_none() && redirect_url.is_none() {
                println!("No changes to update.");
                return;
            }

            let application = commands::get_application_by_id(*id)
                .await
                .expect("Could not get application");
            let result = commands::update_application(
                &application,
                ApplicationParams {
                    name: name.clone().unwrap_or(application.name.to_string()),
                    redirect_url: redirect_url.clone().unwrap_or(application.redirect_url.to_string()),
                },
            )
            .await;

            match result {
                Ok(application) => {
                    println!("Application updated successfully.");
                    print_application(&application);
                }
                Err(err) => println!("Failed to update application.\n\n{err}"),
            }
        }
    }
}
