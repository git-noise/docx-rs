use std::io::BufRead;

use quick_xml::{de::from_reader, Reader};

use crate::unique_filter_on_field;

use super::*;

impl FromXMLQuickXml for CommentsExtended {
    fn parse_xml<R: BufRead>(
        reader: &mut Reader<R>,
        _buf: &mut Vec<u8>,
    ) -> Result<Self, ReaderError> {
        // Parse as commentsEx
        let mut comments_extended: CommentsExtended =
            from_reader(reader.get_mut()).map_err(|e| ReaderError::QuickXMLError(e.to_string()))?;

        // Filter to keep only last extented comment - identified by paragraph_id.
        unique_filter_on_field!(&mut comments_extended.children, paragraph_id);

        Ok(comments_extended)
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
    fn test_parse_xml_multiple_comments() {
        let xml = r#"
            <commentsEx>
                <commentEx paraId="1" done="true" />
                <commentEx paraId="2" done="false" />
            </commentsEx>
        "#;
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = CommentsExtended::parse_xml(&mut reader, &mut buf).unwrap();
        eprintln!("{:?}", result);
        assert_eq!(result.children.len(), 2);
        assert_eq!(result.children[0].paragraph_id, "1");
        assert_eq!(result.children[1].paragraph_id, "2");
    }

    #[test]
    fn test_parse_xml_duplicate_comments() {
        let xml = r#"
            <commentsEx>
                <commentEx paraId="1" done="true" />
                <commentEx paraId="1" done="false" />
            </commentsEx>
        "#;
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = CommentsExtended::parse_xml(&mut reader, &mut buf).unwrap();
        assert_eq!(result.children.len(), 1);
        assert_eq!(result.children[0].paragraph_id, "1");
        assert!(!result.children[0].done);
    }

    #[test]
    fn test_parse_xml_docx() {
        let xml = r#"
        <w15:commentsEx xmlns:wpc="http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas" xmlns:cx="http://schemas.microsoft.com/office/drawing/2014/chartex" xmlns:cx1="http://schemas.microsoft.com/office/drawing/2015/9/8/chartex" xmlns:cx2="http://schemas.microsoft.com/office/drawing/2015/10/21/chartex" xmlns:cx3="http://schemas.microsoft.com/office/drawing/2016/5/9/chartex" xmlns:cx4="http://schemas.microsoft.com/office/drawing/2016/5/10/chartex" xmlns:cx5="http://schemas.microsoft.com/office/drawing/2016/5/11/chartex" xmlns:cx6="http://schemas.microsoft.com/office/drawing/2016/5/12/chartex" xmlns:cx7="http://schemas.microsoft.com/office/drawing/2016/5/13/chartex" xmlns:cx8="http://schemas.microsoft.com/office/drawing/2016/5/14/chartex" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:aink="http://schemas.microsoft.com/office/drawing/2016/ink" xmlns:am3d="http://schemas.microsoft.com/office/drawing/2017/model3d" xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml" xmlns:w16cex="http://schemas.microsoft.com/office/word/2018/wordml/cex" xmlns:w16cid="http://schemas.microsoft.com/office/word/2016/wordml/cid" xmlns:w16="http://schemas.microsoft.com/office/word/2018/wordml" xmlns:w16se="http://schemas.microsoft.com/office/word/2015/wordml/symex" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:wpi="http://schemas.microsoft.com/office/word/2010/wordprocessingInk" xmlns:wne="http://schemas.microsoft.com/office/word/2006/wordml" xmlns:wps="http://schemas.microsoft.com/office/word/2010">
            <w15:commentEx w15:paraId="00000064" w15:done="0"/>
            <w15:commentEx w15:paraId="00000066" w15:paraIdParent="00000064" w15:done="0"/>
        </w15:commentsEx>
        "#;
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = CommentsExtended::parse_xml(&mut reader, &mut buf).unwrap();
        assert_eq!(result.children.len(), 2);
        assert_eq!(result.children[0].paragraph_id, "00000064");
        assert_eq!(
            result.children[1].parent_paragraph_id,
            Some("00000064".into())
        );
    }

    #[test]
    fn test_parse_xml_with_early_eof() {
        let xml = "<commentsEx><commentEx paraId=\"1\" done=\"true\" />";
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = CommentsExtended::parse_xml(&mut reader, &mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_xml_error() {
        let xml = "<commentsEx><commentEx paraId=\"1\" done=\"true\"></Comments";
        let mut reader = setup_xml_reader(xml);
        let mut buf = Vec::new();
        let result = CommentsExtended::parse_xml(&mut reader, &mut buf);
        assert!(result.is_err());
    }
}
