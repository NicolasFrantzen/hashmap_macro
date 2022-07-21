#![allow(unused_macros)]

use std::collections::HashMap;

macro_rules! hashmap {
    ($val:expr) => {
        println!("Creating a hasmap macro with {}", $val);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_empty() {
        //assert_eq!(hashmap!(), HashMap::new());
    }

    #[test]
    fn test_hashmap() {
        //hashmap!(1337);
    }
}
