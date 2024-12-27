
#### basic mutability

- if you want to create a variable that can be changed down the line, use `mut`
- without `mut`, you can still overshadow a variable

```rust
// NOT allowed
let x = 5;
x = 4;

// allowed
let mut x = 5;
x = 4;

// allowed
let x = 5;
let x = x - 1;
```

- code will never compile if trying to mutate a value that does not have the `mut` keyword in its `let` expression

#### constants

- define with  `const` 
- *constants* are values bound to a name – not allowed to change, just like immutable variables
- cannot use `mut` with constants (always immutable)

```rust
const MINUTES_IN_HOUR: u32 = 60;
```

- convention is to use all uppercase to denote a `const`

#### Shadowing & Scope

```rust
fn main() {
	let x = 5;
	{
		let x = x * 2 // 10
	}
	let x = x + 1 // 6
}
```


