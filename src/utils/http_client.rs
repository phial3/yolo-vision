use anyhow::Error;
use reqwest::{multipart, Client, Response};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: Client::builder()
                .timeout(Duration::from_secs(10)) // 设置全局请求超时为 10 秒
                .build()
                .unwrap(),
        }
    }

    /// GET 请求方法，包含 URL、请求头、请求参数，并返回 JSON 数据
    pub async fn get(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        query_params: Option<HashMap<&str, &str>>,
    ) -> Result<Value, Error> {
        let mut req = self.client.get(url);

        // 添加查询参数
        if let Some(params) = query_params {
            req = req.query(&params);
        }

        // 添加 headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                req = req.header(&key, &value);
            }
        }

        // let response = req.send().await?.json::<Value>().await?;
        let response = self.handle_response(req.send().await?).await?;
        Ok(response)
    }

    /// POST 请求方法，包含 URL、请求头、请求 BODY，并返回 JSON 数据
    pub async fn post_json<T: Serialize>(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: &T,
    ) -> Result<Value, Error> {
        let mut req = self.client.post(url).json(body);

        // 添加自定义 headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                req = req.header(&key, &value);
            }
        }

        // let response = req.send().await?.json::<Value>().await?;
        let response = self.handle_response(req.send().await?).await?;
        Ok(response)
    }

    /// POST form-data 请求方法
    pub async fn post_form(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        form_data: &HashMap<&str, &str>,
    ) -> Result<Value, Error> {
        let mut req = self.client.post(url).form(form_data);

        // 添加 headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                req = req.header(&key, &value);
            }
        }

        // let response = req.send().await?.json::<Value>().await?;
        let response = self.handle_response(req.send().await?).await?;
        Ok(response)
    }

    /// POST form-data 请求方法（支持文件上传）
    pub async fn post_form_with_file(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        fields: HashMap<&str, &str>,
        file_field_name: &str,
        file_path: &Path,
    ) -> Result<Value, Error> {
        // 创建一个 multipart 表单
        let mut form = multipart::Form::new();

        // 添加普通字段到 form-data
        for (key, value) in fields {
            form = form.text(key.to_string(), value.to_string());
        }

        // 添加文件字段到 form-data
        let file_bytes = std::fs::read(file_path).unwrap();
        let file_part = multipart::Part::bytes(file_bytes)
            .file_name(file_path.file_name().unwrap().to_string_lossy().to_string());

        form = form.part(file_field_name.to_string(), file_part);

        let mut req = self.client.post(url).multipart(form);

        // 添加 headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                req = req.header(&key, &value);
            }
        }

        // 发送请求并解析为 JSON
        // let response = req.send().await?.json::<Value>().await?;
        let response = self.handle_response(req.send().await?).await?;
        Ok(response)
    }

    /// PUT 请求方法
    pub async fn put_json<T: serde::Serialize>(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: &T,
    ) -> Result<Value, Error> {
        let mut req = self.client.put(url).json(body);

        // 添加 headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                req = req.header(&key, &value);
            }
        }

        let response = self.handle_response(req.send().await?).await?;
        Ok(response)
    }

    /// DELETE 请求方法
    pub async fn delete(
        &self,
        url: &str,
        headers: Option<HashMap<String, String>>,
    ) -> Result<Value, Error> {
        let mut req = self.client.delete(url);

        // 添加 headers
        if let Some(headers_map) = headers {
            for (key, value) in headers_map {
                req = req.header(&key, &value);
            }
        }

        let response = self.handle_response(req.send().await?).await?;
        Ok(response)
    }

    /// 处理响应并将其转换为 JSON
    async fn handle_response(&self, response: Response) -> Result<Value, Error> {
        if response.status().is_success() {
            let json = response.json::<Value>().await?;
            Ok(json)
        } else {
            // 可以根据响应状态码进行自定义错误处理
            let status = response.status();
            let error_text = response.text().await?;
            Err(Error::msg(format!(
                "HTTP request failed with status code: {}, error: {}",
                status, error_text
            )))
        }
    }
}
