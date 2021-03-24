// from https://github.com/fraterenz/numpy-100/blob/master/100_Numpy_exercises.md
#![allow(dead_code)]
use ndarray::Array1;

fn ex3() -> Array1<u32> {
    // array![] cannot default with length I think
    Array1::default(10)
}

fn ex6() -> Array1<u32> {
    let mut a = Array1::zeros(10);
    a[4] = 1;
    a
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
}
