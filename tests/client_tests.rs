macro_rules! b {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

mod mock {
    use serde_json::json;
    use std::borrow::Cow;

    fn client<'a>() -> podcast_api::Client<'a> {
        podcast_api::Client::new(None)
    }

    #[test]
    fn search() {
        b!(async {
            let response = client()
                .search(&json!({
                        "q": "dummy",
                        "sort_by_date": 1
                }))
                .await
                .unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/search");
            let mut p = response.request.url().query_pairs();
            assert_eq!(p.count(), 2);
            assert_eq!(p.next(), Some((Cow::Borrowed("q"), Cow::Borrowed("dummy"))));
            assert_eq!(p.next(), Some((Cow::Borrowed("sort_by_date"), Cow::Borrowed("1"))));
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["results"].as_array().unwrap().len() > 0);
        });
    }

    #[test]
    fn search_with_authentication_error() {
        b!(async {
            let response = podcast_api::Client::new(Some("wrong_key"))
                .search(&json!({
                    "q": "dummy",
                    "sort_by_date": 1
                }))
                .await;
            assert!(match response {
                Err(podcast_api::Error::AuthenticationError) => true,
                _ => false,
            });
        });
    }

    #[test]
    fn typeahead() {
        b!(async {
            let response = client()
                .typeahead(&json!({
                    "q": "dummy",
                    "show_podcasts": 1
                }))
                .await
                .unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/typeahead");
            let mut p = response.request.url().query_pairs();
            assert_eq!(p.count(), 2);
            assert_eq!(p.next(), Some((Cow::Borrowed("q"), Cow::Borrowed("dummy"))));
            assert_eq!(p.next(), Some((Cow::Borrowed("show_podcasts"), Cow::Borrowed("1"))));
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["terms"].as_array().unwrap().len() > 0);
        });
    }

    #[test]
    fn fetch_best_podcasts() {
        b!(async {
            let response = client()
                .fetch_best_podcasts(&json!({
                    "genre_id": 23
                }))
                .await
                .unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/best_podcasts");
            let mut p = response.request.url().query_pairs();
            assert_eq!(p.count(), 1);
            assert_eq!(p.next(), Some((Cow::Borrowed("genre_id"), Cow::Borrowed("23"))));
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["total"].as_i64().unwrap() > 0);
        });
    }

    #[test]
    fn fetch_podcast_by_id() {
        b!(async {
            let response = client().fetch_podcast_by_id("dummy_id", &json!({})).await.unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/podcasts/dummy_id");
            let p = response.request.url().query_pairs();
            assert_eq!(p.count(), 0);
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["episodes"].as_array().unwrap().len() > 0);
        });
    }

    #[test]
    fn batch_fetch_podcasts() {
        b!(async {
            let response = client()
                .batch_fetch_podcasts(&json!({
                    "ids": "996,777,888,1000"
                }))
                .await
                .unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::POST);
            assert_eq!(response.request.url().path(), "/api/v2/podcasts");
            let mut p = form_urlencoded::parse(response.request.body().unwrap().as_bytes().unwrap());
            assert_eq!(p.count(), 1);
            assert_eq!(
                p.next(),
                Some((Cow::Borrowed("ids"), Cow::Borrowed("996,777,888,1000")))
            );
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["podcasts"].as_array().unwrap().len() > 0);
        });
    }

    #[test]
    fn fetch_episode_by_id() {
        b!(async {
            let response = client().fetch_episode_by_id("dummy_id", &json!({})).await.unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/episodes/dummy_id");
            let p = response.request.url().query_pairs();
            assert_eq!(p.count(), 0);
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["podcast"].as_object().unwrap()["rss"].as_str().unwrap().len() > 0);
        });
    }

    #[test]
    fn batch_fetch_episodes() {
        b!(async {
            let response = client()
                .batch_fetch_episodes(&json!({
                    "ids": "996,777,888,1000"
                }))
                .await
                .unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::POST);
            assert_eq!(response.request.url().path(), "/api/v2/episodes");
            let mut p = form_urlencoded::parse(response.request.body().unwrap().as_bytes().unwrap());
            assert_eq!(p.count(), 1);
            assert_eq!(
                p.next(),
                Some((Cow::Borrowed("ids"), Cow::Borrowed("996,777,888,1000")))
            );
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["episodes"].as_array().unwrap().len() > 0);
        });
    }

    #[test]
    fn fetch_curated_podcasts_list_by_id() {
        b!(async {
            let response = client()
                .fetch_curated_podcasts_list_by_id("asdfsdaf", &json!({}))
                .await
                .unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/curated_podcasts/asdfsdaf");
            let p = response.request.url().query_pairs();
            assert_eq!(p.count(), 0);
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["podcasts"].as_array().unwrap().len() > 0);
        });
    }

    #[test]
    fn fetch_curated_podcasts_lists() {
        b!(async {
            let response = client()
                .fetch_curated_podcasts_lists(&json!({
                    "page": 2
                }))
                .await
                .unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/curated_podcasts");
            let mut p = response.request.url().query_pairs();
            assert_eq!(p.count(), 1);
            assert_eq!(p.next(), Some((Cow::Borrowed("page"), Cow::Borrowed("2"))));
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["total"].as_i64().unwrap() > 0);
        });
    }

    #[test]
    fn fetch_podcast_genres() {
        b!(async {
            let response = client()
                .fetch_podcast_genres(&json!({
                    "top_level_only": 1
                }))
                .await
                .unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/genres");
            let mut p = response.request.url().query_pairs();
            assert_eq!(p.count(), 1);
            assert_eq!(p.next(), Some((Cow::Borrowed("top_level_only"), Cow::Borrowed("1"))));
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["genres"].as_array().unwrap().len() > 0);
        });
    }

    #[test]
    fn fetch_podcast_regions() {
        b!(async {
            let response = client().fetch_podcast_regions(&json!({})).await.unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/regions");
            let p = response.request.url().query_pairs();
            assert_eq!(p.count(), 0);
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["regions"].as_object().unwrap().keys().len() > 0);
        });
    }

    #[test]
    fn fetch_podcast_languages() {
        b!(async {
            let response = client().fetch_podcast_languages(&json!({})).await.unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/languages");
            let p = response.request.url().query_pairs();
            assert_eq!(p.count(), 0);
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["languages"].as_array().unwrap().len() > 0);
        });
    }

    #[test]
    fn just_listen() {
        b!(async {
            let response = client().just_listen(&json!({})).await.unwrap();
            // Request
            assert_eq!(response.request.method(), http::Method::GET);
            assert_eq!(response.request.url().path(), "/api/v2/just_listen");
            let p = response.request.url().query_pairs();
            assert_eq!(p.count(), 0);
            // Response
            let body = response.json().await.unwrap();
            assert!(body.is_object());
            assert!(body["audio_length_sec"].as_i64().unwrap() > 0);
        });
    }
}
