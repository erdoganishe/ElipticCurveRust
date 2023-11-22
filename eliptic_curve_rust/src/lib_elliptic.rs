pub mod elliptic {
    extern crate num_bigint as bigint;
    use std::str::FromStr;
    use bigint::{BigUint, BigInt};
    use num_traits::{Zero, One};

    pub struct EllipticCurve {
        pub a: bigint::BigInt,
        pub b: bigint::BigInt, 
    }

    pub struct ECPoint {
        pub x: bigint::BigInt,
        pub y: bigint::BigInt,
    }
    
    impl EllipticCurve {

        pub fn is_on_curve_check(&self, point: &ECPoint) -> bool {
            let curve_equation = point.y.clone().pow(2) == point.x.clone().pow(3) +  &self.a * point.x.clone() + &self.b;

            curve_equation
        }

        pub fn add_ec_points(&self, p: &ECPoint, q: &ECPoint) -> ECPoint {
            // (a) If P = O, then P + Q = Q.
            if p.x.is_zero() && p.y.is_zero() {
                return ECPoint {x: q.x.clone(), y:q.y.clone()};
            }
            // (b) Otherwise, if Q = O, then P + Q = P.
            else if q.x.is_zero() && q.y.is_zero() {
                return ECPoint {x: p.x.clone(), y:p.y.clone()};
            }
            // (c) Otherwise, write P = (x1, y1) and Q = (x2, y2).
            else {
                // (d) If x1 = x2 and y1 = −y2, then P + Q = O.
                if p.x == q.x && p.y == &self.b - &q.y {
                    return ECPoint {
                        x: Zero::zero(),
                        y: Zero::zero(),
                    };
                } else {
                    // (e) Otherwise:
                    let mut lambda: BigInt;
                    // (e1) if P ≠ Q: λ = (y2 - y1) / (x2 - x1)
                    if (p.x != q.x) && (p.y != q.y) {
                        let numer = &q.y - &p.y;
                        let denom = &q.x - &p.x;
                        lambda = numer * denom.modpow(&(&self.b - 2), &self.b);
                        lambda = lambda % &self.b;
                    }
                    // (e2) if P = Q: λ = (3x12 + a) / 2y1
                    else {
                        let numer = 3u32 * &p.x.clone() * &p.x.clone() + &self.a;
                        let denom = 2u32 * &p.y.clone();
                        lambda = numer * denom.modpow(&(&self.b - 2u32), &self.b);
                        lambda = lambda % &self.b;
                    }
    
                    // (f) x3 = λ2 − x1 − x2
                    let x3 = lambda.clone().pow(2) - &p.x - &q.x;
                    // (f) y3 = λ(x1 − x3) − y1
                    let y3 = lambda.clone() * (&p.x - &x3) - &p.y;
    
                    // (g) P + Q = (x3, y3)
                    return ECPoint { x: x3 % &self.b, y: y3 % &self.b };
                }
            }
        }


    }
    
    

    // ECPoint creation
    pub fn ec_point_gen(x: &bigint::BigInt, y: &bigint::BigInt) -> ECPoint {
        ECPoint { x: x.clone(), y: y.clone() }
    }

    //G-generator receiving
    pub fn base_point_g_get() -> ECPoint {
        let x_coord: bigint::BigInt = FromStr::from_str("48439561293906451759052585252797914202762949526041747995844080717082404635286").unwrap();
        let y_coord: bigint::BigInt = FromStr::from_str("36134250956749795798585127919587881956611106672985015071877198253568414405109").unwrap();
    
        ECPoint{x: x_coord, y: y_coord}
    }

    //Print point
    pub fn print_ec_point(point: &ECPoint) {
        println!("X: {}", point.x);
        println!("Y: {}", point.y);
    }
}