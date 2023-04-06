// use std::{
//     env,
//     sync::{Arc, Mutex},
// };

// use diesel::PgConnection;
// use webfinger::{Prefix, Resolver};

// pub struct WebfingerResolver;

// impl Resolver<Arc<Mutex<PgConnection>>> for WebfingerResolver {
//     fn instance_domain<'a>(&self) -> &'a str {
//         env::var("DOMAIN").expect("DOMAIN must be set").as_str()
//     }

//     fn find(
//         &self,
//         prefix: Prefix,
//         acct: String,
//         db: Arc<Mutex<PgConnection>>,
//     ) -> Result<Webfinger, ResolverError> {
//         Ok(Webfinger {
//             subject: acct.clone(),
//             aliases: vec![acct.clone()],
//             links: vec![Link {
//                 rel: "http://webfinger.net/rel/profile-page".to_string(),
//                 mime_type: None,
//                 href: format!("https://dev.poi.fyi/@{}", acct),
//             }],
//         })
//     }
// }

use std::{
    collections::HashMap,
    env,
    sync::{Arc, Mutex},
};

use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
};
use anyhow::Error;
use axum::{extract::Query, http::HeaderMap, response::IntoResponse, Json};
use diesel::PgConnection;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

fn handle_webfinger(
    header_map: HeaderMap,
    path: String,
    data: Data<Arc<Mutex<PgConnection>>>,
) -> Result<Json<Webfinger>, Error> {
    log::debug!("extracting");
    let name = extract_webfinger_name(&path, &data)?;

    log::debug!("setting up url");
    let url_text = format!(
        "https://{}/@{}",
        env::var("DOMAIN").expect("DOMAIN must be set"),
        name
    );
    log::debug!("parsing url");
    let url = Url::parse(&url_text)?;
    // let db_user = data.read_local_user(name).await?;
    log::debug!("building response");
    Ok(Json(build_webfinger_response(name, url)))
}

pub async fn webfinger(
    header_map: HeaderMap,
    Query(path): Query<HashMap<String, String>>,
    data: Data<Arc<Mutex<PgConnection>>>,
) -> impl IntoResponse {
    log::debug!("handling webfinger");
    match handle_webfinger(
        header_map,
        path.get("resource").unwrap_or(&"".to_string()).clone(),
        data,
    ) {
        Ok(response) => {
            log::debug!("okay");
            response
        }
        Err(err) => {
            log::error!("Error: {}", err);
            Webfinger {
                subject: "".to_string(),
                links: vec![],
                aliases: vec![],
                properties: HashMap::new(),
            }
            .into()
        }
    }
}
