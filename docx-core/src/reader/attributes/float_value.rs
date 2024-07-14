use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_float<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let val: f32 = Deserialize::deserialize(deserializer)?;
    Ok(val as usize)
}
