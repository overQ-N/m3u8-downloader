use std::{collections::HashMap, fs};

pub struct Parser {}

impl Parser {
    // 解析 curl
    pub fn from_curl(&self, content: &str) -> (String, HashMap<String, String>) {
        let mut url = String::new();
        let mut headers = HashMap::new();
        for line in content.lines() {
            let part = extract_single_quote_content(line);
            if part.starts_with("https") || part.starts_with("http") {
                url = part.to_string();
            } else if part.contains(":") {
                if let Some((name, value)) = self.extract_header(&part) {
                    headers.insert(name, value);
                }
            }
        }
        (url, headers)
    }
    fn extract_header(&self, header: &str) -> Option<(String, String)> {
        let parts: Vec<_> = header.split(":").collect();
        println!("parts: {:?}", parts);
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            Some((name, value))
        } else {
            None
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_file() {
        let p = Parser {};
        let file = fs::read("./input.txt").unwrap();
        let content = String::from_utf8(file).unwrap();
        let (url, headers) = p.from_curl(&content);
        // assert_eq!(url,url)
    }
}
