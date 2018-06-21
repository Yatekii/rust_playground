use ncollide2d::math::Point;
use ncollide2d::bounding_volume::AABB;
use ncollide2d::partitioning::DBVT;
use ncollide2d::partitioning::DBVTLeaf;
use ncollide2d::query::PointInterferencesCollector;

use rand::prelude::*;

use elapsed::measure_time;

pub fn dbvt_insert_remove_fast() {
    let mut idx_and_bounding_boxes: Vec<AABB<f64>> = vec![];
    let mut rng = thread_rng();

    let num_aabb = 1_000_000;

    for _i in 0..num_aabb {
        let x = rng.gen_range(-1000.0, 1000.0);
        let x2 = rng.gen_range(-1000.0, 1000.0);
        let y = rng.gen_range(-1000.0, 1000.0);
        let y2 = rng.gen_range(-1000.0, 1000.0);
        let aabb = AABB::new(
            Point::new(if x > x2 { x2 } else { x }, if y > y2 { y2 } else { y }),
            Point::new(if x > x2 { x } else { x2 }, if y > y2 { y } else { y2 })
        );
        idx_and_bounding_boxes.push(aabb.clone());
    }

    let mut dbvt = DBVT::new();
    let point = Point::new(0.0, 0.0);

    let mut to_remove = Vec::new();

    let (elapsed, hits) = measure_time(|| {
        for i in 0..num_aabb {
            let choices = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let mut rng = thread_rng();
            let leaf_id = dbvt.insert(DBVTLeaf::new(idx_and_bounding_boxes[i].clone(), i));
            if rng.choose(&choices).unwrap() == &6 {
                to_remove.push(leaf_id);
            }
        }

        let mut result = Vec::new();
        {
            let mut visitor = PointInterferencesCollector::new(&point, &mut result);
            dbvt.visit(&mut visitor);
        }
        result
    });

    println!("elapsed = {}, len = {}", elapsed, hits.len());

    let (elapsed, _) = measure_time(|| {
        for leaf_id in to_remove {
            dbvt.remove(leaf_id);
        }
    });

    println!("elapsed = {}", elapsed);

    let (elapsed, hits) = measure_time(|| {
        let mut result = Vec::new();
        {
            let mut visitor = PointInterferencesCollector::new(&point, &mut result);
            dbvt.visit(&mut visitor);
        }
        result
    });

    println!("elapsed = {}, len = {}", elapsed, hits.len());
    loop {}
}