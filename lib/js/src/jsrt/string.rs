use utils::flystring::FlyString;

use crate::gc::{GcCell, GcPointer, Heap, Trace};

#[derive(Debug, Clone)]
pub struct JsString {
    pub string: FlyString,
}

unsafe impl Trace for JsString {}
impl GcCell for JsString {}

impl JsString {
    pub fn new<T>(str: T) -> JsString
    where
        T: Into<FlyString>,
    {
        JsString { string: str.into() }
    }
}
