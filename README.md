# pasarguard-api

Typed async Rust client for the [PasarGuard](https://github.com/PasarGuard/panel) panel API,
built on `reqwest` and `serde`.

It is a full port of the Python client
[`pasarguard`](https://github.com/AmirKenzo/pasarguard_api) 2.4.0: every endpoint method
(users, admins, roles, API keys, nodes, cores, hosts, groups, templates, subscriptions,
settings, system) and every model, with the same method names and JSON field names.

## Install

```toml
[dependencies]
pasarguard-api = { git = "https://github.com/AmirKenzo/pasarguard-api-rs", tag = "v0.1.0" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick start

```rust,no_run
use pasarguard_api::client::GetUsersParams;
use pasarguard_api::{Credential, PanelClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Log in with an admin account...
    let anonymous = PanelClient::new("https://panel.example.com", Credential::Anonymous)?;
    let token = anonymous.get_token("admin", "secret").await?;
    let panel = anonymous.with_credential(Credential::from_token(token.access_token));

    // ...or use an API key directly.
    // let panel = PanelClient::new("https://panel.example.com", Credential::from_token("pg_key_..."))?;

    let users = panel
        .get_users(&GetUsersParams { limit: Some(10), ..Default::default() })
        .await?;
    for user in users.users {
        println!("{} {}", user.username, user.status);
    }
    Ok(())
}
```

A runnable version lives in [`examples/getting_started.rs`](examples/getting_started.rs).

## Authentication

The client holds one [`Credential`](src/credential.rs):

| Credential | Header sent |
|---|---|
| `Credential::Bearer(token)` | `Authorization: Bearer <token>` |
| `Credential::ApiKey(key)` | `X-API-Key: <key>` |
| `Credential::Anonymous` | none (login, subscription and health endpoints) |

`Credential::from_token` picks `ApiKey` for values starting with `pg_key_` and `Bearer`
otherwise. Where the Python client takes `token=` on every call, use
`panel.with_credential(...)`: it returns a cheap clone that shares the same connection pool.

## Transport options

```rust,no_run
use std::time::Duration;
use pasarguard_api::{Credential, HttpOptions, PanelClient, build_http_client_with};

# fn main() -> Result<(), pasarguard_api::PanelError> {
let http = build_http_client_with(&HttpOptions {
    timeout: Duration::from_secs(30),
    verify: true,                                     // check TLS certificates
    proxy: Some("socks5://127.0.0.1:1080".into()),    // http://, https:// or socks5://
    ..HttpOptions::default()
})?;
let panel = PanelClient::with_http(http, "https://panel.example.com", Credential::from_token("pg_key_..."));
# Ok(())
# }
```

Like the Python client, certificates are **not** verified by default because many panels
use self-signed certificates. Set `verify: true` for panels with a valid certificate.

## Errors

Every method returns `Result<T, PanelError>`:

- `PanelError::Status`: the panel answered with a non-2xx status; the error carries the
  status code, URL and response body.
- `PanelError::Transport`: connection, TLS or timeout failure.
- `PanelError::Decode`: the response did not match the expected model.

## Differences from the Python client

- No per-call `token=` argument: see [Authentication](#authentication).
- No SSH tunnel mode. Open the tunnel yourself (or use `proxy`) and point the client at it.
- No `close()`: the connection pool is released when the client is dropped.
- Models keep fields the panel adds in newer versions in an `extra` map instead of
  failing, so older crate versions keep working against newer panels.

## License

MIT
