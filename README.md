# Measuring the performance of syn using different strategies

Small crate to measure the time requirements of [`syn`](https://docs.rs/syn/latest/syn/) parsing with different usages.

## Experiment

The experiment consist in generating the implementation of the following trait for some types:

```rust
/// Trait to measure the performance of parsing
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

And measuring how much time the procedure generating the [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) takes. We implement the generation procedure following different strategies:

1. **Quote, using [`syn`](https://docs.rs/syn/latest/syn/) with full features**: The naive implementation generates the [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) by using the [`quote!`](https://docs.rs/quote/latest/quote/) macro to generate the
   implementation of the trait for each type. This is the most straightforward way to generate the code, but
   it is not the most efficient one. Specifically, we will enable all the features of [`syn`](https://docs.rs/syn/latest/syn/) to see whether the
   performance is significantly affected by the features enabled. This implementation is found in the
   [`using-syn-full`](using-syn-full) crate.
2. **Quote, using [`syn`](https://docs.rs/syn/latest/syn/) with minimal features**: The naive implementation generates the [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) by using the [`quote!`](https://docs.rs/quote/latest/quote/) macro to generate the
   implementation of the trait for each type. This time, we will enable only the features that are strictly
   necessary to generate the code. This implementation is found in the [`using-syn-partial`](using-syn-partial) crate.
3. **Manual generation**: The manual implementation generates the [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) by using the [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) API directly. A priori, we expect this
   implementation to be the most efficient one, as it avoids the overhead of the [`quote!`](https://docs.rs/quote/latest/quote/) macro. We still enable
   all the features of [`syn`](https://docs.rs/syn/latest/syn/) to see whether the performance is significantly affected by the features enabled.
   This implementation is found in the [`using-no-parsing-full-syn`](using-no-parsing-full-syn) crate.
4. **Manual generation with minimal features**: In order to actually make use of all of the tokens necessary for the Manual generation,
   it is necessary to enable the `full` feature set of [`syn`](https://docs.rs/syn/latest/syn/), and therefore we cannot, at this time, make a crate which uses
   the "minimal" features set.

## Results

In this section, we present the results of the experiment. We measure both the time necessary to build each of the
crates, the sizes of the generated binaries, and the time necessary to execute the code-generation code.
We use respectively [`cargo build --timings`](https://doc.rust-lang.org/cargo/reference/timings.html), `ls -lh target/debug/` and [`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html). The benches are found specifically in the [`syn-bencher/benches`](syn-bencher/benches) directory.

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

Surprisingly enough, the build times are not significantly different between the different strategies. Even more surprisingly, the crate with the largest build time requirements is the one that uses the [`syn`](https://docs.rs/syn/latest/syn/) APIs directly without use of the [`quote!`](https://docs.rs/quote/latest/quote/) macro.

### Binary sizes

In this section, we present the sizes of the generated binaries for each of the crates. The sizes are measured using the `ls -lh` command, for both the debug and release builds. No particular build flags are used to optimize for size.

| Crate | Size (debug) | Size (release) |
|-------|--------------|----------------|
| [`using-syn-full`](using-syn-full) | 793KB | 272KB |
| [`using-syn-partial`](using-syn-partial) | 793KB | 273KB |
| [`using-no-parsing-full-syn`](using-no-parsing-full-syn) | 1.5MB | 387KB |

Unexpectedly, we find again that the crate that uses the [`syn`](https://docs.rs/syn/latest/syn/) APIs directly without the [`quote!`](https://docs.rs/quote/latest/quote/) macro generates the largest binary. This is surprising, as we would expect the crate that uses the [`quote!`](https://docs.rs/quote/latest/quote/) macro to generate the largest binary, as it includes the [`quote!`](https://docs.rs/quote/latest/quote/) macro itself.

### Execution times

In this section, we present the execution times for the code-generation code. The benchmarks are run using the `cargo bench` command, and are based on [`criterion`](https://docs.rs/criterion/latest/criterion/). The benchmarks are found in the [`syn-bencher/benches`](syn-bencher/benches) directory.

| Crate | Time (ms) |
|-------|-----------|
| [`using-syn-full`](using-syn-full) | 8.44 |
| [`using-syn-partial`](using-syn-partial) | 8.63 |
| [`using-no-parsing-full-syn`](using-no-parsing-full-syn) | 9.42 |

Again, unexpectedly, the crate that uses the [`syn`](https://docs.rs/syn/latest/syn/) APIs directly without the [`quote!`](https://docs.rs/quote/latest/quote/) macro generates the slowest code-generation code. This is surprising, as we would expect the crate that uses the [`quote!`](https://docs.rs/quote/latest/quote/) macro to generate the slowest code-generation code, as it includes the [`quote!`](https://docs.rs/quote/latest/quote/) macro itself.

## Conclusions

Our experiments compared different approaches to generating Rust code with Syn, measuring build times, binary sizes, and execution times. The results we obtained were somewhat unexpected.

Build times were similar across strategies, with manual [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) handling (`using-no-parsing-full-syn`) taking the longest, contradicting expectations. Binary size analysis showed that this approach also produced the largest binaries, despite assumptions that [`quote!`](https://docs.rs/quote/latest/quote/) would be more bloated.

Execution of [`criterion`](https://docs.rs/criterion/latest/criterion/)-based benchmarks revealed that manual [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) construction was the slowest, challenging the idea that avoiding [`quote!`](https://docs.rs/quote/latest/quote/) improves efficiency. Instead, manually managing tokens introduced overhead.

Overall, using [`quote!`](https://docs.rs/quote/latest/quote/) with [`syn`](https://docs.rs/syn/latest/syn/)—even with all features—provides a balanced trade-off between build time, binary size, and runtime performance. Manual [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) handling does not offer clear advantages and may introduce inefficiencies. Future work should investigate why direct token manipulation results in larger, slower binaries and explore alternative optimizations.
