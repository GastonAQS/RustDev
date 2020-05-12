
extern crate a_o_c_1;

use a_o_c_1::MyThreadpool;

fn main() {
    let pool = MyThreadpool::new();
    pool.execute();
}

