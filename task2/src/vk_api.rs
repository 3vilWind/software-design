use std::time::Duration;
use anyhow::{Context, Error};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use thiserror::Error;

#[async_trait]
pub trait VkApiRequester {
    async fn newsfeed_search(&self, query: &str, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> anyhow::Result<Value>;
}

pub trait VkApiParser {
    fn parse_newsfeed_search(&self, data: &Value) -> anyhow::Result<NewsFeedSearch>;
}

pub struct NewsFeedSearch {
    pub count: u32,
}
pub struct VkApi<T, E>
    where T: VkApiRequester,
          E: VkApiParser {
    pub requester: Box<T>,
    pub parser: Box<E>,
    pub sleep_on_too_many_requests: Duration,
}

impl<T, E> VkApi<T, E>
    where T: VkApiRequester,
          E: VkApiParser {
    pub async fn newsfeed_search(&self, query: &str, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> anyhow::Result<NewsFeedSearch> {
        let data: Value;
        loop {
            let response = self.requester.newsfeed_search(query, start_time, end_time).await;
            match response {
                Err(x) => {
                    if let Some(x) = x.downcast_ref::<VkApiError>() {
                        if x.code == 6 {
                            tokio::time::sleep(self.sleep_on_too_many_requests).await;
                            continue;
                        }
                    }
                    return Err(x.context("Error during http request"));
                }
                Ok(x) => {
                    data = x;
                    break;
                }
            }
        }

        self.parser.parse_newsfeed_search(&data)
            .context("Error during parsing response")
    }
}
pub struct RealVkApiRequester {
    pub access_token: String,
    pub base_url: String,
}

#[derive(Error, Debug)]
#[error("{code}: {msg}")]
pub struct VkApiError {
    pub code: i64,
    pub msg: String,
}

pub struct RealVkApiParser;

#[async_trait]
impl VkApiRequester for RealVkApiRequester {
    async fn newsfeed_search(&self, query: &str, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> anyhow::Result<Value> {
        let client = reqwest::Client::new();

        let mut body = client.get(&self.base_url)
            .query(&[
                ("v", "5.131"),
                ("count", "0"),
                ("access_token", &self.access_token),
                ("q", query),
                ("start_time", &start_time.timestamp().to_string()),
                ("end_time", &end_time.timestamp().to_string()),
            ])
            .send()
            .await?
            .json::<Value>()
            .await?;

        if let Some(response) = body.get_mut("response") {
            return Ok(response.take());
        }

        let error = body.get_mut("error").map(|error| {
            let err = VkApiError {
                msg: error.get_mut("error_msg")?.as_str()?.into(),
                code: error.get_mut("error_code")?.as_i64()?,
            };
            return Some(err.into());
        });
        if let Some(Some(parsed_error)) = error {
            return Err(parsed_error);
        }
        Err(Error::msg("Can't parse error"))
    }
}

impl VkApiParser for RealVkApiParser {
    fn parse_newsfeed_search(&self, data: &Value) -> anyhow::Result<NewsFeedSearch> {
        let count = data.get("count").map(|count| {
            count.as_i64()
        });
        if let Some(Some(count)) = count {
            if let Ok(count) = u32::try_from(count) {
                return Ok(NewsFeedSearch { count });
            }
        }
        Err(Error::msg("Can't parse response"))
    }
}