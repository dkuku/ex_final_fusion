defmodule ExFinalFusion.Embeddings do
  @moduledoc """
  Struct that keeps a reference to the model
  """
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
