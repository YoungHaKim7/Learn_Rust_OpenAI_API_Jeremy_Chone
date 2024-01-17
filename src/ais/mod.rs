pub mod asst;

use async_openai::Client;

use crate::Result;

use self::asst::{OaClient, ENV_OPENAI_API_KEY};

pub fn new_oa_client() -> Result<OaClient> {
    if std::env::var(ENV_OPENAI_API_KEY).is_ok() {
        Ok(Client::new())
    } else {
        println!("No {ENV_OPENAI_API_KEY} env variable. Please set it.");
        Err("No openai api key in env".into())
    }
}
