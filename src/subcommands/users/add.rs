use core::controller;
use core::models::{User, UserModify};
use core::utils::EMAIL_REGEX;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use regex::Regex;
use std::error::Error;

pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    let pool = controller::connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();

    let username = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Username: ")
        .interact()
        .unwrap();

    let email = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Email: ")
        .validate_with(|input: &String| -> Result<(), &str> {
            let re = Regex::new(EMAIL_REGEX).unwrap();
            if re.is_match(input) {
                Ok(())
            } else {
                Err("Invalid email address provided")
            }
        })
        .interact()
        .unwrap();

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password: ")
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();

    User::create(username, email, password, pool).await?;
    Ok(())
}
