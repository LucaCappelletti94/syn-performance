# Measuring the performance of Syn using different strategies

Small crate to measure the time requirements of syn parsing with different usages.

## Experiment

The experiment consist in generating the implementation of the following trait for some types:

```rust
/// Trait to measure the performance of syn parsing
pub trait Foo {
    /// Returns the number of attributes in the struct
    fn number_of_attributes() -> usize;
    /// Returns the number of fields in the struct
    fn number_of_optional_fields() -> usize;
}
```

were the abstract struct which will also be needed to implement will be defined as:

```rust
/// Attribute struct
pub struct Attribute {
    /// Name of the attribute
    pub name: String,
    /// Type of the attribute
    pub optional: bool,
}

let my_struct = vec![
    Attribute { name: "field1".to_string(), optional: false },
    Attribute { name: "field2".to_string(), optional: true },
    Attribute { name: "field3".to_string(), optional: false },
];
```

And measuring how much time the procedure generating the TokenStream takes. We implement the generation
procedure following different strategies:

1. **Quote, using Syn with full features**: The naive implementation generates the TokenStream by using the `quote!` macro to generate the
   implementation of the trait for each type. This is the most straightforward way to generate the code, but
   it is not the most efficient one. Specifically, we will enable all the features of Syn to see whether the
   performance is significantly affected by the features enabled. This implementation is found in the
   [`using-syn-full`](using-syn-full) crate.
2. **Quote, using Syn with minimal features**: The naive implementation generates the TokenStream by using the `quote!` macro to generate the
   implementation of the trait for each type. This time, we will enable only the features that are strictly
   necessary to generate the code. This implementation is found in the [`using-syn-partial`](using-syn-partial) crate.
3. **Manual generation**: The manual implementation generates the TokenStream by using the `TokenStream` API directly. A priori, we expect this
   implementation to be the most efficient one, as it avoids the overhead of the `quote!` macro. We still enable
   all the features of Syn to see whether the performance is significantly affected by the features enabled.
   This implementation is found in the [`using-no-parsing-full-syn`](using-no-parsing-full-syn) crate.
4. **Manual generation with minimal features**: In order to actually make use of all of the tokens necessary for the Manual generation,
   it is necessary to enable the `full` feature set of `syn`, and therefore we cannot, at this time, make a crate which uses
   the "minimal" features set.

## Results

In this section, we present the results of the experiment. We measure both the time necessary to build each of the
crates, the sizes of the generated binaries, and the time necessary to execute the code-generation code.
We use respectively `cargo build --timings`, `ls -lh target/debug/` and `cargo bench`. The benches are found
specifically in the [`syn-bencher/benches`](syn-bencher/benches) directory.

### Build times

TODO!

### Binary sizes

TODO!

### Execution times

TODO!
