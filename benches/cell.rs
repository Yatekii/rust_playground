#[macro_use]
extern crate criterion;

use criterion::{ Criterion, Fun };

use std::cell::Cell;

trait CellCopy<T> {
    fn copy(&self) -> Cell<T>;
}

impl<T> CellCopy<T> for Cell<T> where T: Copy + Default {
    fn copy(&self) -> Cell<T> {
        let v = self.take();
        let v2 = v.clone();
        self.set(v);
        Cell::new(v2)
    }
}

fn cell_take_put_back() -> Cell<u64> {
    let c = Cell::new(5);
    let v = c.take();
    let v2 = v.clone();
    c.set(v);
    Cell::new(v2)
}

fn cell_take_put_back_trait() -> Cell<u64> {
    let c = Cell::new(5);
    c.copy()
}

fn cell_copy() -> Cell<u64> {
    let c = Cell::new(5);
    let v = c.get();
    let v2 = v.clone();
    Cell::new(v2)
}

fn criterion_benchmark(c: &mut Criterion) {
    let c_take_put_back = Fun::new("Take & Put back", |b, _| b.iter(|| cell_take_put_back()));
    let c_take_put_back_trait = Fun::new("Take & Put back with trait", |b, _| b.iter(|| cell_take_put_back_trait()));
    let c_copy = Fun::new("Copy", |b, _| b.iter(|| cell_copy()));
    let functions = vec!(c_take_put_back, c_take_put_back_trait, c_copy);
    c.bench_functions("Cell", functions, 20);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);