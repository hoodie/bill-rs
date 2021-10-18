use ordered_float::OrderedFloat;
#[cfg(feature = "serialization")]
use serde::ser::{Serialize, Serializer};

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::convert;
use std::fmt;
use std::ops::Deref;

/// Representation of Tax value
#[derive(Copy, Clone, Debug)]
pub struct Tax(OrderedFloat<f64>);

impl Tax {
    pub fn new(value: f64) -> Self {
        Tax(OrderedFloat(value))
    }

    pub fn value(&self) -> f64 {
        *self.0.as_ref()
    }
}

#[cfg(feature = "serialization")]
impl Serialize for Tax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.as_ref().to_string().serialize(serializer)
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Tax> for f64 {
    fn from(val: Tax) -> Self {
        val.0.into_inner()
    }
}

impl convert::From<f64> for Tax {
    fn from(value: f64) -> Tax {
        Tax::new(value)
    }
}
