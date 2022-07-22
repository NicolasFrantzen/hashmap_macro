# HashMap macro

This is an attempt to solve following exercise (for learning purpose):
https://exercism.org/tracks/rust/exercises/macros

## Usage example:
```
use hashmap_macro::hashmap;
use std::collections::HashMap;

let hashmap1 = hashmap!("foo" => 42, "bar" => 69);

let mut hashmap2 = HashMap::new();
hashmap.insert("foo", 42);
hashmap.insert("bar", 69);

assert_eq(hashmap1, hashmap2);
```
