use crate::content::Content;
use crate::types::Result;
use scraper::{Html, Selector};
use std::io::prelude::*;

pub struct Parser {
    html: String,
}

impl Parser {
    fn new(html: String) -> Self {
        Self { html }
    }
    fn parse(&self) -> Result<Content> {
        todo!()
    }
}

#[cfg(test)]
mod parserモジュールのテスト {
    use std::{fs::File, io::Read};

    use super::*;
    #[test]
    fn データが正しいとき期待した値を取得できる() {
        todo!()
    }

    #[test]
    fn パーサーを試してみる() {
        let mut file =
            File::open("/workspace/yahoo-timeline-cli/src/parser/test-data/response.html")
                .expect("テストデータの読み込みに失敗しました");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("テストデータの読み込みに失敗しました");

        let fragment = Html::parse_fragment(contents.as_ref());
        let li_selector = Selector::parse("#commentTimeline ul li").unwrap();

        for li in fragment.select(&li_selector) {
            println!("testaaaaaaaa");
            println!("{}", li.value().name());
        }
    }
}
