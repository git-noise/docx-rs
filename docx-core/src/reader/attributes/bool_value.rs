use serde::{
    de::{self, Visitor},
    Deserializer,
};
use xml::attribute::OwnedAttribute;

pub fn is_false(v: &str) -> bool {
    v == "0" || v == "false"
}

pub fn read_bool(attrs: &[OwnedAttribute]) -> bool {
    if let Some(v) = attrs.get(0) {
        if is_false(&v.value) {
            return false;
        }
    }
    true
}

pub(crate) fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolVisitor;

    impl<'de> Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a boolean, 'true', 'false', '1', or '0'")
        }

        fn visit_bool<E>(self, value: bool) -> Result<bool, E>
        where
            E: de::Error,
        {
            Ok(value)
        }

        fn visit_str<E>(self, value: &str) -> Result<bool, E>
        where
            E: de::Error,
        {
            match value {
                "true" | "1" => Ok(true),
                "false" | "0" => Ok(false),
                _ => Err(E::custom(format!("invalid value for boolean: {}", value))),
            }
        }
    }

    deserializer.deserialize_any(BoolVisitor)
}
