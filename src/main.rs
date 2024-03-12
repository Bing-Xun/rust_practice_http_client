use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 发送 GET 请求
    let response = reqwest::get("http://127.0.0.1:7878")
        .await?
        .text()
        .await?;
    println!("Response: {}", response);

    // 发送 POST 请求
    // let client = reqwest::Client::new();
    // let response = client.post("https://jsonplaceholder.typicode.com/posts")
    //     .json(&json!({
    //         "title": "foo",
    //         "body": "bar",
    //         "userId": 1
    //     }))
    //     .send()
    //     .await?
    //     .text()
    //     .await?;
    // println!("Response: {}", response);

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // Your code here
//     Ok(())
// }