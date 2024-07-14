/// Helper generating a custom deserializer function for a list of items in the form of a Vec.
///
/// It can generally be use with the `deserialize_with` serde features on struct that have
/// a list within container nesting patterns.
///
/// # Arguments
/// * `$type`: The type of the items in the list.
/// * `$name`: The field name in the serialized data contained by the list.
#[macro_export]
macro_rules! gen_deserialize_children_list {
    ($type:ty, $name:expr) => {
        fn deserialize_children_container_list<'de, D>(
            deserializer: D,
        ) -> Result<Vec<$type>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(serde::Deserialize)]
            struct Wrapper {
                #[serde(default, rename = $name)]
                inner: Vec<$type>,
            }

            let wrapper: Wrapper = serde::Deserialize::deserialize(deserializer)?;
            Ok(wrapper.inner)
        }
    };
}

/// Helper filtering Vec field of a struct, keeping last occurence ordered by a specific field.
///
/// It can generally be use with the `deserialize_with` serde features on struct that have
/// recursive nesting patterns.
///
/// # Arguments
/// * `$vec_field`: fully-qualified field to sort.
/// * `$field_name`: field name to consider for uniqueness
#[macro_export]
macro_rules! unique_filter_on_field {
    ($vec_field:expr, $field_name:ident) => {{
        use std::collections::BTreeMap;

        let mut temp_map: BTreeMap<_, _> = BTreeMap::new();
        // The expression now explicitly handles vectors directly
        for item in $vec_field.drain(..) {
            temp_map.insert(item.$field_name.clone(), item);
        }

        $vec_field.clear();
        $vec_field.extend(temp_map.into_iter().map(|(_key, value)| value));
    }};
}
