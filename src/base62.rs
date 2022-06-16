use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, DeserializeFromStr, SerializeAs, SerializeDisplay};

#[derive(Clone, PartialEq, SerializeDisplay, DeserializeFromStr)]
pub struct Base62Encoded<T>(T);

impl<T> std::fmt::Display for Base62Encoded<T>
where
    T: Clone + Into<u128>,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&base62::encode(self.0.clone()))
    }
}

impl<T> FromStr for Base62Encoded<T>
where
    u128: TryInto<T>,
{
    type Err = base62::DecodeError;

    fn from_str(other: &str) -> Result<Self, Self::Err> {
        match base62::decode(other)?.try_into() {
            Ok(n) => Ok(Base62Encoded(n)),
            Err(_) => Err(Self::Err::ArithmeticOverflow),
        }
    }
}

impl<T> SerializeAs<T> for Base62Encoded<T>
where
    T: Clone + Into<u128>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Base62Encoded(source.clone()).serialize(serializer)
    }
}

impl<'de, T> DeserializeAs<'de, T> for Base62Encoded<T>
where
    u128: TryInto<T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        Base62Encoded::deserialize(deserializer).map(|this| this.0)
    }
}
