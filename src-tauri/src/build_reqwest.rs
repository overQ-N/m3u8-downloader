use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderValue};

#[derive(PartialEq, Eq, Hash)]
pub enum Key {
    Curl,
    Url,
}

#[derive(Debug, PartialEq)]
pub struct BuildReqwest {
    pub url: String,
    pub header_map: HeaderMap,
}

impl BuildReqwest {
    /** 解析 curl */
    pub fn from_curl(content: &str) -> BuildReqwest {
        let mut url = String::new();
        let mut headers = HashMap::new();
        for line in content.lines() {
            let part = extract_single_quote_content(line);
            if part.starts_with("https") || part.starts_with("http") {
                url = part.to_string();
            } else if part.contains(":") {
                if let Some((name, value)) = extract_header(&part) {
                    headers.insert(name, value);
                }
            }
        }
        // hashmap convert to headermap
        let mut header_map = HeaderMap::new();
        for (key, value) in headers {
            let header_value = HeaderValue::from_str(&value).expect("Invalid header value");
            let k: &str = Box::leak(key.into_boxed_str());
            header_map.insert(k, header_value);
        }
        BuildReqwest { url, header_map }
    }
}

impl From<HashMap<Key, &str>> for BuildReqwest {
    fn from(map: HashMap<Key, &str>) -> Self {
        if let Some(value) = map.get(&Key::Curl) {
            return BuildReqwest::from_curl(value);
        }
        BuildReqwest {
            url: "".into(),
            header_map: HeaderMap::new(),
        }
    }
}

//　获取单引号内的内容
fn extract_single_quote_content(input: &str) -> String {
    let mut results = "".into();
    let mut is_inside_quote = false;
    let mut start_index = 0;

    for (i, c) in input.char_indices() {
        if c == '\'' {
            if is_inside_quote {
                results = input[start_index..i].to_string();
                return results;
                // is_inside_quote = false;
            } else {
                is_inside_quote = true;
                start_index = i + 1;
            }
        }
    }

    results
}

/** 解析头部 */
fn extract_header(header: &str) -> Option<(String, String)> {
    let parts: Vec<_> = header.split(":").collect();
    if parts.len() == 2 {
        let name = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();
        Some((name, value))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form() {
        let mut form = HashMap::new();
        form.insert(Key::Curl, "curl 'https://api-iam.intercom.io/messenger/web/ping' \
        -H 'authority: api-iam.intercom.io' \
        -H 'accept: */*' \
        -H 'accept-language: zh-CN,zh-TW;q=0.9,zh;q=0.8,en-US;q=0.7,en;q=0.6' \
        -H 'content-type: application/x-www-form-urlencoded' \
        -H 'origin: https://beta-customeow.maiyuan.online' \
        -H 'sec-ch-ua: \"Google Chrome\";v=\"119\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\"' \
        -H 'sec-ch-ua-mobile: ?0' \
        -H 'sec-ch-ua-platform: \"macOS\"' \
        -H 'sec-fetch-dest: empty' \
        -H 'sec-fetch-mode: cors' \
        -H 'sec-fetch-site: cross-site' \
        -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
        --data-raw 'app_id=ibk0c9i6&v=3&g=129b538905ec8bf9557e82c6ee9631212f17a10f&s=23a13f1d-dffd-47cb-aacc-4740e0a201f7&r=&platform=web&Idempotency-Key=2f7d4d5ed9b1eaaf&internal=%7B%7D&is_intersection_booted=false&page_title=CustoMeow%20-%20A%20better%20way%20build%20custom%20features&user_active_company_id=undefined&user_data=%7B%22anonymous_id%22%3A%22864a6f82-d22d-40be-967b-34d53935ceb0%22%7D&source=apiBoot&sampling=false&referer=https%3A%2F%2Fbeta-customeow.maiyuan.online%2Fcustom&anonymous_session=VXZhMUtLcHpqQmJUNVNxVS9pQXVzVW5Bemd6eVF6UVVXYnoyUVh4dTZIR0VDbnUxTW5xTi9kc1lCUXFiRWhscy0tcGNIc1krbkZwMCtZUHpaV3c4V2laUT09--9aaa08789e710eb4a9cf5463d3a355e42819e1a5&device_identifier=fa2f7e4e-4493-454a-8446-cf5ccb5f9476' \
        --compressed");

        let expect_url: String = "https://api-iam.intercom.io/messenger/web/ping".into();
        // let b = form.get(&Key::Curl).unwrap();

        // let expect_headers =
        let build_reqwest: BuildReqwest = form.into();
        // println!("{:?}", b);
        assert_eq!(expect_url, build_reqwest.url);
    }
}
