use std::hash::Hash;

pub trait Feature: Copy + PartialOrd {}

pub trait FeatureVector: Clone {
    type Feature: Feature;
    type FeatureIdx: FeatureIdx;

    fn get_feature(&self, idx: &Self::FeatureIdx) -> Self::Feature;
}

pub trait FeatureIdx: Copy + Eq + PartialEq + Hash {
    fn enumerate() -> impl Iterator<Item = Self>;
}

pub trait Label: Copy + Eq + Hash {}
impl Label for u64 {}

impl<T: Feature, U: Copy> FeatureVector for (T, T, U) {
    type Feature = T;
    type FeatureIdx = TwoTupleIdx;

    fn get_feature(&self, idx: &Self::FeatureIdx) -> Self::Feature {
        match idx {
            TwoTupleIdx::X1 => self.0,
            TwoTupleIdx::X2 => self.1,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum TwoTupleIdx {
    X1,
    X2,
}

impl FeatureIdx for TwoTupleIdx {
    fn enumerate() -> impl Iterator<Item = Self> {
        [Self::X1, Self::X2].into_iter()
    }
}
