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

// TODO: Finish this later, for now use the crate
// pub mod coder {
//     //! Based on glowfall's Java implementation with some tweaks.
//     //! <https://github.com/glowfall/base62/blob/master/Base62.java>
//     //! <https://github.com/jxskiss/base62/blob/master/base62.go>

//     const BASE: usize = 62;
//     const COMPACT_MASK: u8 = 0xb00011110;
//     const MASK_5_BITS: u8 = 0xb00011111;
//     const MASK_6_BITS: u8 = 0xb00111111;

//     type Alphabet = [u8; BASE];
//     type Decoder = [u8; u8::MAX as usize];

//     static ALPHABET_ORDERED: Alphabet =
//         *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
//     static ALPHABET_REVERSED: Alphabet =
//         *b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

//     pub enum Error {
//         InvalidCharacter(char),
//     }

//     pub type Result<T> = std::result::Result<T, Error>;

//     pub struct Encoding {
//         pub encode: Alphabet,
//         pub decode: Decoder,
//     }

//     impl Encoding {
//         pub fn new(alphabet: Alphabet) -> Self {
//             assert_eq!(alphabet.len(), BASE);
//             assert!(!alphabet.contains(&b'\n'));
//             assert!(!alphabet.contains(&b'\r'));

//             let this = Encoding {
//                 encode: alphabet,
//                 decode: [u8::MAX; u8::MAX as usize],
//             };

//             for i in 0..alphabet.len() {
//                 this.decode[i] = i as u8;
//             }

//             this
//         }
//     }

//     fn encode()

//     // pub fn encode_str<S>(string: S, alphabet: Alphabet) -> u64
//     // where
//     //     S: AsRef<str>,
//     // {
//     //     encode_bytes(string.as_ref().bytes(), alphabet)
//     // }

//     // pub fn encode_bytes<I>(bytes: I, alphabet: Alphabet) -> u64
//     // where
//     //     I: IntoIterator<Item = u8>,
//     // {
//     //     let bytes = bytes.into_iter();
//     //     let encoding =
//     // }
// }
