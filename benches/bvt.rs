#[macro_use]
extern crate criterion;

use criterion::{ Criterion, Fun };

#[macro_use]
extern crate ncollide2d;

use ncollide2d::math::Point;
use ncollide2d::bounding_volume::AABB;
use ncollide2d::partitioning::DBVT;
use ncollide2d::partitioning::DBVTLeaf;
use ncollide2d::query::PointInterferencesCollector;

#[macro_use]
extern crate rand;

use rand::prelude::*;

fn dbvt_insert_remove_fast() {
    let mut idx_and_bounding_boxes: Vec<AABB<f64>> = vec![];
    let mut rng = thread_rng();

    for i in 0..10000 {
        idx_and_bounding_boxes.push(AABB::new(
            Point::new(rng.gen_range(0.0, 10.0), rng.gen_range(0.0, 10.0)),
            Point::new(rng.gen_range(0.0, 10.0), rng.gen_range(0.0, 10.0))
        ))
    }

    let mut dbvt = DBVT::new();
    let point = Point::new(0.0, 0.0);

    for i in 0..10000 {
        dbvt.insert(DBVTLeaf::new(idx_and_bounding_boxes[i].clone(), i));
    }

    let mut result = Vec::new();
    {
        let mut visitor = PointInterferencesCollector::new(&point, &mut result);
        dbvt.visit(&mut visitor);
    }
    println!("{:?}", result);
}

fn criterion_benchmark(c: &mut Criterion) {
    let f_dbvt_insert_remove_fast = Fun::new("Insert and Remove nodes with high frequency", |b, _| b.iter(|| dbvt_insert_remove_fast()));
    let functions = vec!(f_dbvt_insert_remove_fast);
    c.bench_functions("Cell", functions, 20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);