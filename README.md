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

You can find the `timings` reports in the [`timings`](timings) directory. Here follows the table illustrating the build
times for each of the crates, both in debug and release mode. Note that since the first `cargo build` command also loads
the documents in memory, the first run takes longer than the subsequent ones - for this reason, we have run the `cargo build --timings`
command twice for each crate. Between all of the `cargo build` runs, we have run `cargo clean` to ensure that the build times
are not affected by the previous builds, as otherwise the build times become extremely small as effectively there is nothing
more to be built. We only report the results of the second run, which is free of the overhead of loading the documents in memory.

| Crate | Debug (s) | Release (s) |
|-------|-----------|-------------|
| [`using-syn-full`](using-syn-full) | [0.9](timings/using-syn-full.debug.html) | [1.5](timings/using-syn-full.release.html) |
| [`using-syn-partial`](using-syn-partial) | [0.9](timings/using-syn-partial.debug.html) | [1.1](timings/using-syn-partial.release.html) |
| [`using-no-parsing-full-syn`](using-no-parsing-full-syn) | [0.9](timings/using-no-parsing-full-syn.debug.html) | [1.6](timings/using-no-parsing-full-syn.release.html) |

Surprisingly enough, the build times are not significantly different between the different strategies. Even more surprisingly, the crate with the largest build time requirements is the one that uses the `syn` APIs directly without use of the `quote!` macro.

### Binary sizes

In this section, we present the sizes of the generated binaries for each of the crates. The sizes are measured using the `ls -lh` command, for both the debug and release builds. No particular build flags are used to optimize for size.

| Crate | Size (debug) | Size (release) |
|-------|--------------|----------------|
| [`using-syn-full`](using-syn-full) | 793K | 272K |
| [`using-syn-partial`](using-syn-partial) | 793K | 273K |
| [`using-no-parsing-full-syn`](using-no-parsing-full-syn) | 1.5M | 387K |

Unexpectedly, we find again that the crate that uses the `syn` APIs directly without the `quote!` macro generates the largest binary. This is surprising, as we would expect the crate that uses the `quote!` macro to generate the largest binary, as it includes the `quote!` macro itself.

### Execution times

In this section, we present the execution times for the code-generation code. The benchmarks are run using the `cargo bench` command, and are based on criterion. The benchmarks are found in the [`syn-bencher/benches`](syn-bencher/benches) directory.

| Crate | Time (ms) |
|-------|-----------|
| [`using-syn-full`](using-syn-full) | 8.44 |
| [`using-syn-partial`](using-syn-partial) | 8.63 |
| [`using-no-parsing-full-syn`](using-no-parsing-full-syn) | 9.42 |

Again, unexpectedly, the crate that uses the `syn` APIs directly without the `quote!` macro generates the slowest code-generation code. This is surprising, as we would expect the crate that uses the `quote!` macro to generate the slowest code-generation code, as it includes the `quote!` macro itself.

## Conclusions

Our experiments compared different approaches to generating Rust code with Syn, measuring build times, binary sizes, and execution times. The results we obtained were somewhat unexpected.

Build times were similar across strategies, with manual `TokenStream` handling (`using-no-parsing-full-syn`) taking the longest, contradicting expectations. Binary size analysis showed that this approach also produced the largest binaries, despite assumptions that `quote!` would be more bloated.

Execution of `criterion`-based benchmarks revealed that manual `TokenStream` construction was the slowest, challenging the idea that avoiding `quote!` improves efficiency. Instead, manually managing tokens introduced overhead.

Overall, using `quote!` with Syn—even with all features—provides a balanced trade-off between build time, binary size, and runtime performance. Manual `TokenStream` handling does not offer clear advantages and may introduce inefficiencies. Future work should investigate why direct token manipulation results in larger, slower binaries and explore alternative optimizations.
