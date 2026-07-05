# Rust Notes: Move vs Copy & Borrowing with References

## Move vs Copy

### Copy

```rust
let a=5;
let b=a;
```
- `i32`, `bool`, `char`, `&T` implement `Copy`.
- Value is copied.
- Both variables remain valid.

### Move

```rust
let s1=String::from("hello");
let s2=s1;
```
- `String` does not implement `Copy`.
- Ownership moves to `s2`.
- Heap data is **not** copied.
- `s1` becomes invalid.

## Borrowing with References

```rust
let s1=String::from("hello");
let len=calculate_length(&s1);
```

```rust
fn calculate_length(s:&String)->usize{
    s.len()
}
```

- `s1` is the owner.
- `s` is only a borrower.
- Ownership does not change.
- When the function ends, the reference is destroyed, **not** the heap data.
- Heap memory is freed only when the owner (`s1`) goes out of scope.

## Memory

```
Stack                     Heap

s1 (Owner) -----------> "hello"
   ^
   |
s (&String)
```

## Golden Rules

- Copy types -> Copy
- `String`, `Vec`, `HashMap` -> Move
- `&T` -> Borrow
- Only the owner frees heap memory.
