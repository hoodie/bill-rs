use ordered_float::OrderedFloat;

use std::fmt;
use std::cmp::{PartialOrd, Ord, PartialEq, Eq, Ordering};
use std::ops::Deref;

/// Representation of Tax value
#[derive(Copy, Clone, Debug)]
pub struct Tax(OrderedFloat<f64>);

impl Tax {
    pub fn new(float: f64) -> Self {
        Tax(OrderedFloat(float))
    }
}

impl Ord for Tax {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for Tax {}

impl PartialEq for Tax {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl PartialOrd for Tax {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Deref for Tax {
    type Target = OrderedFloat<f64>;
    fn deref(&self) -> &OrderedFloat<f64> {
        &self.0
    }
}

impl fmt::Display for Tax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}



