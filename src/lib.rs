// from https://github.com/fraterenz/numpy-100/blob/master/100_Numpy_exercises.md
#![allow(dead_code)]
use ndarray::prelude::*;

fn ex3() -> Array1<u32> {
    // array![] cannot default with length I think
    Array1::default(10)
}

fn ex6() -> Array1<u32> {
    let mut a = Array1::zeros(10);
    a[4] = 1;
    a
}

fn ex7() -> Array1<f32> {
    // must be float
    Array1::range(10., 49., 1.)
}

fn ex8<T: Clone>(a: Array1<T>) -> Array1<T>{
    a.slice(s![..;-1]).to_owned()
}

fn ex9() -> Array::<f32, Ix2> {
    // rust cannot create arbitrary shaped arrays, need to know the size at compile time
    Array::range(1., 10., 1.).into_shape((3,3)).unwrap()
}

fn ex10() -> Array::<f32, Ix2> {
    Array::eye(3)
}

fn ex11() -> Array::<f32, Ix2> {
    // rust cannot create arbitrary shaped arrays, need to know the size at compile time
    Array::eye(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex3() {
        let a = ex3();
        assert_eq!(10, a.len());
        assert_ne!(1, a.len());
    }

    #[test]
    fn test_ex6() {
        let a = ex6();
        assert_eq!(1, a.sum());
        assert_eq!(1, a[4]);
    }

    #[test]
    fn test_ex8() {
        let a = ex8(Array1::range(1., 5., 1.));
        assert_eq!(a, array![4., 3., 2., 1.]);
    }

    #[test]
    fn test_ex9() {
        let a = ex9();
        assert_eq!(
            a, 
            array![
                [1., 2., 3.], 
                [4., 5., 6.],
                [7., 8., 9.]
            ])
    }

    #[test]
    fn test_ex11() {
        let a = ex11();
        assert_eq!(
            a, 
            array![
                [1., 0., 0.], 
                [0., 1., 0.],
                [0., 0., 1.]
            ])
    }
}
