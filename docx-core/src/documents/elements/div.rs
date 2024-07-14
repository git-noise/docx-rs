use serde::{Deserialize, Serialize};

use crate::{deserialize_float, gen_deserialize_children_list};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "w:div")]
pub struct Div {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@leftMargin", deserialize_with = "deserialize_float")]
    pub margin_left: usize,
    #[serde(rename = "@rightMargin", deserialize_with = "deserialize_float")]
    pub margin_right: usize,
    #[serde(rename = "@topMargin", deserialize_with = "deserialize_float")]
    pub margin_top: usize,
    #[serde(rename = "@bottomMargin", deserialize_with = "deserialize_float")]
    pub margin_bottom: usize,
    #[serde(
        default,
        rename = "divsChild",
        deserialize_with = "deserialize_children_container_list"
    )]
    pub divs_child: Vec<Div>,
}

gen_deserialize_children_list!(Div, "div");

impl Default for Div {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            margin_left: 0,
            margin_right: 0,
            margin_top: 0,
            margin_bottom: 0,
            divs_child: Default::default(),
        }
    }
}

impl Div {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            ..Default::default()
        }
    }

    pub fn margin_left(mut self, s: usize) -> Self {
        self.margin_left = s;
        self
    }

    pub fn margin_right(mut self, s: usize) -> Self {
        self.margin_right = s;
        self
    }

    pub fn margin_top(mut self, s: usize) -> Self {
        self.margin_top = s;
        self
    }

    pub fn margin_bottom(mut self, s: usize) -> Self {
        self.margin_bottom = s;
        self
    }

    pub fn add_child(mut self, s: Div) -> Self {
        self.divs_child.push(s);
        self
    }
}
