use cli_tooling::tables::{schema, user};
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::error::Error;
fn main() {
    schema::build()
        .migrate("users")
        .unwrap_or_else(|_| println!("Could not run migrations"))
}

// fn init() -> Result<Option<bool>, Box<dyn Error>> {
//     let theme = ColorfulTheme::default();
//     let is_confirmed = Confirm::with_theme(&theme)
//         .with_prompt("Do you want to create a new table migration?")
//         .interact()
//         .unwrap();
//     if !is_confirmed {
//         return Ok(None);
//     }
//     Ok(None)
// }
