# bitmath_macros

Contains the utility proc macro `bitslice!` used with the `bitmath` crate.

### bitslice!()

Allows for taking subsets of `Bits` objects as a new `Bits`, with conventional bitwise syntax (e.g. 15:8 instead of 8..16)

```rs
let source = Bits::<16>::try_from("00001101 10110000").unwrap();
let middle_bits = bitslice!(source[12:4]);
println!("{}", middle_bits);
// Bits<8>{ 1101 1011 | dec 219/-37 | hex 0xdb/-0x25 }
```
