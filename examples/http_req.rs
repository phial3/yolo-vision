use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use yolo_vision::utils::http_client::HttpClient;

fn main() {
    // http_get();

    // http_post();

    let _ = test_http_client();
}

#[tokio::main]
async fn http_get() -> Result<(), Box<dyn Error>> {
    // 设置请求的 URL
    let url = "http://172.24.82.44/umeam-ctu/alarm/plan/targetList";

    // 创建请求头
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_static("Bearer YOUR_TOKEN"),
    );
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    // 定义查询参数
    let params = [("param1", "value1"), ("param2", "value2")];

    // 发送 GET 请求
    let response: Value = reqwest::Client::new()
        .get(url)
        .query(&params)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    // 打印响应结果
    // println!("GET 响应: {:?}", response);

    let data = response.get("data").unwrap();
    let rows = data.as_array().unwrap();
    for row in rows {
        println!("{:?}", row);
        println!("---------------------");
    }

    Ok(())
}

#[tokio::main]
async fn http_post() -> Result<(), Box<dyn Error>> {
    // 设置请求的 URL
    let url = "http://172.24.82.44/umeam-ctu/alarm/plan/page";

    // 创建请求头
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_static("Bearer YOUR_TOKEN"),
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    // 创建请求体
    let body = serde_json::json!({
        "key1": "value1",
        "key2": "value2",
        "pageNo": 1,
        "pageSize": 10
    });

    // 发送 POST 请求
    let response: Value = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    // 打印响应结果
    //  println!("POST 响应: {:?}", response);

    let data = response.get("data").unwrap();
    let records = data.get("records").unwrap();
    for record in records.as_array().unwrap() {
        println!("{:?}", record);
        println!("---------------------");
    }

    Ok(())
}

#[tokio::main]
async fn test_http_client() -> Result<(), Box<dyn Error>> {
    let http_client = HttpClient::new();

    // GET 请求示例
    let mut query_params = HashMap::new();
    query_params.insert("userId", "1");

    let get_response = http_client
        .get(
            "https://jsonplaceholder.typicode.com/posts",
            None,
            Some(query_params),
        )
        .await?;

    println!("GET JSON Response: {:?}", get_response);
    println!("--------------------");

    // POST JSON 示例
    let mut json_body = HashMap::new();
    json_body.insert("title", "foo");
    json_body.insert("body", "bar");
    json_body.insert("userId", "1");

    let post_response = http_client
        .post_json(
            "https://jsonplaceholder.typicode.com/posts",
            None,
            &json_body,
        )
        .await?;

    println!("POST JSON Response: {:?}", post_response);
    println!("--------------------");

    // POST form-data 示例
    let mut form_data = HashMap::new();
    form_data.insert("key1", "value1");
    form_data.insert("key2", "value2");

    let post_form_response = http_client
        .post_form("https://httpbin.org/post", None, &form_data)
        .await?;

    println!("POST Form JSON Response: {:?}", post_form_response);
    println!("--------------------");

    // POST form-data 示例（带文件）
    let mut fields = HashMap::new();
    fields.insert("description", "This is a test file field description.");

    let file_path = std::path::Path::new("/tmp/aTrustInstallOut.log");

    let post_form_with_file_response = http_client
        .post_form_with_file("https://httpbin.org/post", None, fields, "file", &file_path)
        .await?;

    println!(
        "POST Form with File JSON Response: {:?}",
        post_form_with_file_response
    );
    println!("--------------------");

    tokio::time::sleep(Duration::from_secs(5)).await;

    Ok(())
}
