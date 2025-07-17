use std::ops::{Add, Mul};

/// Computes the dot product of two vectors.
fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot_product(){
	assert_eq!(dot(&[1,2,3,4], &[1,1,1,1]), 10);
	assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0)
}

pub fn calculate_dot_product() {
	let v1 = [1.0, 2.0, 3.0];
	let v2 = [4.0, 5.0, 6.0];
	let result = dot(&v1, &v2);
	println!("Dot product of {:?} and {:?} is {}", v1, v2, result);
}
