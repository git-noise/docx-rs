use crate::documents::{BuildXML, Level};
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractNumbering {
    id: usize,
    levels: Vec<Level>,
}

impl AbstractNumbering {
    pub fn new(id: usize) -> Self {
        Self { id, levels: vec![] }
    }

    pub fn add_level(mut self, level: Level) -> Self {
        self.levels.push(level);
        self
    }
}

impl BuildXML for AbstractNumbering {
    fn build(&self) -> Vec<u8> {
        let id = format!("{}", self.id);
        let mut b = XMLBuilder::new();
        b = b.open_abstract_num(&id);
        for l in &self.levels {
            b = b.add_child(l);
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use crate::documents::{Level, LevelJc, LevelText, NumberFormat, Start};
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_numbering() {
        let mut c = AbstractNumbering::new(0);
        c = c.add_level(Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%4."),
            LevelJc::new("left"),
        ));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:abstractNum w:abstractNumId="0"><w:lvl w:ilvl="1"><w:start w:val="1" /><w:numFmt w:val="decimal" /><w:lvlText w:val="%4." /><w:lvlJc w:val="left" /><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr></w:lvl></w:abstractNum>"#
        );
    }
}