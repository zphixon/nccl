
#[macro_export]
/// Calls .into() on every element added to a new vector.
///
/// Examples:
///
/// ```
/// # #[macro_use] extern crate nccl;
/// #[derive(Debug, PartialEq)]
/// struct Empty;
/// impl Into<Empty> for bool {
///     fn into(self) -> Empty {
///         Empty
///     }
/// }
///
/// impl Into<Empty> for i32 {
///     fn into(self) -> Empty {
///         Empty
///     }
/// }
///
/// impl<'a> Into<Empty> for &'a str {
///     fn into(self) -> Empty {
///         Empty
///     }
/// }
///
/// fn main() {
///     let v: Vec<Empty> = vec_into![true, 32, "hello"];
///     assert_eq!(v, vec![Empty, Empty, Empty]);
/// }
/// ```
///
macro_rules! vec_into {
    ($($item:expr),*) => {
        {
            let mut tmp = Vec::new();
            $(
                tmp.push($item.into());
            )*
            tmp
        }
    }
}

