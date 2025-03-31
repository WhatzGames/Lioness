use std::borrow::Cow;

#[cfg(feature = "http")]
impl From<http::Uri> for InternalUri<'_>{
    fn from(value: http::Uri) -> Self {
        Self::from(value.to_string())
    }
}


impl <'a:'b,'b> From<&'a str> for InternalUri<'b>{
    fn from(value: &'a str) -> Self {
        Self {
            address: Cow::Borrowed(value),
        }
    }
}

impl From<String> for InternalUri<'_>{
    fn from(value: String) -> Self {
        Self{
            address: Cow::Owned(value)
        }
    }
}




pub(super) struct NoUri;
pub struct InternalUri<'a>{
    address: Cow<'a, str>,
}

