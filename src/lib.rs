#![warn(missing_docs)]

//! Macro definition for creating HashMaps in the same style as the vec! macro for Vec.
//!
//! TODO: Example

/// Function for counting expressions.
#[doc(hidden)]
#[macro_export]
macro_rules! count_expr {
    ($_tt:tt $_expr:expr) => (());
    ($($expr:expr),*) => {<[()]>::len(&[$($crate::count_expr!($expr ())),*])};
}

/// Creates a HashMap containing the arguments.
/// TODO: Doc test
#[macro_export]
macro_rules! hashmap {
    () => (
        ::std::collections::HashMap::new()
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
            let num_elements = $crate::count_expr!($($key),*);
            let mut hashmap = ::std::collections::HashMap::with_capacity(num_elements);
            $(hashmap.insert($key, $val);)*

            hashmap
        }
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hashmap_arms() {
        let hashmap = hashmap!("foo" => 42, "bar" => 69, "baz" => 666);

        assert_eq!(hashmap["foo"], 42);
        assert_eq!(hashmap["bar"], 69);
        assert_eq!(hashmap["baz"], 666);
    }

    #[test]
    fn test_hashmap_scoped_arms() {
        let hashmap = hashmap!({"foo" => 42, "bar" => 69, "baz" => 666});

        assert_eq!(hashmap["foo"], 42);
        assert_eq!(hashmap["bar"], 69);
        assert_eq!(hashmap["baz"], 666);
    }

    #[test]
    fn test_hashmap_tuples() {
        let hashmap = hashmap!(("foo", 42), ("bar", 69), ("baz", 666));

        assert_eq!(hashmap["foo"], 42);
        assert_eq!(hashmap["bar"], 69);
        assert_eq!(hashmap["baz"], 666);
    }
}
