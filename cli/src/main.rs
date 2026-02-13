use clap::{Parser, Subcommand};

use chrono::NaiveDate;
use identity_core::commands;
use identity_core::models::Application;
use identity_core::params::{ApplicationParams, UserParams};
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
        birthdate: NaiveDate,
        #[arg(short, long)]
        country_code: String,
    },
    DeleteApplication {
        #[arg(short, long)]
        id: Uuid,
    },
    UpdateApplication {
        #[arg(short, long)]
        id: Uuid,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        redirect_url: String,
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        CliCommand::ApplicationsList => {
            let applications = commands::all_applications().await;

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
        CliCommand::UpdateApplication { id, name, redirect_url } => {
            let application = commands::get_application_by_id(*id)
                .await
                .expect("Could not get application");
            let result = commands::update_application(
                &application,
                ApplicationParams {
                    name: name.clone(),
                    redirect_url: redirect_url.clone(),
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
