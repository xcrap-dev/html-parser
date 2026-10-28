use napi_derive::napi;
use scraper::Html;
use std::rc::Rc;
use sxd_document::Package;

use crate::engines::{
    select_first_by_css, select_first_by_xpath, select_many_by_css, select_many_by_xpath,
};
use crate::types::{HTMLElement, QueryType, SelectFirstOptions, SelectManyOptions};

#[napi(js_name = "HTMLParser")]
pub struct HTMLParser {
    pub(crate) content: String,
    document: Option<Rc<Html>>,
    package: Option<Package>,
}

#[napi]
impl HTMLParser {
    #[napi(constructor)]
    pub fn new(content: String) -> Self {
        return HTMLParser {
            content,
            document: None,
            package: None,
        };
    }

    fn get_package(&mut self) -> &Package {
        if self.package.is_none() {
            let parsed = sxd_html::parse_html(&self.content);
            self.package = Some(parsed);
        }

        return self.package.as_ref().unwrap();
    }

    fn get_document(&mut self) -> &Rc<Html> {
        if self.document.is_none() {
            let parsed = Html::parse_document(&self.content);
            self.document = Some(Rc::new(parsed));
        }

        self.document.as_ref().unwrap()
    }

    #[napi]
    pub fn select_first(&mut self, options: SelectFirstOptions) -> Option<HTMLElement> {
        let query_config = options.query;

        match query_config.query_type {
            QueryType::CSS => {
                let document = self.get_document();
                return select_first_by_css(&document, query_config.query);
            }
            QueryType::XPath => {
                let package = self.get_package();
                return select_first_by_xpath(&package, query_config.query);
            }
        }
    }

    #[napi]
    pub fn select_many(&mut self, options: SelectManyOptions) -> Vec<HTMLElement> {
        let query_config = options.query;

        match query_config.query_type {
            QueryType::CSS => {
                let document = self.get_document();
                return select_many_by_css(&document, query_config.query, options.limit);
            }
            QueryType::XPath => {
                let package = self.get_package();
                return select_many_by_xpath(&package, query_config.query, options.limit);
            }
        }
    }
}
