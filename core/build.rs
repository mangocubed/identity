use chrono::{SecondsFormat, Utc};

fn main() {
    let build_datetime = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);

    println!("cargo:rustc-env=BUILD_DATETIME={build_datetime}");
}
