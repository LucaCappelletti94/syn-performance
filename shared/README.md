# Shared

Crate containing the shared code for the different experiments.
While it would be possibly cleaner to define here a single trait for the `codegen` and have
it implemented in the different other crates, this would require for this crate to depend on
`syn` with a particular feature set, which would defeat the purpose of the experiment.
