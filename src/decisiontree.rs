use std::collections::HashMap;

use crate::feature::*;

/// Binary decision tree
pub struct DTree<T: Ord> {
    root: Box<DNode<T>>,
}

/// Interior node in a binary decision tree
struct DNode<T: Ord> {
    thresh: T,
    lchild: Box<DNode<T>>,
    rchild: Box<DNode<T>>,
}

impl<T: Ord> DTree<T> {}

/// Compute entropy of a
fn entropy_binary<F: FeatureVector, L: Label>(
    labeled_features: &[(F, L)],
    idx: F::FeatureIdx,
    thresh: F::Feature,
) -> f64 {
    let n_below = labeled_features
        .iter()
        .filter(|x| x.0.get_feature(idx) < thresh)
        .count() as f64;
    let n_above = labeled_features.len() as f64;
    -(n_below * n_below.log2() + n_above * n_above.log2())
}

/// Calculate information entropy of a collection of feature vectors given a binary threshold
fn info_gain_ratio_binary<F: FeatureVector, L: Label>(
    features: &[(F, L)],
    idx: F::FeatureIdx,
    thresh: F::Feature,
) -> f64 {
    let by_label: HashMap<L, Vec<F>> =
        features
            .iter()
            .map(|x| (x.1, x.0.clone()))
            .fold(HashMap::new(), |mut m, x| {
                m.entry(x.0).or_default().push(x.1);
                m
            });

    let mut cond_ent = 0.0;
    for fvecs in by_label.values() {
        let mut n_below = 0;
        let mut n_tot = 0;
        for fvec in fvecs.iter() {
            if fvec.get_feature(idx) < thresh {
                n_below += 1;
            }
            n_tot += 1;
        }

        let p_below = n_below as f64 / n_tot as f64;
        cond_ent -= p_below * p_below.log2();
    }
    cond_ent / entropy_binary(features, idx, thresh)
}

#[cfg(test)]
mod tests {
    use crate::feature::{FeatureVector, Label};

    use super::entropy_binary;

    impl FeatureVector for f64 {
        type Feature = f64;

        type FeatureIdx = usize;

        fn get_feature(&self, _: Self::FeatureIdx) -> Self::Feature {
            *self
        }
    }

    impl Label for i64 {}

    #[test]
    fn test_bernoulli_entropy() {
        let test: Vec<(f64, i64)> = (1..=10).map(|x| (x as f64 / 10.0, 1)).collect();
        assert_eq!(entropy_binary(&test, 0, 0.5), 0.5);
    }
}
