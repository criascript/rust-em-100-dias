use std::borrow::Borrow;

use serde::de::DeserializeOwned;
use skyscraper::{
    html::{DocumentNode, HtmlDocument},
    xpath::Xpath,
};

pub trait GetFirstNode {
    fn get_first_node(&self, doc: &HtmlDocument) -> Option<DocumentNode>;

    fn get_first_text(&self, doc: &HtmlDocument) -> Option<String> {
        self.get_first_node(doc).and_then(|node| node.get_text(doc))
    }
}

impl GetFirstNode for Xpath {
    fn get_first_node(&self, doc: &HtmlDocument) -> Option<DocumentNode> {
        self.apply(doc).ok()?.pop()
    }
}

pub trait ParseJson {
    fn parse_json<T: DeserializeOwned>(&self) -> serde_json::Result<T>;
}

impl<S: Borrow<str>> ParseJson for S {
    fn parse_json<T: DeserializeOwned>(&self) -> serde_json::Result<T> {
        serde_json::from_str(self.borrow())
    }
}
