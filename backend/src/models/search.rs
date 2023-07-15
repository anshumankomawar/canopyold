use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchTopicPayload {
    pub query: String,
}
