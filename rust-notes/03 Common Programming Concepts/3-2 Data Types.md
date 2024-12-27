**Two data type subsets:**
1. [[3-2 Data Types#Scalar Types|Scalar]]
2. [[3-2 Data Types#Compound Types|Compound]]

Rust is *statically typed* – must know types of ALL vars at compile time.

When the compiler cannot infer the type of variable (due to many being possible), we must manually annotate the type:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

## Scalar Types

**Scalar Types** are single value types:
- integers
- floating-point numbers
- booleans
- characters

### Integer Types

- **signed**: `i..`
- **unsigned**: `u..`

Signed numbers are stored using [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation. Signed variants can store numbers $[-(2^{n-1}), 2^{n-1}-1]$ inclusive.
- i.e., an `i8` variable would have $n=8$, and could store numbers $[-(2^7), 2^7-1]$ or $-128, 127$

Unsigned variants can store $[0, 2^n-1]$.

---

Rust integer types default to `i32`. The primary situation in which to use `isize / usize` is when indexing some collection.

#### Integer Overflow

Trying to change a variable to a value outside its specified range, i.e., changing a `u8: [0, 255]` to `256` will result in *integer overflow*.

This will cause your code to *panic* at runtime (when a program exists with an error)

## Compound Types

*Compound types* can group multiple values into **one** type. Rust has only two **primitive compound** types:
1. tuples
2. arrays

### Tuple Types

