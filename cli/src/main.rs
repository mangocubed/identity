use clap::{Parser, Subcommand};

use identity_core::commands;
use identity_core::inputs::RegisterInput;
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
        #[arg(short, long, required = false)]
        webhook_url: Option<String>,
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
    UpdateApplication {
        #[arg(short, long)]
        id: Uuid,
        #[arg(short, long, required = false)]
        redirect_url: Option<String>,
        #[arg(short, long, required = false, default_missing_value = "", num_args = 0..=1)]
        webhook_url: Option<String>,
    },
    UpdateApplicationSecret {
        #[arg(short, long)]
        id: Uuid,
    },
    UpdateApplicationWebhookSecret {
        #[arg(short, long)]
        id: Uuid,
    },
}

fn print_application(application: &Application) {
    println!(
        "\nID: {}\nName: {}\nRedirect URL: {}\nWebhook URL: {}\nCreated at: {}\nUpdated at: {}\n",
        application.id,
        application.name,
        application.redirect_url,
        application.webhook_url.clone().unwrap_or_else(|| "None".to_owned()),
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
        CliCommand::CreateApplication {
            name,
            redirect_url,
            webhook_url,
        } => {
            let result = commands::insert_application(name, redirect_url, webhook_url.as_deref()).await;

            match result {
                Ok((application, secret)) => {
                    println!("Application created successfully.");
                    print_application(&application);
                    println!("\nSecret: {}", secret);
                    println!("\nWebhook Secret: {}", application.webhook_secret);
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
            let result = commands::insert_user(&RegisterInput {
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
            let application = commands::get_application_by_id(*id)
                .await
                .expect("Could not get application");
            let result = commands::delete_application(application).await;

            match result {
                Ok(_) => println!("Application deleted successfully."),
                Err(err) => println!("Failed to delete application.\n\n{err}"),
            }
        }
        CliCommand::UpdateApplication {
            id,
            redirect_url,
            webhook_url,
        } => {
            let application = commands::get_application_by_id(*id)
                .await
                .expect("Could not get application");
            let webhook_url = if let Some(webhook_url) = webhook_url {
                if webhook_url.is_empty() {
                    None
                } else {
                    Some(webhook_url.as_str())
                }
            } else {
                application.webhook_url.as_deref()
            };
            let result = commands::update_application(
                &application,
                &redirect_url
                    .clone()
                    .unwrap_or_else(|| application.redirect_url.to_string()),
                webhook_url,
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
        CliCommand::UpdateApplicationSecret { id } => {
            let application = commands::get_application_by_id(*id)
                .await
                .expect("Could not get application");
            let result = commands::update_application_secret(&application).await;

            match result {
                Ok((_, secret)) => {
                    println!("Application secret updated successfully.");
                    println!("\nSecret: {}", secret);
                }
                Err(err) => println!("Failed to update application secret.\n\n{err}"),
            }
        }
        CliCommand::UpdateApplicationWebhookSecret { id } => {
            let application = commands::get_application_by_id(*id)
                .await
                .expect("Could not get application");
            let result = commands::update_application_webhook_secret(&application).await;

            match result {
                Ok(application) => {
                    println!("Application webhook secret updated successfully.");
                    println!("\nWebhook secret: {}", application.webhook_secret);
                }
                Err(err) => println!("Failed to update application webhook secret.\n\n{err}"),
            }
        }
    }
}
