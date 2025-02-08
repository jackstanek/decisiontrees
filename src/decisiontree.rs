use std::collections::HashMap;
use std::hash::Hash;

use crate::feature::*;

/// Binary decision tree
pub struct DTree<T: FeatureVector, L: Label> {
    root: Box<DNode<T, L>>,
}

/// Interior node in a binary decision tree
enum DNode<T: FeatureVector, L: Label> {
    Internal {
        ftr_idx: T::FeatureIdx,
        thresh: T::Feature,
        lchild: Box<DNode<T, L>>,
        rchild: Box<DNode<T, L>>,
    },
    Leaf {
        label: L,
    },
}

impl<T: FeatureVector, L: Label> DTree<T, L> {
    pub fn build(features: &[impl FeatureVector]) -> Self {
        todo!()
    }

    fn determine_candidate_split(&self) {

    }

    pub fn decide(&self, test_inst: &T) -> L {
        let mut curr = &self.root;
        loop {
            match curr.as_ref() {
                DNode::Internal {
                    ftr_idx,
                    thresh,
                    lchild,
                    rchild,
                } => {
                    curr = if test_inst.get_feature(ftr_idx) < *thresh {
                        lchild
                    } else {
                        rchild
                    }
                }
                DNode::Leaf { label } => return *label,
            }
        }
    }
}

/// Compute entropy of some stream
fn entropy_binary<C: Copy + Eq + Hash>(items: impl Iterator<Item = C>) -> f64 {
    let item_counts: HashMap<C, usize> = items.fold(HashMap::new(), |mut m, i| {
        *m.entry(i).or_insert(0) += 1_usize;
        m
    });
    let total = item_counts.values().sum::<usize>() as f64;
    -item_counts.values().fold(0.0, |ent, count| {
        let p = *count as f64 / total;
        ent + p * p.log2()
    })
}

/// Calculate information entropy of a collection of feature vectors given a binary threshold
fn info_gain_ratio_binary<F: FeatureVector, L: Label>(
    features: &[F],
    labels: &[L],
    idx: F::FeatureIdx,
    thresh: F::Feature,
) -> f64 {
    let by_label: HashMap<L, Vec<F>> =
        features
            .iter()
            .zip(labels)
            .fold(HashMap::new(), |mut m, x| {
                m.entry(*x.1).or_default().push(x.0.clone());
                m
            });

    let mut cond_ent = 0.0;
    for fvecs in by_label.values() {
        let mut n_below = 0;
        let mut n_tot = 0;
        for fvec in fvecs.iter() {
            if fvec.get_feature(&idx) < thresh {
                n_below += 1;
            }
            n_tot += 1;
        }

        if n_tot == 0 {
            continue;
        }

        let p_below = n_below as f64 / n_tot as f64;
        cond_ent -= p_below * p_below.log2();
    }
    cond_ent / entropy_binary(labels.iter())
}

#[cfg(test)]
mod tests {
    use crate::{
        decisiontree::info_gain_ratio_binary,
        feature::{Feature, FeatureIdx, FeatureVector, Label},
    };

    use super::entropy_binary;

    impl Feature for f64 {}
    impl FeatureIdx for bool {
        fn enumerate() -> impl Iterator<Item = Self> {
            [false, true].into_iter()
        }
    }

    impl FeatureVector for f64 {
        type Feature = f64;
        type FeatureIdx = bool;

        fn get_feature(&self, _: &Self::FeatureIdx) -> Self::Feature {
            *self
        }
    }

    impl Label for i64 {}

    #[test]
    fn test_bernoulli_entropy() {
        let test: Vec<f64> = (1..=10).map(|x| (x as f64 / 10.0)).collect();
        assert_eq!(entropy_binary(test.iter().map(|x| *x < 0.55)), 1.0);
        assert_eq!(entropy_binary(test.iter().map(|_| true)), 0.0);
    }
}
