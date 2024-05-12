
# aggregate-map
Easily collect a list of key-value pairs into a mapping of keys to collections of values in Rust.

 - [Documentation](https://docs.rs/aggregate-map/latest/aggregate_map/)
 - [Crate docs](https://docs.rs/crate/aggregate-map/latest)
 - [crates.io](https://crates.io/crates/aggregate-map)

If you have a set of data that you want to collect into a map, by default you'll only keep the last value in the data for that key. But what if you want instead to keep a collection of all the values for each key? Enter `AggregateMap`!

```rust
use std::collections::HashMap;
use aggregate_map::AggregateMap;
let data = [
    ("dog", "Terry"),
    ("dog", "Zamboni"),
    ("cat", "Jonathan"),
    ("dog", "Priscilla"),
];
let collected: AggregateMap<HashMap<_, Vec<_>>> = data.into_iter().collect();
let expected = HashMap::from([
    ("dog", vec!["Terry", "Zamboni", "Priscilla"]),
    ("cat", vec!["Jonathan"])
]);
assert_eq!(collected.into_inner(), expected);
```

See [docs](https://docs.rs/aggregate-map/latest/aggregate_map/) for more info and examples.