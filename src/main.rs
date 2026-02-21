use cli_tooling::file_manager::write_migration;
use cli_tooling::tables::user;
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::error::Error;
fn main() {
    let migration = user::create_user_table().create_table();
    write_migration(&migration, " file_name ").unwrap();
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
