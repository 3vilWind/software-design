use std::ops::Sub;

use async_trait::async_trait;
use chrono::Duration;
use chrono::prelude::{DateTime, Utc};
use futures::StreamExt;

use crate::vk_api::{VkApi, VkApiParser, VkApiRequester};

#[async_trait]
pub trait HashtagStatistics {
    async fn get_statistics(&self, hashtag: &str, from: DateTime<Utc>, hours: u32) -> anyhow::Result<Vec<u32>>;
}

#[async_trait]
impl<T, K> HashtagStatistics for VkApi<T, K>
    where T: VkApiRequester + Sync + Send, K: VkApiParser + Sync + Send {
    async fn get_statistics(&self, hashtag: &str, from: DateTime<Utc>, hours: u32) -> anyhow::Result<Vec<u32>> {
        let mut futures = Vec::with_capacity(hours as usize);
        for hour in 0..hours {
            let right = from.sub(Duration::hours(hour as i64));
            let left = right.sub(Duration::hours(1i64));
            futures.push(self.newsfeed_search(hashtag, left, right));
        }
        // let handles = futures::future::join_all(futures).await;
        let handles = futures::stream::iter(futures)
            .buffered(5)
            .collect::<Vec<_>>()
            .await;
        let mut result = Vec::<u32>::with_capacity(handles.len());
        for task in handles {
            result.push(task?.count);
        }
        Ok(result)
    }
}