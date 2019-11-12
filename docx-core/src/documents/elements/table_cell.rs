use super::{Paragraph, TableCellProperty};
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct TableCell {
    property: TableCellProperty,
    contents: Vec<TableCellContent>,
}

#[derive(Debug, Clone)]
pub enum TableCellContent {
    Paragraph(Paragraph),
}

impl TableCell {
    pub fn new() -> TableCell {
        let property = TableCellProperty::new();
        let contents = vec![];
        Self { property, contents }
    }

    pub fn add_paragraph(mut self, p: Paragraph) -> TableCell {
        self.contents.push(TableCellContent::Paragraph(p));
        self
    }
}

impl BuildXML for TableCell {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_table_cell().add_child(&self.property);
        for c in &self.contents {
            match c {
                TableCellContent::Paragraph(p) => b = b.add_child(p),
            }
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::super::*;
    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_cell() {
        let b = TableCell::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tc><w:tcPr><w:tcBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tcBorders></w:tcPr></w:tc>"#
        );
    }

    #[test]
    fn test_cell_add_p() {
        let b = TableCell::new()
            .add_paragraph(Paragraph::new().add_run(Run::new("Hello")))
            .build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:tc><w:tcPr><w:tcBorders><w:top w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:left w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:bottom w:val="single" w:sz="2" w:space="0" w:color="000000" /><w:insideH w:val="single" w:sz="2" w:space="0" w:color="000000" /></w:tcBorders></w:tcPr><w:p><w:pPr><w:pStyle w:val="Normal" /><w:rPr /></w:pPr><w:r><w:rPr /><w:t xml:space="preserve">Hello</w:t></w:r></w:p></w:tc>"#
        );
    }
}
