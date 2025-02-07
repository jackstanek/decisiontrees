use std::hash::Hash;

pub trait FeatureVector: Clone {
    type Feature: Copy + Ord;
    type FeatureIdx: Copy + Eq + PartialEq + Hash;

    fn get_feature(&self, idx: Self::FeatureIdx) -> Self::Feature;
}

pub trait Label: Copy + Eq + Hash {}
impl Label for u64 {}

impl<T: Copy + Ord, U: Copy> FeatureVector for (T, T, U) {
    type Feature = T;
    type FeatureIdx = TwoTupleIdx;

    fn get_feature(&self, idx: Self::FeatureIdx) -> Self::Feature {
        match idx {
            TwoTupleIdx::X1 => self.0,
            TwoTupleIdx::X2 => self.1
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum TwoTupleIdx {
    X1,
    X2,
}