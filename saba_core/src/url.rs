use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    // &mutはRustにおける可変参照を示すシンタックス
    // - 通常の参照（&）は値を変更できませんが、&mut を使えば可能
    // - &mut は借用先を排他的に操作できる参照を作る
    // Result<Self, String> とは、成功時はimpl Urlの構造体cloneを、失敗時はエラー文字列を返すことを表す
    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported.".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }

    // &selfをselfにしてしまうとselfの所有権がis_http関数に移動してしまい、その後の関数でselfが使用できなくなる
    fn is_http(&self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        return false;
    }

    // host文字列を返す
    fn extract_host(&self) -> String {
        /**
         * 1. trim_start_matchesで先頭からhttp://を削除
         * 2. splitnで"/"を区切り文字とし、最大で2つに分割
         * 3. collect()でsplitnの返すイテレーターをベクター変換している
         */
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        // ホストにポート番号を含むか":"を使用し判定し
        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(":") {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string()
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

         if url_parts.len() < 2 {
            return "".to_string()
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        if path_and_searchpart.len() < 2 {
            "".to_string()
        } else {
            path_and_searchpart[1].to_string()
        }
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> String {
        self.port.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }    
}
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let url = "http://example.com".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_with_port() {
        let url = "http://example.com:8888".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_with_path() {
        let url = "http://example.com/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_with_port_path() {
        let url = "http://example.com:8888/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_url_with_port_path_searchpart() {
        let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_localhost() {
        let url = "http://localhost:8000".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "localhost".to_string(),
            port: "8000".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com:8888/index.html".to_string();
        let expected = Err("Only HTTP scheme is supported.".to_string());
        assert_eq!(expected, Url::new(url).parse());
    }
}