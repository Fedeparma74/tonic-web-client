use client::proto::{echo_client::EchoClient, EchoRequest};
use tonic::Code;
use tonic_web_client::Client;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

fn build_client() -> EchoClient<Client> {
    let base_url = "http://localhost:50051".to_string();
    let client = Client::new(base_url);

    EchoClient::new(client)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_echo() {
    let mut client = build_client();

    let response = client
        .echo(EchoRequest {
            message: "John".to_string(),
        })
        .await
        .expect("success response")
        .into_inner();

    assert_eq!(response.message, "echo(John)");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_echo_stream() {
    let mut client = build_client();

    let mut stream_response = client
        .echo_stream(EchoRequest {
            message: "John".to_string(),
        })
        .await
        .expect("success stream response")
        .into_inner();

    for i in 0..3 {
        let response = stream_response.message().await.expect("stream message");
        assert!(response.is_some(), "{}", i);
        let response = response.unwrap();

        assert_eq!(response.message, "echo(John)");
    }

    let response = stream_response.message().await.expect("stream message");
    assert!(response.is_none());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_infinite_echo_stream() {
    let mut client = build_client();

    let mut stream_response = client
        .echo_infinite_stream(EchoRequest {
            message: "John".to_string(),
        })
        .await
        .expect("success stream response")
        .into_inner();

    for i in 0..3 {
        let response = stream_response.message().await.expect("stream message");
        assert!(response.is_some(), "{}", i);
        let response = response.unwrap();

        assert_eq!(response.message, format!("echo(John, {})", i + 1));
    }

    let response = stream_response.message().await.expect("stream message");
    assert!(response.is_some());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_error_response() {
    let mut client = build_client();

    let error = client
        .echo_error_response(EchoRequest {
            message: "John".to_string(),
        })
        .await
        .unwrap_err();

    assert_eq!(error.code(), Code::Unauthenticated);
}
