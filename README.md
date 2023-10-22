# Just Another CLI Calculator

jacc is just another cli calculator for doing simple mathematics with
ease.

The language itself is quite simple, using mostly standard maths that
one should already know, plus a few extra for ease:

| Operator | Operation   |
| -------- | ----------- |
| `^`      | xor         |
| `&`      | bitwise and |
| `\|`     | bitwise or  |
| `**`     | exponent    |

In addition to these operators, there are a few functions built in, most
of them should be obvious by name:

| Function(s)                      | Description                                |
| -------------------------------- | ------------------------------------------ |
| `sin(x)`/`cos(x)`/`tan(x)`       | Find the sin/cos/tan of `x`                |
| `asin(x)`/`acos(x)`/`atan(x)`    | Find the arc sin/cos/tan of `x`            |
| `sinh(x)`/`cosh(x)`/`tanh(x)`    | Find the sinh/cosh/tanh of `x`             |
| `asinh(x)`/`acosh(x)`/`atanh(x)` | Find the arc sinh/cosh/tanh of `x`         |
| `ln(x)`/`log(x)`                 | Find the natural log or log base 10 of `x` |
| `log_BASE(x)`                    | Find the log using a base of BASE          |
| `log_B(x, base)`                 | Find the log using a base of `base`        |
| `sqrt(x)`/`cbrt(x)`              | Find the square or cube root of `x`        |
| `floor(x)`/`ceil(x)`/`round(x)`  | floor/ceil/round `x`                       |
| `abs(x)`                         | Find the absolute value of `x`             |
