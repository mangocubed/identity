use std::process::Command;

use chrono::{SecondsFormat, Utc};

fn main() {
    let app_server_url = std::env::var("APP_SERVER_URL").unwrap_or("http://127.0.0.1:8080".to_owned());
    let app_token = std::env::var("APP_TOKEN").unwrap_or("00000000".to_owned());
    let git_rev_short = {
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .unwrap();

        String::from_utf8(output.stdout).unwrap()
    };
    let build_datetime = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    println!("cargo:rustc-env=APP_SERVER_URL={app_server_url}");
    println!("cargo:rustc-env=APP_TOKEN={app_token}");
    println!("cargo:rustc-env=GIT_REV_SHORT={git_rev_short}");
    println!("cargo:rustc-env=BUILD_DATETIME={build_datetime}");
}
