defmodule ExFinalFusion do
  @moduledoc """
  Documentation for `ExFinalFusion`.

  Where to get models:
  - https://fasttext.cc/
  """
end

defmodule ExFinalFusion.Embeddings do
  @moduledoc false
  defstruct [:resource]
end

defimpl Inspect, for: ExFinalFusion.Embeddings do
  def inspect(embeddings, _opts) do
    Inspect.Algebra.concat([
      "#ExFinalFusion",
      to_string(Enum.drop(:erlang.ref_to_list(embeddings.resource), 4))
    ])
  end
end

defmodule ExFinalFusion.Native do
  @moduledoc false
  use Rustler,
    otp_app: :ex_final_fusion,
    crate: "exfinalfusion_native"

  @type similarity_type ::
          :cosine_similarity
          | :angular_similarity
          | :euclidean_similarity
          | :euclidean_distance
  @typedoc """
  default options:
  limit: 1,
  batch_size: None, means all at once - this is memory intensive.
  similarity_type: :cosine_similarity

  """
  @type search_options :: [
          limit: integer,
          batch_size: integer,
          similarity_type: similarity_type
        ]
  @typedoc """
  It allows to specify what function will be used to parse the embeddings file.
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
  def read(_arg1, _arg2), do: err()

  @doc """
  returns embedding of a word
  """
  @spec embedding(reference(), String.t()) :: {:ok, [float]}
  def embedding(_arg1, _arg2), do: err()

  @doc """
  returns a vector of embeddings of a word
  """
  @spec embedding(reference(), [String.t()]) :: {:ok, [[float]]}
  def embedding_batch(_arg1, _arg2), do: err()

  @doc """
  returns a list of words included in the embeddings
  """
  @spec words(reference()) :: [String.t()]
  def words(_arg1), do: err()

  @doc """
  returns the index of a word
  """
  @spec idx(reference(), String.t()) :: nil | {:word, [integer]} | {:subword, [integer]}
  def idx(_arg1, _arg2), do: err()

  @spec len(reference()) :: integer
  def len(_arg1), do: err()

  @spec words_len(reference()) :: integer
  def words_len(_arg1), do: err()

  @spec vocab_len(reference()) :: integer
  def vocab_len(_arg1), do: err()

  @spec dims(reference()) :: [integer]
  def dims(_arg1), do: err()

  @doc """
  returns the metadata as a map or nil
  """
  @spec metadata(reference()) :: map() | nil
  def metadata(_arg1), do: err()

  @doc """
  returns words that are similar to the query word.
  the options are
  """
  @spec word_similarity(reference, String.t(), Keyword.t()) :: {:ok, [{String.t(), float}]}
  def word_similarity(_arg1, _arg2, _arg3 \\ []), do: err()

  @doc """
  returns the calculated analogy

  This method returns words that are close in vector space for the analogy query word1 is to word2 as word3 is to ?.
  More concretely, it searches embeddings that are similar to:
  """
  @spec analogy(reference, String.t(), String.t(), String.t(), Keyword.t()) :: {:ok, [{String.t(), float}]}
  def analogy(_arg1, _arg2, _arg3, _arg4, _arg5 \\ []), do: err()

  @doc """
  similar to the analogy query but allows to remove queried words from results
  """
  @spec analogy_masked(reference, String.t(), bool, String.t(), bool, String.t(), bool, Keyword.t()) ::
          {:ok, [{String.t(), float}]}
  def analogy_masked(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8 \\ []), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
