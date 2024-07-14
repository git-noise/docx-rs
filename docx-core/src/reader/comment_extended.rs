#[cfg(test)]
mod tests {
    use crate::CommentExtended;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_comment_extended_valid() {
        let xml_data = r#"
              <commentEx paraId="123" done="true" />
          "#;
        let expected = CommentExtended {
            parent_paragraph_id: None,
            paragraph_id: "123".to_string(),
            done: true,
        };
        let result: Result<CommentExtended, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_deserialize_comment_extended_valid_with_parent_id() {
        let xml_data = r#"
              <commentEx paraId="123" done="true" paraIdParent="5698" />
          "#;
        let expected = CommentExtended {
            parent_paragraph_id: Some("5698".to_string()),
            paragraph_id: "123".to_string(),
            done: true,
        };
        let result: Result<CommentExtended, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_deserialize_comment_extended_invalid_attribute() {
        let xml_data = r#"
              <commentEx paragraphId="123" done="true" />
          "#;
        let result: Result<CommentExtended, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_comment_extended_missing_attribute() {
        let xml_data = r#"
              <commentEx done="true" />
          "#;
        let result: Result<CommentExtended, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_comment_extended_incorrect_boolean() {
        let xml_data = r#"
              <commentEx paraId="123" done="not_a_boolean" />
          "#;
        let result: Result<CommentExtended, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_err());
    }
}
