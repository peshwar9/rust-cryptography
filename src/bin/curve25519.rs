use elliptic_curve::{ Curve, point::AffinePoint};
use num_bigint::{BigInt, Sign};
use num_traits::One;

struct Point {
    x: BigInt,
    y: BigInt,
}

struct EllipticCurve {
    coeff_a: BigInt,
    coeff_b: BigInt,
    mod_p: BigInt,
}

impl EllipticCurve {
    fn is_point_on_curve(&self, point: &Point) -> bool {
        let x_squared = &point.x * &point.x % &self.mod_p;
        let y_squared = &point.y * &point.y % &self.mod_p;
        (y_squared + &self.coeff_a * &point.x + &self.coeff_b) % &self.mod_p == x_squared

    }
}

fn calculate_y(curve: &EllipticCurve, x: &BigInt) -> BigInt {
    // Simplified calculation of y for the given curve equation (y^2 = x^3 + ax + b)
    let x_cubed = x * x * x % &curve.mod_p;
    let ax = &curve.coeff_a * x % &curve.mod_p;
    let b_mod_p = &curve.coeff_b % &curve.mod_p;
    let y_squared = (x_cubed + ax + b_mod_p) % &curve.mod_p;

    // Calculate the square root of y^2 modulo p using a simple algorithm (not secure for real use)
    let y = y_squared.clone();
    let mut t = BigInt::one();
    while t < curve.mod_p {
        if (&t * &t) % &curve.mod_p == y {
            return t.clone();
        }
        t = &t + 1;
    }
    BigInt::from(0)
}

fn main() {
        // Define curve parameters for a simple example curve (y^2 = x^3 + ax + b)
let coeff_a = BigInt::from(1u32);
let coeff_b = BigInt::from(2u32);
let mod_p = BigInt::from(17u32);

// Generator point
let gen_x = BigInt::from(5u32); // X-coordinate of generator point G

// Create the generator point using curve parameters
let curve = EllipticCurve { coeff_a, coeff_b, mod_p };
let generator_point = Point {
    x: gen_x.clone(),
    y: calculate_y(&curve, &gen_x),
};

// Print the generator point
println!("Generator point is: ({},{})", gen_x, generator_point.y);

// Verify that the generator point is on the curve
assert!(curve.is_point_on_curve(&generator_point));



}


