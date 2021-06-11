use crate::gc::{GcCell, GcPointer, Trace};

pub type RcString = GcPointer<String>;
impl GcCell for String {}

pub type RcFlyString = GcPointer<utils::flystring::FlyString>;

unsafe impl Trace for utils::flystring::FlyString {}
impl GcCell for utils::flystring::FlyString {}
