
extern crate nccl;

use nccl::Value;

fn main() {
    let n: Vec<i64> = vec![1, 2, 3, 4, 5];
    let v = Value::from(n);
    println!("{:?}", v);
}

