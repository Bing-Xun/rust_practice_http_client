use hyper::{Body, Client, Request};
use hyper::http::Uri;
use hyper_tls::HttpsConnector;
use std::str::FromStr;


// static COUNT: AtomicUsize = AtomicUsize::new(0);
#[tokio::main]
async fn main() {
    for _ in 0..10 {
        let _ = send().await;
    }
}

async fn send() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建一个 HTTPS 连接器
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // 构建请求 URI
    let uri = Uri::from_str("http://127.0.0.1:8080?aa")?;

    // 构建 HTTP 请求
    let request = Request::builder()
        .uri(uri)
        .body(Body::empty())?;

    // 发送请求并等待响应
    let response = client.request(request).await?;

    // 打印响应状态码
    // println!("Response status code: {}", response.status());

    // 读取响应主体，并转换为字符串
    let body = hyper::body::to_bytes(response.into_body()).await?;
    let body_str = String::from_utf8(body.to_vec())?;
    println!("Response body: {}", body_str);


    Ok(())
}
