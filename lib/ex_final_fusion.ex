defmodule ExFinalFusion do
  @moduledoc """
  Documentation for `ExFinalFusion`.
  """
end

defmodule ExFinalFusion.Embeddings do
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

  @spec len(reference()) :: integer
  def len(_arg1), do: err()
  @spec dims(reference()) :: [integer]
  def dims(_arg1), do: err()
  @spec metadata(reference()) :: map() | nil
  def metadata(_arg1), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
