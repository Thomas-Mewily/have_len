Define `is_empty(&self)` and `is_not_empty(&self)` for container that implement HaveLen, such as vector, array, string...

```rust
use have_len::*;

assert_eq!([1, 2, 3].is_empty(), false);
assert_eq!([1, 2, 3].is_not_empty(), true);

let empty_array : [i32; 0] = []; 
assert_eq!(empty_array.is_empty(), true);
assert_eq!(empty_array.is_not_empty(), false);


assert_eq!("".is_empty(), true);
assert_eq!("hello".is_empty(), false);

assert_eq!("".is_not_empty(), false);
assert_eq!("hello".is_not_empty(), true);

assert_eq!("".is_empty(), !("".is_not_empty()));
```

```rust
pub trait HaveLen
{
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool { self.len() == 0 }
    fn is_not_empty(&self) -> bool { !self.is_empty() }
}
```