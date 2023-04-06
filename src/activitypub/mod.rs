use activitypub_federation::protocol::public_key::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    id: String,
    #[serde(rename = "type")]
    kind: String, // TODO: force to be "Person" somehow?
    preferred_username: String,
    name: String,
    summary: String,
    attachment: Vec<PersonAttachment>,
    public_key: PublicKey,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonAttachment {
    #[serde(rename = "type")]
    kind: String,
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: String,
    #[serde(rename = "type")]
    kind: String, // TODO: force to be "Person" somehow?
    content: String,
    conversation: String,
    summary: String,
    attributed_to: String,
    to: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NoteAttachment {
    #[serde(rename = "type")]
    kind: String,
    media_type: String,
    url: String,
    name: String,
    blurhash: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Poi {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String, // TODO: force to be "Person" somehow?
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub attachment: Vec<PersonAttachment>,
}
