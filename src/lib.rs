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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hashmap_tuple() {
        let mut example = HashMap::new();
        assert_eq!(hashmap!(), example);

        example.insert("foo", 1337);
        assert_eq!(hashmap!(("foo", 1337)), example);
        assert_eq!(hashmap!(("foo", 1337),), example);

        example.insert("bar", 42);
        assert_eq!(hashmap!(("foo", 1337), ("bar", 42)), example);
        assert_eq!(hashmap!(("foo", 1337), ("bar", 42),), example);
    }

    #[test]
    fn test_hashmap_arms() {
        let mut example = HashMap::new();
        assert_eq!(hashmap!(), example);

        example.insert("foo", 1337);
        assert_eq!(hashmap!("foo" => 1337), example);
        assert_eq!(hashmap!("foo" => 1337,), example);

        example.insert("bar", 42);
        assert_eq!(hashmap!("foo" => 1337, "bar" => 42), example);
        assert_eq!(hashmap!("foo" => 1337, "bar" => 42,), example);

        example.insert("baz", 666);
        assert_eq!(hashmap!({"foo" => 1337, "bar" => 42, "baz" => 666}), example);
    }

    #[test]
    fn test_hashmap_insert() {
        let mut example = HashMap::new();
        example.insert("bar", 69);

        let mut hm = hashmap!();
        hm.insert("bar", 69);
        assert_eq!(example, hm);
    }
}
