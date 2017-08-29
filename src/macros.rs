
#[macro_export]
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

