#[cfg(test)]
mod tests {
    use crate::Div;
    use quick_xml::de::from_str;

    #[test]
    fn test_deserialize_div_no_divschild() {
        let xml_data = r#"
        <w:div w:id="1" w:leftMargin="200" w:rightMargin="200" w:topMargin="100" w:bottomMargin="100" />
          "#;
        let expected = Div {
            id: "1".to_string(),
            margin_left: 200,
            margin_right: 200,
            margin_top: 100,
            margin_bottom: 100,
            divs_child: Default::default(),
        };
        let result: Result<Div, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_deserialize_div_nested_divschild() {
        let xml_data = r#"
        <w:div w:id="1" w:leftMargin="200" w:rightMargin="200" w:topMargin="100" w:bottomMargin="100">
            <w:divsChild>
            <w:div w:id="2" w:leftMargin="50" w:rightMargin="50" w:topMargin="50" w:bottomMargin="50" />
            </w:divsChild>
        </w:div>
        "#;
        let expected = Div {
            id: "1".to_string(),
            margin_left: 200,
            margin_right: 200,
            margin_top: 100,
            margin_bottom: 100,
            divs_child: vec![Div {
                id: "2".to_string(),
                margin_left: 50,
                margin_right: 50,
                margin_top: 50,
                margin_bottom: 50,
                divs_child: Default::default(),
            }],
        };

        let result: Result<Div, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_deserialize_div_invalid_attribute() {
        let xml_data = r#"
            <w:div w:id="1" w:leftMargin="200" w:non_existing="200" w:topMargin="100" w:bottomMargin="100"/>
          "#;
        let result: Result<Div, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_div_missing_attribute() {
        let xml_data = r#"
        <w:div w:leftMargin="200" w:rightMargin="200" w:topMargin="100" w:bottomMargin="100"/>
          "#;
        let result: Result<Div, quick_xml::DeError> = from_str(xml_data);
        assert!(result.is_err());
    }
}
