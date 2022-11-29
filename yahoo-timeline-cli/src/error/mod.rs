use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct HttpRequestError;
#[derive(Debug, Clone)]
pub struct DataParseError;
#[derive(Debug, Clone)]
pub struct UnknownError;

// HttpRequestError
impl fmt::Display for HttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "サーバーへのリクエスト時にエラーが発生しました")
    }
}
impl error::Error for HttpRequestError {}

// DataParseError
impl fmt::Display for DataParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "取得したhtmlの解析に失敗しました")
    }
}
impl error::Error for DataParseError {}

// UnknownError
impl fmt::Display for UnknownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "原因不明のエラーが発生しました")
    }
}
impl error::Error for UnknownError {}
