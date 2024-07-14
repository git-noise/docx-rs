use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::reader::deserialize_bool;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "commentEx", rename_all(serialize = "camelCase"))]
pub struct CommentExtended {
    #[serde(rename(deserialize = "@paraId", serialize = "paragraphId"))]
    pub paragraph_id: String,
    #[serde(rename(deserialize = "@done"), deserialize_with = "deserialize_bool")]
    pub done: bool,
    #[serde(rename(deserialize = "@paraIdParent"))]
    pub parent_paragraph_id: Option<String>,
}

impl CommentExtended {
    pub fn new(paragraph_id: impl Into<String>) -> CommentExtended {
        Self {
            paragraph_id: paragraph_id.into(),
            done: false,
            parent_paragraph_id: None,
        }
    }

    pub fn done(mut self) -> CommentExtended {
        self.done = true;
        self
    }

    pub fn parent_paragraph_id(mut self, id: impl Into<String>) -> CommentExtended {
        self.parent_paragraph_id = Some(id.into());
        self
    }
}

impl BuildXML for CommentExtended {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new()
            .comment_extended(&self.paragraph_id, self.done, &self.parent_paragraph_id)
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    #[test]
    fn test_comment_extended_json() {
        let ex = CommentExtended {
            paragraph_id: "00002".to_owned(),
            done: false,
            parent_paragraph_id: Some("0004".to_owned()),
        };
        assert_eq!(
            serde_json::to_string(&ex).unwrap(),
            r#"{"paragraphId":"00002","done":false,"parentParagraphId":"0004"}"#
        );
    }
}
