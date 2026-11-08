//! Log in, then read system stats and the first users.
//!
//! ```sh
//! PANEL_URL=https://panel.example.com PANEL_USERNAME=admin PANEL_PASSWORD=secret \
//!     cargo run --example getting_started
//! ```

use pasarguard_api::client::GetUsersParams;
use pasarguard_api::{Credential, PanelClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("PANEL_URL")?;
    let username = std::env::var("PANEL_USERNAME")?;
    let password = std::env::var("PANEL_PASSWORD")?;

    let anonymous = PanelClient::new(&url, Credential::Anonymous)?;
    let token = anonymous.get_token(&username, &password).await?;
    let panel = anonymous.with_credential(Credential::from_token(token.access_token));

    let stats = panel.get_system_stats(None).await?;
    println!("panel {} up for {}s", stats.version, stats.uptime_seconds);

    let users = panel
        .get_users(&GetUsersParams {
            limit: Some(5),
            ..GetUsersParams::default()
        })
        .await?;
    println!("{} users in total", users.total);
    for user in users.users {
        println!("- {} ({})", user.username, user.status);
    }
    Ok(())
}
