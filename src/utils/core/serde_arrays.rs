use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize_256<S>(bytes: &Option<[u8; 256]>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match bytes {
        Some(arr) => serializer.serialize_some(&arr[..]),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_256<'de, D>(deserializer: D) -> Result<Option<[u8; 256]>, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Option<Vec<u8>> = Deserialize::deserialize(deserializer)?;
    match bytes {
        Some(v) if v.len() == 256 => {
            let mut arr = [0u8; 256];
            arr.copy_from_slice(&v);
            Ok(Some(arr))
        }
        Some(_) => Err(serde::de::Error::custom("Invalid length for [u8; 256]")),
        None => Ok(None),
    }
}
