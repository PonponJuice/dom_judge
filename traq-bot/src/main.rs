use std::{env, net::SocketAddr};

use axum::{Router, body::Bytes, extract::State, routing::post};
use http::{HeaderMap, StatusCode};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;
use traq::{
    apis::{
        configuration::Configuration, message_api::post_direct_message,
        stamp_api::add_message_stamp,
    },
    models::PostMessageStampRequest,
};
use traq_bot_http::{Event, RequestParser, payloads::MessageCreatedPayload};

mod util;

#[derive(Clone)]
pub struct App {
    request_parser: RequestParser,
    client_config: Configuration,
    authorization: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .init();

    let verification_token = env::var("VERIFICATION_TOKEN").unwrap();
    let request_parser = RequestParser::new(&verification_token);

    let access_token = env::var("BOT_ACCESS_TOKEN").unwrap();
    let client_config = Configuration {
        bearer_access_token: Some(access_token),
        ..Default::default()
    };

    let authorization = env::var("AUTHORIZATION").unwrap();

    let app = App {
        request_parser,
        client_config,
        authorization,
    };

    let router = make_router(app);
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    let server = TcpListener::bind(addr).await.unwrap();
    axum::serve(server, router).await.unwrap();
}

pub fn make_router(app: App) -> Router {
    Router::new().route("/", post(bot_handle)).with_state(app)
}

async fn bot_handle(State(app): State<App>, headers: HeaderMap, body: Bytes) -> StatusCode {
    let event = match app.request_parser.parse(headers.iter(), &body) {
        Ok(event) => event,
        Err(err) => {
            eprintln!("ERROR: {err}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    match event {
        Event::MessageCreated(payload) => message_created(app, payload).await,
        _ => StatusCode::NO_CONTENT,
    }
}

async fn message_created(app: App, payload: MessageCreatedPayload) -> StatusCode {
    tracing::info!(
        "{}さんがメッセージを投稿しました。\n内容: {}",
        payload.message.user.display_name,
        payload.message.text
    );
    if payload.message.text.starts_with("/create_team ") {
        // コメントを確認したことを示すスタンプを押す
        let _ = add_message_stamp(
            &app.client_config,
            &payload.message.id,
            "4e4c3c0b-2a23-439d-98b1-2fa3ef5caf40", // 既読スタンプのID
            Some(PostMessageStampRequest { count: 1 }),
        )
        .await;

        let team_name = payload
            .message
            .text
            .trim_start_matches("/create_team ")
            .trim();
        if team_name.is_empty() {
            tracing::warn!("チーム名が指定されていません。");
            return StatusCode::BAD_REQUEST;
        }

        tracing::info!("チーム '{}' を作成します。", team_name);

        let password = util::make_random_str(16);

        let user = util::User {
            username: payload.message.user.name.clone(),
            team_name: team_name.to_string(),
            password,
        };

        if let Err(e) = util::make_user(user.clone(), &app.authorization).await {
            tracing::error!("ユーザー作成に失敗しました: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        tracing::info!("チーム '{}' のユーザーを作成しました。", team_name);

        let message = format!(
            "username: {}\nパスワード: {}\nこの情報は大切に保管してください。",
            user.team_name, user.password
        );

        let request = traq::models::PostMessageRequest {
            content: message,
            embed: None,
        };

        let response =
            post_direct_message(&app.client_config, &payload.message.user.id, Some(request)).await;

        let _ = add_message_stamp(
            &app.client_config,
            &payload.message.id,
            "aea52f9a-7484-47ed-ab8f-3b4cc84a474d", // 既読スタンプのID
            Some(PostMessageStampRequest { count: 1 }),
        )
        .await;

        if let Err(e) = response {
            tracing::error!("ダイレクトメッセージの送信に失敗しました: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR;
        }

        tracing::info!("ダイレクトメッセージを送信しました。");
    }
    StatusCode::NO_CONTENT
}
