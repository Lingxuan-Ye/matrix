# matreex

A simple matrix implementation.

## Quick Start

### Addition

```rust
use matreex::matrix;

let lhs = matrix![[0, 1, 2], [3, 4, 5]];
let rhs = matrix![[5, 4, 3], [2, 1, 0]];

assert_eq!(lhs + rhs, matrix![[5, 5, 5], [5, 5, 5]]);
```

### Subtraction

```rust
use matreex::matrix;

let lhs = matrix![[0, 1, 2], [3, 4, 5]];
let rhs = matrix![[5, 4, 3], [2, 1, 0]];

assert_eq!(lhs - rhs, matrix![[-5, -3, -1], [1, 3, 5]]);
```

### Multiplication

```rust
use matreex::matrix;

let lhs = matrix![[0, 1, 2], [3, 4, 5]];
let rhs = matrix![[0, 1], [2, 3], [4, 5]];

assert_eq!(lhs * rhs, matrix![[10, 13], [28, 40]]);
```

## FAQs

### Why `matreex` instead of `matrix`?

Hmm ... Who knows? Could be a name conflict.
