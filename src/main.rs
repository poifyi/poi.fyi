use std::{
    env,
    fmt::format,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use activitypub::Poi;
use activitypub_federation::{
    axum::json::FederationJson,
    config::{Data, FederationConfig, FederationMiddleware},
    protocol::context::WithContext,
    FEDERATION_CONTENT_TYPE,
};
use anyhow::Error;
use axum::{extract::Path, http::HeaderMap, response::IntoResponse, routing::get};
use diesel::{Connection, PgConnection};

mod activitypub;
mod webfinger_resolver;

fn establish_connection() -> Arc<Mutex<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Arc::new(Mutex::new(
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
    ))
}

#[actix_rt::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    log::debug!("setup");

    env::var("DOMAIN").expect("DOMAIN must be set");
    let data = FederationConfig::builder()
        .domain("dev.poi.fyi")
        .app_data(establish_connection())
        .build()?;

    let app = axum::Router::new()
        .route("/.well-known/webfinger", get(webfinger_resolver::webfinger))
        .route("/:user", get(http_get_user))
        .layer(FederationMiddleware::new(data));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn http_get_user(
    header_map: HeaderMap,
    Path(name): Path<String>,
    data: Data<Arc<Mutex<PgConnection>>>,
) -> impl IntoResponse {
    let accept = header_map.get("accept").map(|v| v.to_str().unwrap());
    if accept == Some(FEDERATION_CONTENT_TYPE) {
        // let db_user = data.read_local_user(name).await.unwrap();
        let user = Poi {
            id: format!(""),
            kind: "Person".into(),
            preferred_username: "poi_123".into(),
            name: "Point of Interest".into(),
            summary: "This is a point of interest.".into(),
            attachment: vec![],
        };
        let json_user = serde_json::to_string(&user);
        FederationJson(WithContext::new_default(user)).into_response()
    } else {
        "Hello World".into_response()
    }
}
