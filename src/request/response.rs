use std::ops::{Deref, DerefMut};
use serde::{Serialize, Deserialize};

/// This type is a pair of a response's body bytes and the deserialized value.
/// It usually wraps a [`DataResponse`] or a [`PaginatedDataResponse`], for
/// instance when returned from a method in [`crate::official::endpoints`]. In
/// these cases, use the [`ApiDataResult`] and [`ApiPageResult`] aliases.
#[derive(Debug)]
pub struct ApiResponse<T> {
    pub(crate) bytes: Vec<u8>,
    pub(crate) value: T,
}

impl<T> ApiResponse<T> {
    /// Get an immutable borrow to the response's body bytes.
    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get a mutable borrow to the response's body bytes.
    pub fn get_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Get an immutable borrow to the value deserialized from bytes.
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// Get a mutable borrow to the value deserialized from bytes.
    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Take out the response's body bytes, discarding the deserialized data.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Take out the value deserialized from bytes, discarding those bytes.
    pub fn into_value(self) -> T {
        self.value
    }

    /// Take out both the bytes and the deserialized value as a tuple.
    pub fn into_bytes_value(self) -> (Vec<u8>, T) {
        (self.bytes, self.value)
    }
}

impl<T> Deref for ApiResponse<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for ApiResponse<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaginatedResponse<T> {
    pub hits: Vec<T>,
    pub offset: usize,
    pub limit: usize,
    pub total_hits: usize,
}

pub type ApiPageResult<T> = Result<ApiResponse<PaginatedResponse<T>>, crate::Error>;
