use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Base62(u64);

impl Base62 {
    pub fn new(number: u64) -> Self {
        Self(number)
    }
}

impl From<Base62> for u64 {
    fn from(other: Base62) -> u64 {
        other.0
    }
}

impl<S> From<S> for Base62
where
    S: AsRef<str>,
{
    fn from(other: S) -> Self {
        Self(base62::decode(other.as_ref()).unwrap() as u64)
        // Self(u64::from_be_bytes(
        //     base_62::decode(other.as_ref()).unwrap()[..8]
        //         .try_into()
        //         .unwrap(),
        // ))
    }
}

impl From<Base62> for String {
    fn from(other: Base62) -> String {
        other.to_string()
    }
}

impl std::fmt::Display for Base62 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&base62::encode(self.0))
        // formatter.write_str(&base_62::encode(&self.0.to_be_bytes()))
    }
}

impl Serialize for Base62 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Base62 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        se_de::deserialize(deserializer).map(Base62::new)
    }
}

pub mod se_de {
    use serde::{
        de::{self, Visitor},
        Deserializer, Serializer,
    };

    pub fn serialize<S>(subject: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base62::encode(*subject))
        // serializer.serialize_str(&base_62::encode(&subject.to_be_bytes()))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Base62Visitor;

        impl<'de> Visitor<'de> for Base62Visitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a base62-string encoded u64")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                base62::decode(value).map_err(E::custom).map(|x| x as u64)
                // base_62::decode(value)
                //     .map(|bytes| u64::from_be_bytes(bytes[..8].try_into().unwrap()))
                //     .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Base62Visitor)
    }
}
