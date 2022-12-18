use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::hashtag_statistics::HashtagStatistics;
use crate::vk_api::{RealVkApiParser, RealVkApiRequester, VkApi};

pub mod vk_api;
pub mod hashtag_statistics;

pub async fn run(access_token: &str, hashtag: &str, from: DateTime<Utc>, hours: u32) -> anyhow::Result<Vec<u32>> {
    let requester = Box::new(RealVkApiRequester {
        access_token: access_token.to_string(),
        base_url: "https://api.vk.com/method/newsfeed.search".to_string(),
    });
    let parser = Box::new(RealVkApiParser);
    let api = VkApi {
        requester,
        parser,
        sleep_on_too_many_requests: Duration::from_secs(1),
    };
    api.get_statistics(hashtag, from, hours).await
}
