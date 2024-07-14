use serde::{Deserialize, Serialize};

use crate::gen_deserialize_children_list;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "webSettings")]
pub struct WebSettings {
    #[serde(default, deserialize_with = "deserialize_children_container_list")]
    pub divs: Vec<Div>,
}

gen_deserialize_children_list!(Div, "div");

impl WebSettings {
    pub fn new() -> WebSettings {
        Default::default()
    }
}

impl Default for WebSettings {
    fn default() -> Self {
        Self { divs: vec![] }
    }
}
