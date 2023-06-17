use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Body {
    inner: Option<Vec<u8>>,
}

impl Body {
    #[inline]
    pub fn empty() -> Self {
        Self { inner: None }
    }

    #[inline]
    pub fn serialize<T: Serialize>(val: &T) -> Self {
        Self {
            inner: Some(rmp_serde::to_vec(&val).unwrap()),
        }
    }

    #[inline]
    pub fn deserialize<'a, T: Deserialize<'a>>(&'a self) -> Result<T, rmp_serde::decode::Error> {
        rmp_serde::from_slice(self.try_as_bytes().expect("body is empty"))
    }

    #[inline]
    pub fn try_as_bytes(&self) -> Option<&[u8]> {
        self.inner.as_deref()
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        Self { inner: Some(bytes) }
    }
}
