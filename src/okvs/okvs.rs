use crate::data::point::Point;

pub trait OKVS {
    fn decode(&self, key: impl Into<u128>) -> Point;
}
