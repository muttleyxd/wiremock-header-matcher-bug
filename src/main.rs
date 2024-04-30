fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use wiremock::{
        matchers::{header, headers},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn test_reproduce_header_matcher_bug() {
        let mock_server = MockServer::start().await;

        Mock::given(header("if-modified-since", "Sat, 02 Apr 2005 20:37:00 GMT"))
            .respond_with(ResponseTemplate::new(304))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _result = reqwest::Client::new()
            .get(mock_server.uri())
            .header("if-modified-since", "Sat, 02 Apr 2005 20:37:00 GMT")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
    }

    #[tokio::test]
    async fn test_workaround_that_works() {
        let mock_server = MockServer::start().await;

        Mock::given(headers(
            "if-modified-since",
            vec!["Sat", "02 Apr 2005 20:37:00 GMT"],
        ))
        .respond_with(ResponseTemplate::new(304))
        .expect(1)
        .mount(&mock_server)
        .await;

        let _result = reqwest::Client::new()
            .get(mock_server.uri())
            .header("if-modified-since", "Sat, 02 Apr 2005 20:37:00 GMT")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
    }
}
