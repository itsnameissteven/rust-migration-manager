use chrono::Utc;
use std::fs;
use std::io::ErrorKind;

struct Config {
    migration_dir: String,
}
pub fn write_migration(migration: &str, file_name: &str) -> Result<(), std::io::Error> {
    let dir = "migrations";
    let time_stamp = Utc::now().timestamp().to_string();
    let file_path = format!("{}/{}_{}.sql", dir, time_stamp, file_name.trim());

    let path = match fs::create_dir("migrations") {
        Ok(_) => {
            println!("Migration directory created");
            Ok(())
        }
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e),
    };
    if let Err(e) = path {
        return Err(e);
    }

    match fs::write(file_path, migration) {
        Ok(_) => {
            println!("Migration created");
            Ok(())
        }
        Err(e) => Err(e),
    }
}
