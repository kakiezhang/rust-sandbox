use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.real + rhs.real, self.imagine + rhs.imagine)
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.real + rhs.real, self.imagine + rhs.imagine)
    }
}

impl Add<i64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: i64) -> Self::Output {
        Complex::new(self.real + rhs as f64, self.imagine)
    }
}

fn main() {
    let c1 = Complex::new(2.0, 8.0);
    let c2 = Complex::new(12.1, 18.5);
    println!("c1 + c2 = {:?}", &c1 + &c2);
    println!("{:?}", c1);

    let c3 = Complex::new(10.0, 8 as f64);
    println!("c3 + 90 = {:?}", &c3 + 90);
    println!("{:?}", c3);
}
