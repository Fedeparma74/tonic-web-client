use http::{
    header::{ACCEPT, CONTENT_TYPE},
    response::Builder,
    HeaderMap, HeaderValue, Request, Response,
};
use http_body::Body;
use tonic::body::BoxBody;

use crate::{Error, ResponseBody};

pub async fn call(
    mut base_url: String,
    request: Request<BoxBody>,
) -> Result<Response<ResponseBody>, Error> {
    base_url.push_str(&request.uri().to_string());

    let headers = prepare_headers(request.headers());

    let body = request
        .into_body()
        .data()
        .await
        .transpose()?
        .ok_or(Error::MissingResponseBody)?;

    let client = reqwest::Client::new();

    let response = client
        .post(&base_url)
        .headers(headers)
        .body(body)
        .send()
        .await?;

    let result = Response::builder().status(response.status());
    let (result, content_type) = set_response_headers(result, &response)?;

    let content_type = content_type.ok_or(Error::MissingContentTypeHeader)?;
    let body_stream = response.bytes_stream();

    let body = ResponseBody::new(body_stream, &content_type)?;

    Ok(result.body(body)?)
}

fn prepare_headers(header_map: &HeaderMap<HeaderValue>) -> HeaderMap<HeaderValue> {
    let mut headers = HeaderMap::new();
    headers.append(
        CONTENT_TYPE,
        HeaderValue::from_static("application/grpc-web+proto"),
    );
    headers.append(
        ACCEPT,
        HeaderValue::from_static("application/grpc-web-text+proto"),
    );
    headers.append("x-grpc-web", HeaderValue::from_static("1"));

    for (header_name, header_value) in header_map {
        if header_name != CONTENT_TYPE && header_name != ACCEPT {
            headers.append(header_name, header_value.clone());
        }
    }

    headers
}

fn set_response_headers(
    mut result: Builder,
    response: &reqwest::Response,
) -> Result<(Builder, Option<HeaderValue>), Error> {
    let headers = response.headers();

    let mut content_type = None;

    for (header_name, header_value) in headers {
        if header_name == CONTENT_TYPE {
            content_type = Some(header_value.clone());
        }

        result = result.header(header_name, header_value);
    }

    Ok((result, content_type))
}
