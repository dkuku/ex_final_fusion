defmodule ExFinalFusion do
  @moduledoc """
  ExFinalFusion is an Elixir binding to the Rust crate.
  [finalfusion](https://crates.io/crates/finalfusion)
  Finalfusion is a file format for word embeddings,
  along with an associated set of libraries and utilities.

  from the crate documentation:
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
  """
  alias ExFinalFusion.Native

  @typedoc """
  This specifies how to calculate the similarity type when returning
  similarities. This only changes the returned value, as cosine similarity
  is always used.
  """
  @type similarity_type ::
          :cosine_similarity
          | :angular_similarity
          | :euclidean_similarity
          | :euclidean_distance
  @typedoc """
  Options passed to the functions that search for embeddings:

  Default options:
  - Limit: 1
  - Batch size: None (means all at once, but this is memory intensive)
  - Similarity type: Cosine similarity

  """
  @type search_options :: [
          limit: integer,
          batch_size: integer,
          similarity_type: similarity_type
        ]
  @typedoc """
  It allows you to specify which function will be used to parse the
   embeddings file. You can find more information in the Rust crate
   documentation.
  """
  @type read_type ::
          :floret_text
          | :embeddings
          | :mmap_embeddings
          | :fasttext
          | :fasttext_lossy
          | :text
          | :text_lossy
          | :text_dims
          | :text_dims_lossy
          | :word2vec_binary
          | :word2vec_binary_lossy
          | :fifu
          | :word2vec
          | :floret

  @doc """
  Functions Available on the Embeddings Module

  - :floret_text,
  - :embeddings,
  - :mmap_embeddings,
  - :fasttext,
  - :fasttext_lossy,
  - :text,
  - :text_lossy,
  - :text_dims,
  - :text_dims_lossy,
  - :word2vec_binary,
  - :word2vec_binary_lossy,

  Aliases

  - :fifu = :embeddings,
  - :word2vec = :word2vec_binary,
  - :floret = :floret_text
  """
  @spec read(String.t(), read_type) :: reference()
  defdelegate read(path, model_type), to: Native

  @doc """
  Returns the embedding of a word.
  """
  @spec embedding(reference(), String.t()) :: {:ok, [float]}
  defdelegate embedding(ref, word), to: Native

  @doc """
  Returns a vector of embeddings for a word.
  """
  @spec embedding(reference(), [String.t()]) :: {:ok, [[float]]}
  defdelegate embedding_batch(ref, list_of_words), to: Native

  @doc """
  Returns a list of words included in the embeddings.
  """
  @spec words(reference()) :: [String.t()]
  defdelegate words(ref), to: Native

  @doc """
  Returns the index of a word.
  """
  @spec idx(reference(), String.t()) :: nil | {:word, [integer]} | {:subword, [integer]}
  defdelegate idx(ref, word), to: Native

  @spec len(reference()) :: integer
  defdelegate len(ref), to: Native

  @spec words_len(reference()) :: integer
  defdelegate words_len(ref), to: Native

  @spec vocab_len(reference()) :: integer
  defdelegate vocab_len(ref), to: Native

  @spec dims(reference()) :: [integer]
  defdelegate dims(ref), to: Native

  @doc """
  Returns the metadata as a map or nil.
  """
  @spec metadata(reference()) :: map() | nil
  defdelegate metadata(ref), to: Native

  @doc """
  Returns words that are similar to the query word.
  """
  @spec word_similarity(reference, String.t(), search_options) :: {:ok, [{String.t(), float}]}
  defdelegate word_similarity(ref, word, search_params \\ []), to: Native

  @doc """
  Returns the calculated analogy.

  This method returns words that are close in vector space for the
  analogy query word1 is to word2 as word3 is to ?.
  More concretely, it searches embeddings that are similar to:
  """
  @spec analogy(reference, String.t(), String.t(), String.t(), search_options) :: {:ok, [{String.t(), float}]}
  defdelegate analogy(ref, word1, word2, word3, search_options \\ []), to: Native

  @doc """
  This function is similar to the analogy query, but it also allows for
  the removal of queried words from the results.
  """
  @spec analogy_masked(reference, String.t(), bool, String.t(), bool, String.t(), bool, search_options) ::
          {:ok, [{String.t(), float}]}
  defdelegate analogy_masked(ref, word_1, hide_1, word_2, hide_2, word_3, hide_3, search_options \\ []), to: Native
end
