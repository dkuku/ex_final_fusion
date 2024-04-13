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

  @spec words(reference()) :: [String.t()]
  def words(_arg1), do: err()

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
  @spec metadata(reference()) :: map() | nil
  def metadata(_arg1), do: err()
  @spec analogy(reference, String.t(), String.t(), String.t(), Keyword.t()) :: {:ok, [{String.t(), float}]}
  def analogy(_arg1, _arg2, _arg3, _arg4, _arg5 \\ []), do: err()
  @spec word_similarity(reference, String.t(), Keyword.t()) :: {:ok, [{String.t(), float}]}
  def word_similarity(_arg1, _arg2, _arg3 \\ []), do: err()

  @spec analogy_masked(reference, String.t(), String.t(), String.t(), bool, bool, bool, Keyword.t()) ::
          {:ok, [{String.t(), float}]}
  def analogy_masked(_arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8 \\ []), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
