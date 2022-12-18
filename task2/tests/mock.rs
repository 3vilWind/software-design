use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;

use task2::hashtag_statistics::HashtagStatistics;
use task2::vk_api::{RealVkApiParser, VkApi, VkApiRequester};

struct MockVkApi {}

#[async_trait]
impl VkApiRequester for MockVkApi {
    async fn newsfeed_search(&self, _query: &str, _start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> anyhow::Result<Value> {
        if end_time == DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(1668388063i64, 0u32).unwrap(),
            Utc) {
            Ok(serde_json::from_str("{\"count\":20,\"items\":[],\"total_count\":20}").unwrap())
        } else {
            Ok(serde_json::from_str("{\"count\":15,\"items\":[],\"total_count\":15}").unwrap())
        }
    }
}

#[tokio::test]
async fn mock_api() {
    let requester = Box::new(MockVkApi {});
    let parser = Box::new(RealVkApiParser);
    let api = VkApi {
        requester,
        parser,
        sleep_on_too_many_requests: Duration::from_secs(1),
    };
    let start_time = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(1668388063i64, 0u32).unwrap(),
        Utc);
    let data = api.get_statistics("#mem", start_time, 2).await.unwrap();
    assert_eq!(data, vec![20u32, 15u32]);
}
