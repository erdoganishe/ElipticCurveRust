mod lib_elliptic;

use lib_elliptic::elliptic;
use num_bigint::BigInt;

use crate::lib_elliptic::elliptic::ECPoint;


fn main() {
    let curve_a = BigInt::from(-3);
    let curve_b = BigInt::from(7);
    let elliptic_curve = elliptic::EllipticCurve { a: curve_a, b: curve_b };

    let x_coord = BigInt::from(5);
    let y_coord = BigInt::from(6);
    let your_point = elliptic::ec_point_gen(&x_coord, &y_coord) ;

    let your_point_on_curve = elliptic_curve.is_on_curve_check(&your_point);
    println!("Is your point on curve: {}", your_point_on_curve);

    let point_sum = elliptic_curve.add_ec_points(&your_point, &ECPoint{x:BigInt::from(4), y:BigInt::from(7)});
    println!("Sum of points: ");
    point_sum.print_ec_point();

    let scalar = BigInt::from(78);
    let scalar_mult_result = elliptic_curve.scalar_mult(&scalar, &your_point);
    println!("Scalar multiplication result: ");
    scalar_mult_result.print_ec_point();

    let base_point_g = elliptic::base_point_g_get();
    println!("Base Point G: ");
    base_point_g.print_ec_point();

    let serialized_point = your_point.ec_point_to_string();
    println!("Serialized Point: {}", serialized_point);

    let deserialized_point = elliptic::string_to_ec_point(&serialized_point);
    println!("Deserialized Point: ");
    deserialized_point.print_ec_point();
}