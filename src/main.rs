use std::{fs::read_to_string, path::Path};

use crate::config::Config;

mod api;
mod config;
mod headers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = toml::from_str(&read_to_string("config.toml")?)?;
    println!("config: {:#?}", config);
    headers::set_identity(config.identity);
    headers::set_authorization(config.authorization);
    headers::set_blade_auth(config.blade_auth);
    for (dest, site_id) in config.sites {
        let result = api::list_all_scores(site_id).await?;
        println!("{}: {:#?}", dest, result);
        result.write_to_file(Path::new(&format!("{}.csv", dest)))?;
    }
    Ok(())
}
