# Equa
A math based programming language written in rust.

---

# Datatypes
| name              | example / description                                 |
|-------------------|-------------------------------------------------------|
| number            | `1`, `2`, `1.5`, ...*any number*...                   |
| vector            | `[...]`                                               |
| set               | `{...}`                                               |
| tuple             | `(..., ...)`                                          |
| function          | *a callable functions that takes in arguments*        |
| foreign-function  | *a callable function that takes in arguments in rust* |

# Grammar
The grammar is just like math (even including the `|` symbol for absolute values).
But to make it more useful, there are more symbols:
`x +- y` - Plus-minus: returns a tuple with the values `x + y` and `x - y`
`x ++ y` - Concatinate: concatinates `x` with `y` (used for vectors or sets)
`x -- y` - Remove: removes `y` from `x` (used for sets)
`x % y` - Modulo: modulo `y` of `x`
`x :: y` - Immutable assignment: assigns `y` to `x` whhich cannpt be changed
`x := y` - Mutable assignment: assigns `y` to `x` which can be changed
`x -> y` - Function-Definition: returns a function with the body `y` and the parameter(s) `x`
`#x` - Amount: returns the amount of values in `x` if x is a vector, tuple or set