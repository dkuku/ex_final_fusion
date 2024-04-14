defmodule ExFinalFusion do
  @moduledoc """
  Documentation for `ExFinalFusion`.

  Where to get models:
  - https://fasttext.cc/
  """
  alias ExFinalFusion.Native

  @type similarity_type ::
          :cosine_similarity
          | :angular_similarity
          | :euclidean_similarity
          | :euclidean_distance
  @typedoc """
  default options:

  - limit: 1,
  - batch_size: None, means all at once - this is memory intensive.
  - similarity_type: :cosine_similarity
  """
  @type search_options :: [
          limit: integer,
          batch_size: integer,
          similarity_type: similarity_type
        ]
  @typedoc """
  It allows to specify what function will be used to parse the embeddings
  file.
  You can find more in the rust crate documentation
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
  Functions available on embeddings module

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
  returns embedding of a word
  """
  @spec embedding(reference(), String.t()) :: {:ok, [float]}
  defdelegate embedding(ref, word), to: Native

  @doc """
  returns a vector of embeddings of a word
  """
  @spec embedding(reference(), [String.t()]) :: {:ok, [[float]]}
  defdelegate embedding_batch(ref, list_of_words), to: Native

  @doc """
  returns a list of words included in the embeddings
  """
  @spec words(reference()) :: [String.t()]
  defdelegate words(ref), to: Native

  @doc """
  returns the index of a word
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
  returns the metadata as a map or nil
  """
  @spec metadata(reference()) :: map() | nil
  defdelegate metadata(ref), to: Native

  @doc """
  returns words that are similar to the query word.
  the options are
  """
  @spec word_similarity(reference, String.t(), search_options) :: {:ok, [{String.t(), float}]}
  defdelegate word_similarity(ref, word, search_params \\ []), to: Native

  @doc """
  returns the calculated analogy

  This method returns words that are close in vector space for the
  analogy query word1 is to word2 as word3 is to ?.
  More concretely, it searches embeddings that are similar to:
  """
  @spec analogy(reference, String.t(), String.t(), String.t(), search_options) :: {:ok, [{String.t(), float}]}
  defdelegate analogy(ref, word1, word2, word3, search_options \\ []), to: Native

  @doc """
  similar to the analogy query but allows to remove queried words from results
  """
  @spec analogy_masked(reference, String.t(), bool, String.t(), bool, String.t(), bool, search_options) ::
          {:ok, [{String.t(), float}]}
  defdelegate analogy_masked(ref, word_1, hide_1, word_2, hide_2, word_3, hide_3, search_options \\ []), to: Native
end
