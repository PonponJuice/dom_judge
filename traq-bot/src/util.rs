use anyhow::Result;

pub fn make_password() -> String {
    use rand::Rng;
    use rand::distr::Alphanumeric;

    let mut rng = rand::rng();
    let password: String = (0..12).map(|_| rng.sample(Alphanumeric) as char).collect();
    password
}

#[derive(Debug, Clone)]
pub struct User {
    pub team_name: String,
    pub password: String,
}

pub async fn make_user(user: User, authorization: &str) -> Result<()> {
    use reqwest::Client;
    let base_url = std::env::var("BASE_URL").unwrap();

    let team_id = user.team_name.clone().replace(' ', "_");

    let client = Client::new();
    let response = client
        .post(format!("{base_url}/teams"))
        .header("Authorization", authorization)
        .form(&[
            ("name", &user.team_name),
            ("id", &team_id),
            ("group_ids[]", &"participants".to_string()),
            ("display_name", &user.team_name),
            ("nationality", &"JPN".to_string()),
            ("organization_id", &"isct".to_string()),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        tracing::error!("Failed to create user: {} - {}", status, text);
        return Err(anyhow::anyhow!("Failed to create user"));
    }
    tracing::info!("User created successfully: {}", user.team_name);

    let response = client
        .post(format!("{base_url}/users"))
        .header("Authorization", authorization)
        .form(&[
            ("username", &user.team_name),
            ("name", &user.team_name),
            ("password", &user.password),
            ("roles[]", &"team".to_string()),
            ("team_id", &team_id),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        tracing::error!("Failed to create user: {} - {}", status, text);
        return Err(anyhow::anyhow!("Failed to create user"));
    } else {
        tracing::info!("User created successfully: {}", response.text().await?);
    }

    Ok(())
}
