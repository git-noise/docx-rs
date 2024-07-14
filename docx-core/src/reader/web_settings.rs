use super::*;
use crate::{reader::ReaderError, unique_filter_on_field};

use quick_xml::{de::from_reader, Reader};
use std::io::BufRead;

impl FromXMLQuickXml for WebSettings {
    fn parse_xml<R: BufRead>(
        reader: &mut Reader<R>,
        _buf: &mut Vec<u8>,
    ) -> Result<WebSettings, ReaderError> {
        // Parse as WebSettings
        let mut web_settings: WebSettings =
            from_reader(reader.get_mut()).map_err(|e| ReaderError::QuickXMLError(e.to_string()))?;

        // Filter to keep only last extented div - identified by id.
        unique_filter_on_field!(&mut web_settings.divs, id);

        Ok(web_settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::Reader;
    use std::io::Cursor;

    fn setup_xml_reader(xml: &str) -> Reader<Cursor<&[u8]>> {
        let cursor = Cursor::new(xml.as_bytes());
        Reader::from_reader(cursor)
    }

    #[test]
    fn test_parse_xml_multiple_divs() {
        let xml = r#"
            <w:webSettings>
                <w:divs>
                    <w:div w:id="1" w:leftMargin="200" w:rightMargin="200" w:topMargin="100" w:bottomMargin="100" />
                    <w:div w:id="2" w:leftMargin="50" w:rightMargin="50" w:topMargin="50" w:bottomMargin="50" />
                </w:divs>
            </w:webSettings>
        "#;
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = WebSettings::parse_xml(&mut reader, &mut buf).unwrap();
        assert_eq!(result.divs.len(), 2);
        assert_eq!(result.divs[0].id, "1");
        assert_eq!(result.divs[1].id, "2");
    }

    #[test]
    fn test_parse_xml_duplicate_divs() {
        let xml = r#"
            <w:webSettings>
                <w:divs>
                    <w:div w:id="1" w:leftMargin="200" w:rightMargin="200" w:topMargin="100" w:bottomMargin="100" />
                    <w:div w:id="1" w:leftMargin="50" w:rightMargin="50" w:topMargin="50" w:bottomMargin="50" />
                </w:divs>
            </w:webSettings>
        "#;
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = WebSettings::parse_xml(&mut reader, &mut buf).unwrap();
        assert_eq!(result.divs.len(), 1);
        assert_eq!(result.divs[0].id, "1");
        assert_eq!(result.divs[0].margin_left, 50);
    }

    #[test]
    fn test_parse_xml_with_early_eof() {
        let xml = "<webSettings><w:divs><w:div w:id=\"1\" w:leftMargin=\"200\" />";
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = WebSettings::parse_xml(&mut reader, &mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_xml_error() {
        let xml = "<webSettings><w:div w:id=\"1\" w:leftMargin=\"200\" w:rightMargin=\"200\"";
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = WebSettings::parse_xml(&mut reader, &mut buf);
        assert!(result.is_err());
    }
}
