use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Neg};

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    /// Real portion of the complex number
    re: T,
    /// Imaginary portion of the complex number
    im: T,
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}


pub fn handle_complex() {
	let a = Complex { re: 1, im: 2 };
	let b = Complex { re: 3, im: 4 };
	// let c = Complex { re: 5, im: 6 };

	// Test addition
	assert_eq!(a + b, Complex { re: 4, im: 6 });
	
	// Test negation
	assert_eq!(-a, Complex { re: -1, im: -2 });

	// Test addition assignment
	let mut d = a;
	d += b;
	assert_eq!(d, Complex { re: 4, im: 6 });

	// Test equality
	assert!(a != b);
	assert!(a == Complex { re: 1, im: 2 });
}