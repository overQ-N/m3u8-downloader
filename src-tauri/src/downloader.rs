use std::{
    collections::HashMap, error::Error, fs, path::Path, process::Command, time::Duration, vec,
};

use reqwest::header::{HeaderMap, HeaderValue};
use tokio::time::sleep;

#[derive(Debug, PartialEq)]
pub struct Client<'a> {
    url: &'a str,
    pub prefix_url: &'a str,
    pub body: Vec<String>,
    header_map: HeaderMap,
}

impl<'a> Client<'a> {
    pub fn new(url: &'a str) -> Self {
        Client {
            url,
            prefix_url: parse_url(url),
            body: vec!["".into()],
            header_map: HeaderMap::new(),
        }
    }

    pub fn get_body_from_file(&self) {}
    // async
    pub async fn get_body(&mut self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();

        let response = client
            .get(self.url)
            .headers(self.header_map.clone())
            .send()
            .await?;
        if response.status().is_success() {
            let body = response.text().await?;
            let lines: Vec<_> = body
                .lines()
                .filter(|item| !item.starts_with("#") && item.contains(".ts"))
                .map(|line| line.to_owned())
                .collect();

            let modify_lines: Vec<_> = lines
                .iter()
                .enumerate()
                .map(|(index, _)| format!("file 'assets/{}.ts'", index))
                .collect();
            let text = modify_lines.join("\n");

            fs::write("./ffmpeg.txt", text)?;
            self.body = lines;
            Ok(())
        } else {
            println!("{:?}", response.error_for_status());
            Err("Failed to get body".into())
        }
    }

    pub async fn download(&self) -> Result<(), Box<dyn Error>> {
        let mut tasks = vec![];
        let body = self.body.clone();
        for (index, url) in body.into_iter().enumerate() {
            let format_url = format!("{}/{}", self.prefix_url, url);
            let headers = self.header_map.to_owned();
            let task = tokio::spawn(fetch_url(format_url, index, headers));
            tasks.push(task);
        }
        let mut success_count = 0;
        tokio::join!(async {
            for task in tasks {
                match task.await {
                    Ok(_) => success_count += 1,
                    Err(e) => {}
                }
            }
        });
        if success_count == self.body.len() {
            execute_ffmpeg();
        } else {
            println!("文件未下载完整: {}/{}", success_count, self.body.len())
        }
        Ok(())
    }
    fn check_files_all_downloaded() {}
}

fn parse_url(url: &str) -> &str {
    let prefix_url = match url.rfind("/") {
        Some(index) => url.split_at(index).0,
        None => "",
    };
    prefix_url
}

fn convert_to_reqwest_headers<'a>(headers: &'a HashMap<&'static str, &'static str>) -> HeaderMap {
    let mut reqwest_headers = HeaderMap::new();

    for (key, value) in headers {
        let header_value = HeaderValue::from_str(value).expect("Invalid header value");
        reqwest_headers.insert(*key, header_value);
    }

    reqwest_headers
}

async fn fetch_url(
    url: String,
    index: usize,
    headers: HeaderMap,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let file_path = format!("./assets/{}.ts", index);
    // println!("file path:{}", file_path);
    let metadata = Path::new(&file_path);
    // 如果文件已存在
    if metadata.exists() {
        // 检测文件是否正常
        let output = Command::new("ffmpeg")
            .args(&vec!["-v", "error", "-i", &file_path, "-f", "null", "-"])
            .output()
            .unwrap();
        if output.status.success() {
            // println!("文件正常");
            return Ok(());
        } else {
            println!("{file_path} is valid");
            fs::remove_file(&file_path).unwrap();
        }
        return Ok(());
    }

    // 请求重试次数
    let attempts = 5;
    // 发起请求
    let client = reqwest::Client::new();
    // client.
    // client.request(method, url)

    for attempt in 1..attempts {
        match client.get(&url).headers(headers.clone()).send().await {
            Ok(response) => {
                let body = response.bytes().await?;
                match fs::write(&file_path, body) {
                    Ok(_) => {
                        println!("写入{}.ts文件成功", index);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("写入文件失败:{}", e)
                    }
                };
            }
            Err(err) => {
                eprintln!("请求失败: {}，重试次数: {}/{}", err, attempt, attempts);
                if attempt < attempts {
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    Err("Failed to download ".into())
}

pub fn execute_ffmpeg() {
    let output = Command::new("ffmpeg")
        .args(&["-f", "concat", "-i", "ffmpeg.txt", "-c", "copy", "742.mkv"])
        .output()
        .expect("无法执行命令");
    // 检查命令执行是否成功
    if output.status.success() {
        println!("命令执行成功");
    } else {
        println!(
            "命令执行失败: {:?}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{self, Path},
        process::Command,
    };

    use super::*;

    #[test]
    fn test_ffmpeg() {
        execute_ffmpeg();
    }
    async fn downloader_executer() -> Result<(), Box<dyn Error>> {
        let url = "https://example.com/index.m3u8";
        let mut client = Client::new(url);
        client.get_body().await?;
        client.download().await?;

        Ok(())
    }
    #[test]
    fn test_downloader() {
        downloader_executer();
    }
}
