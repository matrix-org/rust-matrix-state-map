# StateMap

A container for memory efficient handling of Matrix state maps.


### Usage

```rust
let mut state_map = StateMap::new();

state_map.insert("m.room.member", "@erikj:jki.re", 10);
assert_eq!(state_map.get("m.room.member", "@erikj:jki.re"), Some(10));
```
