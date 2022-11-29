use crate::{config::Config, content::Content, error as AppError, types};

pub struct Request;

impl Request {
    fn get(url: &str) -> types::Result<String> {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(response.text()?);
                }
                return Err(AppError::HttpRequestError.into());
            }
            Err(_) => Err(AppError::HttpRequestError.into()),
        }
    }
}

#[cfg(test)]
mod Requestモジュールのテスト {
    use crate::{config::Config, request::Request};
    #[test]
    fn 成功した場合は文字列を返す() {
        let r = Request::get(Config::URL).unwrap();
        assert!(r.len() > 0);
    }

    #[test]
    fn 失敗した場合はエラーを返す() {
        let r = Request::get(&format!("{}_invalid", Config::URL));
        assert!(r.is_err());
    }
}
