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

fn entropy_binary<F: Copy + Ord>(features: &[F], thresh: F) -> f64 {
    let n_below = features.iter().filter(|x| *x < &thresh).count() as f64;
    let n_above = features.len() as f64;
    - (n_below * n_below.log2() + n_above * n_above.log2())
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
    for (lbl, fvecs) in by_label.iter() {
        let mut n_below = 0;
        let mut n_tot = 0;
        for fvec in fvecs.iter() {
            if fvec.get_feature(idx) < thresh {
                n_below += 1;
            }
            n_tot += 1;
        }

        let p_below = n_below as f64 / n_tot as f64;
        cond_ent += -(p_below * p_below.log2())
    }
    todo!()
}
