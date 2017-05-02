
extern crate nccl;

use nccl::Value;

#[test]
fn from_vec() {
    assert_eq!(Value::from(vec![1, 2, 3]),
               Value::List(vec![Value::Integer(1),
                                Value::Integer(2),
                                Value::Integer(3)]));
}

