mod lib_elliptic;

use lib_elliptic::elliptic;

fn main() {
    let g = elliptic::base_point_g_get();
    elliptic::print_ec_point(&g);
}
