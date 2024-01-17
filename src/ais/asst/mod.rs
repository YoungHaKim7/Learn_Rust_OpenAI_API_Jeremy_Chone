use crate::Result;

use async_openai::{config::OpenAIConfig, Client};

// region : --- Client
pub const ENV_OPENAI_API_KEY: &str = include_str!("../../../OpenAI_API_Key.txt");

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
    todo!()
}
