use crate::config::DEFAULT_CONFIG;

pub fn cmd() -> Result<(), Box<dyn std::error::Error>> {
    println!("{DEFAULT_CONFIG}");
    Ok(())
}
