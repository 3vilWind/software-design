use std::ops::Add;

use chrono::{DateTime, NaiveDateTime, Utc};
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, query_param};

use task2::hashtag_statistics::HashtagStatistics;
use task2::vk_api::{RealVkApiParser, RealVkApiRequester, VkApi, VkApiError, VkApiRequester};

#[tokio::test]
async fn stub_api() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/method/newsfeed.search"))
        .and(query_param("end_time", "1668388063"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string("{\"response\":{\"count\":20,\"items\":[],\"total_count\":20}}")
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/method/newsfeed.search"))
        .and(query_param("end_time", "1668384463"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string("{\"response\":{\"count\":15,\"items\":[],\"total_count\":15}}")
        )
        .mount(&mock_server)
        .await;

    let requester = Box::new(RealVkApiRequester {
        access_token: "token".to_string(),
        base_url: format!("{}/method/newsfeed.search", &mock_server.uri()).to_string(),
    });
    let parser = Box::new(RealVkApiParser);
    let api = VkApi {
        requester,
        parser,
        sleep_on_too_many_requests: std::time::Duration::from_secs(1),
    };
    let start_time = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(1668388063i64, 0u32).unwrap(),
        Utc);
    let data = api.get_statistics("#mem", start_time, 2).await.unwrap();
    assert_eq!(data, vec![20u32, 15u32]);
}

#[tokio::test]
async fn stub_check_vk_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/method/newsfeed.search"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string("{\"error\":{\"error_code\":6,\"error_msg\":\"Too many requests per second\",\"request_params\":[{\"key\":\"count\",\"value\":\"0\"},{\"key\":\"v\",\"value\":\"5.131\"},{\"key\":\"q\",\"value\":\"#мем\"},{\"key\":\"start_time\",\"value\":\"1667591015\"},{\"key\":\"end_time\",\"value\":\"1667677415\"},{\"key\":\"method\",\"value\":\"newsfeed.search\"},{\"key\":\"oauth\",\"value\":\"1\"}]}}")
        )
        .mount(&mock_server)
        .await;

    let requester = Box::new(RealVkApiRequester {
        access_token: "token".to_string(),
        base_url: format!("{}/method/newsfeed.search", &mock_server.uri()).to_string(),
    });
    let start_time = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(1668388063i64, 0u32).unwrap(),
        Utc);
    let end_time = start_time.add(chrono::Duration::hours(1i64));
    let data = requester.newsfeed_search("#mem", start_time, end_time).await;
    assert!(data.is_err());
    assert_eq!(data.err().unwrap().downcast_ref::<VkApiError>().unwrap().code, 6);
}