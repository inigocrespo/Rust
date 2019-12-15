use std::ops::{Add, AddAssign, Neg};

#[derive(Debug)]
struct Comples<T> {
    re: T,
    im: T,
}

impl<T> Comples<T> {
    fn new(re: T, im: T) -> Comples<T> {
        Comples::<T> { re, im }
    }
}

impl<T> Add for Comples<T>
where
    T: Add<Output = T>,
{
    type Output = Comples<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Comples {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> AddAssign for Comples<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Comples<T>
where
    T: Neg<Output = T>,
{
    type Output = Comples<T>;

    fn neg(self) -> Self::Output {
        Comples {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> PartialEq for Comples<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

impl<T> Eq for Comples<T> where T: Eq {}

fn operator_overloading() {
    let mut a = Comples::new(1.0, 2.0);
    let mut b = Comples::new(3.0, 4.0);
    // println!("{:?}", a + b);
    //a += b;
    println!("{}", a == b);
}

fn main() {
    operator_overloading();
}
