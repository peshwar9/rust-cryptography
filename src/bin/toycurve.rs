extern crate num_bigint;
use num_bigint::{BigInt, Sign};

struct Point {
    x: Option<BigInt>,
    y: Option<BigInt>,
}

struct EllipticCurve {
    a: BigInt,
    b: BigInt,
}

impl EllipticCurve {
    fn is_point_on_curve(&self, point: &Point) -> bool {
        if let (Some(x), Some(y)) = (&point.x, &point.y) {
            let x_cubed = x.pow(3);
            let y_squared = y.pow(2);
            let a_times_x = &self.a * x;
            y_squared == x_cubed + &a_times_x + &self.b
        } else {
            true // The point at infinity is considered on the curve
        }
    }
}

fn main() {
    let curve = EllipticCurve {
        a: BigInt::from(2),
        b: BigInt::from(4),
    };

    let point = Point {
        x: Some(BigInt::from(2)),
        y: Some(BigInt::from(4)),
    };

    println!("Is the point on the curve? {}", curve.is_point_on_curve(&point));
}
