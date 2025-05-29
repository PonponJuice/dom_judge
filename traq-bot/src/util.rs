use anyhow::Result;

pub fn make_random_str(len: u16) -> String {
    use rand::Rng;
    use rand::distr::Alphanumeric;

    let mut rng = rand::rng();
    let password: String = (0..len).map(|_| rng.sample(Alphanumeric) as char).collect();
    password
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub team_name: String,
    pub password: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct TeamCreateResponse {
    pub id: String,
}

pub async fn make_user(user: User, authorization: &str) -> Result<()> {
    use reqwest::Client;
    let base_url = std::env::var("BASE_URL").unwrap();

    tracing::info!("Creating user: {:?}", user);    

    let client = Client::new();
    let response = client
        .post(format!("{base_url}/teams"))
        .header("Authorization", authorization)
        .form(&[
            ("name", &user.username),
            ("id", &make_random_str(8)),
            ("label", &user.username),
            ("group_ids[]", &"3".to_string()),
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
    let team_create_response: TeamCreateResponse = response.json().await?;

    let team_id = team_create_response.id;

    let response = client
        .post(format!("{base_url}/users"))
        .header("Authorization", authorization)
        .form(&[
            ("username", &user.team_name),
            ("name", &user.username),
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
