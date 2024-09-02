use ndarray::{Array2, Axis};

pub(crate) fn array2_to_vec<T: Clone>(array2: Array2<T>) -> Vec<Vec<T>> {
    let mut v = Vec::<Vec<T>>::new();
    for i in array2.axis_iter(Axis(0)) {
        let mut sub_v = Vec::<T>::new();
        for e in i {
            sub_v.push(e.to_owned());
        }
        v.push(sub_v);
    }

    v
}
