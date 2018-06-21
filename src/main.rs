extern crate ncollide2d;

extern crate rand;

extern crate elapsed;

mod dbvt;

use dbvt::dbvt_insert_remove_fast;

fn main() {
    dbvt_insert_remove_fast();
}
