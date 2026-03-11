// use anyhow::Ok;
use cli_tooling::tables::schema;
// use std::result::Result::Ok;
// use dialoguer::{Confirm, theme::ColorfulTheme};
// use std::error::Error;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let migration = schema::build().write("migrations");
    match migration {
        Ok(_) => println!("All good"),
        Err(e) => println!("Error: {:?}", e),
    };
    Ok(())
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
