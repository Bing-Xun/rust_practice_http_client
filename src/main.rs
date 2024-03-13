use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 发送 GET 请求
    let response = reqwest::get("http://127.0.0.1:8080").await?;

    // 检查响应状态码
    if response.status().is_success() {
        // 获取响应的文本内容并打印
        let body = response.text().await?;
        println!("Response body: {}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
