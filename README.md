# Bogie 🚃
![GitHub License](https://img.shields.io/github/license/owochle/bogie?style=for-the-badge)
[![Crates.io Version](https://img.shields.io/crates/v/bogie?style=for-the-badge)](https://crates.io/crates/bogie)

Bogie is an alternative to the default `Debug` derive which lacks a lot of features.

## Usage
To start using bogie, just derive `Debogue` on your structure, enumeration or unions. This will create the classical `Debug` implementation.
```rust
#[derive(Debogue)]
struct MyStruct {
    a: usize,
    b: usize,
    c: usize
}
```

But the strength of bogie is not there. It's in its attributes.
```rust
#[derive(Debogue)]
#[bogie(pub_only, hex)]
struct MyStruct {
    a: usize,
    #[bogie(skip)]
    pub b: usize,
    #[bogie(dbg)]
    pub c: usize,
    pub d: usize
}
```
In this example, when formatting this struct using its debug:
- `a` will not be printed, because of the `pub_only` attribute.
- `b` will not be printed, because of the `skip` attribute.
- `c` will be printed using its `Debug` implementation because of the `dbg` attribute.
- `d` will be printed using its `LowerHex` implementation because of the global `hex` attribute on the struct.

## Available attributes
### Common attributes (when present on type, will apply on all its fields)
- `dbg`: Use the `Debug` formatter.
- `display`: Use the `Display` formatter.
- `Hex`: Use the `UpperHex` formatter.
- `hex`: Use the `LowerHex` formatter.
- `oct`: Use the `Octal` formatter.
- `ptr`: Use the `Pointer` formatter.
- `Exp`: Use the `UpperExp` formatter.
- `exp`: Use the `LowerExp` formatter.
- `bin`: Use the `Binary` formatter.
- `empty`: Prints `()` instead of the content of this field. Useful for redacting tokens/secrets 
  or for fields not implementing any formatters.
- `fn(path)`: Use the provided path as formatter. 
  The target function must implement `Fn(&T, &mut Formatter<'_>) -> core::fmt::Result` when targeting a field of type `T`.

### Field attributes
- `skip`: Will skip this field in debug.

### Structure attributes
- `pub_only`: Will only print fields with public visibility.
- `enum_prefix` (Enum only): Will add the enum name as a prefix as type. i.e. `Type::Variant(field)` instead of `Variant(field)`.

## No STD Support
Although not tested, this crate should be able to support `no_std` environments.

## Etymology
Bogie is the wheel assembly of trains, phonetically close to *bogue* which is bug in French.

## License
MIT License Copyright (c) 2025 OwOchlé