# ExFinalFusion

ExFinalFusion is an Elixir binding to the Rust crate.
[finalfusion](https://crates.io/crates/finalfusion)

Finalfusion is a file format for word embeddings,
along with an associated set of libraries and utilities.

From the crate documentation:
finalfusion supports a variety of formats:
- Vocabulary
     - Subwords
     - No subwords
- Storage
     - Array
     - Memory-mapped
     - Quantized
- Format
     - finalfusion
     - fastText
     - floret
     - GloVe
     - word2vec

Moreover, finalfusion provides:

- Similarity queries
- Analogy queries
- Quantizing embeddings through reductive
- Conversion to the following formats:
     - finalfusion
     - word2vec
     - GloVe

[final fusion file format](https://finalfusion.github.io/spec)

[Project page](https://finalfusion.github.io/)

[Train embeddings](https://finalfusion.github.io/finalfrontier.html)

Where to get models:

- [Fasttext models](https://fasttext.cc/)

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `ex_final_fusion` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:ex_final_fusion, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/ex_final_fusion>.

