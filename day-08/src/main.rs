mod task1;
mod task1_no_ref_cell;
mod task2;
fn main() {
    task1::run().unwrap();
    task2::run().unwrap();
    task1_no_ref_cell::run().unwrap();
}
