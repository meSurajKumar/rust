# Rust Memory: Stack, Heap, Pointer, Length, Capacity

## Overview

When you create a `String` in Rust:

``` rust
let s = String::from("Rust");
```

The `String` is split into two parts:

-   **Stack**: Stores metadata (`ptr`, `len`, `capacity`)
-   **Heap**: Stores the actual characters

```{=html}
<!-- -->
```
    Stack                                Heap

    +-----------------------------+      +------------------+
    | ptr ------------------------+----->| R | u | s | t    |
    | len = 4                     |      +------------------+
    | capacity = 4                |
    +-----------------------------+

------------------------------------------------------------------------

## Stack

The stack stores small, fixed-size data.

Examples:

-   `i32`
-   `bool`
-   `char`
-   References (`&T`)
-   Metadata of `String` (`ptr`, `len`, `capacity`)

Stack is: - Very fast - Automatically cleaned up - Fixed size

Example:

``` rust
let x = 10;
let y = x;
```

`x` and `y` are copied because `i32` implements `Copy`.

------------------------------------------------------------------------

## Heap

The heap stores dynamically sized data.

Examples:

-   `String`
-   `Vec`
-   `HashMap`

Example:

``` rust
let s = String::from("Rust");
```

The actual text `"Rust"` is stored on the heap.

------------------------------------------------------------------------

## ptr (Pointer)

`ptr` stores the address of the first byte of the string on the heap.

Example:

    Heap

    Address      Value

    1000         R
    1001         u
    1002         s
    1003         t

Stack:

    ptr = 1000

------------------------------------------------------------------------

## len (Length)

`len` is the number of bytes currently used.

Example:

``` rust
let s = String::from("Rust");
```

    Length = 4

Example:

``` rust
let s = String::from("Hello");
```

    Length = 5

------------------------------------------------------------------------

## capacity

`capacity` is the amount of heap memory reserved.

Example:

    Heap

    +---+---+---+---+---+---+---+---+
    | R | u | s | t |   |   |   |   |
    +---+---+---+---+---+---+---+---+

    Length   = 4
    Capacity = 8

Only 4 bytes are used, but 8 bytes are reserved.

This avoids reallocating memory every time you append data.

------------------------------------------------------------------------

## Capacity Growth

``` rust
let mut s = String::with_capacity(10);

println!("{}", s.capacity()); // 10
```

After:

``` rust
s.push_str("Rust");
```

    Length   = 4
    Capacity = 10

If you exceed capacity:

``` rust
s.push_str(" Programming");
```

Rust allocates a larger block, copies the data, and frees the old block.

Example:

    Before

    Length   = 10
    Capacity = 10

    After

    Length   = 22
    Capacity = 32

(The exact growth strategy is implementation-dependent.)

------------------------------------------------------------------------

## Is Capacity Dynamic?

Yes.

Capacity can increase automatically at runtime when more space is
required.

Length changes based on the current data.

Capacity changes only when Rust needs more memory.

------------------------------------------------------------------------

## String Memory Layout

    Stack

    +--------------------------------------+
    | ptr | len = 4 | capacity = 8         |
    +--------------------------------------+
              |
              |
              V

    Heap

    +---+---+---+---+---+---+---+---+
    | R | u | s | t |   |   |   |   |
    +---+---+---+---+---+---+---+---+

------------------------------------------------------------------------

## Interview Answer

**Q: Is `String` stored on the stack or the heap?**

Answer:

> A `String` is split across both. The metadata (`ptr`, `len`,
> `capacity`) is stored on the stack, while the actual character data is
> stored on the heap.

------------------------------------------------------------------------

## Quick Summary

  Term       Meaning
  ---------- ---------------------------------------------
  Stack      Stores small fixed-size values and metadata
  Heap       Stores dynamically sized data
  ptr        Address of heap data
  len        Number of bytes currently used
  capacity   Number of bytes reserved on the heap
  Length     Changes with content
  Capacity   Grows when more memory is needed
