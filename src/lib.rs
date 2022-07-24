#![warn(missing_docs)]

//! Macro definition for creating HashMaps in the same style as the vec! macro for Vec.
//!
//! Following example shows usage:
//! ```
//! use hashmap_macro::hashmap;
//!
//! let hashmap1 = hashmap!("foo" => 42, "bar" => 69);
//! assert_eq!(hashmap1, std::collections::HashMap::from([("foo", 42), ("bar", 69)]));
//! ```

/// Macro for counting expressions.
#[doc(hidden)]
#[macro_export]
macro_rules! count_expr {
    ($_tt:tt $_expr:expr) => (());
    ($($expr:expr),*) => {<[()]>::len(&[$($crate::count_expr!($expr ())),*])};
}

/// Creates a HashMap containing the arguments.
/// The items are counted an the map is created with capacity of the element count.
///
/// Create a HashMap using arms syntax:
/// ```
/// # #[macro_use] extern crate hashmap_macro;
/// let hashmap = hashmap!("foo" => 42, "bar" => 69, "baz" => 666);
///
/// assert_eq!(hashmap["foo"], 42);
/// assert_eq!(hashmap["bar"], 69);
/// assert_eq!(hashmap["baz"], 666);
/// ```
/// or with block scope:
/// ```
/// # #[macro_use] extern crate hashmap_macro;
/// let hashmap = hashmap!({"foo" => 42, "bar" => 69, "baz" => 666});
///
/// assert_eq!(hashmap["foo"], 42);
/// assert_eq!(hashmap["bar"], 69);
/// assert_eq!(hashmap["baz"], 666);
/// ```
///
/// or with tuple syntax (similar to the the From trait):
/// ```
/// # #[macro_use] extern crate hashmap_macro;
/// let hashmap = hashmap!(("foo", 42), ("bar", 69), ("baz", 666));
///
/// assert_eq!(hashmap["foo"], 42);
/// assert_eq!(hashmap["bar"], 69);
/// assert_eq!(hashmap["baz"], 666);
/// ```
#[macro_export]
macro_rules! hashmap {
    () => (
        ::std::collections::HashMap::new()
    );
    // match {key => val, ...}
    ({ $($tt:tt)* }) => (
        hashmap!($($tt)*)
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
    // match (key, val), ...
    ($(($key:expr, $val:expr)$(,)?)+) => (
        ::std::collections::HashMap::from([$(($key, $val)),+])
    );
}
