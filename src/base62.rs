use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Base62Uint(u64);

impl Base62Uint {
    pub fn new(number: u64) -> Self {
        Self(number)
    }
}

impl From<Base62Uint> for u64 {
    fn from(other: Base62Uint) -> u64 {
        other.0
    }
}

impl<S> From<S> for Base62Uint
where
    S: AsRef<str>,
{
    fn from(other: S) -> Self {
        Self(base62::decode(other.as_ref()).unwrap() as u64)
    }
}

impl From<Base62Uint> for String {
    fn from(other: Base62Uint) -> String {
        other.to_string()
    }
}

impl std::fmt::Display for Base62Uint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&base62::encode(self.0))
    }
}

impl Serialize for Base62Uint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Base62Uint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        se_de::deserialize(deserializer).map(Base62Uint::new)
    }
}

pub mod se_de {
    use serde::de::{self, Visitor};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(subject: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base62::encode(*subject))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Base62Visitor;

        impl<'de> Visitor<'de> for Base62Visitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a u64 encoded as a base-62 string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                base62::decode(value).map_err(E::custom).map(|x| x as u64)
            }
        }

        deserializer.deserialize_str(Base62Visitor)
    }
}
