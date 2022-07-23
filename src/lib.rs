/// Creates a HashMap containing the arguments.
///
#[macro_export]
macro_rules! hashmap {
    () => (
        std::collections::HashMap::new()
    );
    // match {key => val, ...}
    ({ $($tt:tt)* }) => (
        hashmap!($($tt)*)
    );
    // match (key, val), ...
    ($(($key:expr, $val:expr)$(,)?)+) => (
        hashmap!($($key => $val)*)
    );
    // match key => val, ...
    ($($key:expr => $val:expr$(,)?)+) => (
        {
            let mut hashmap = std::collections::HashMap::new();
            $(hashmap.insert($key, $val);)*

            hashmap
        }
    );
}
