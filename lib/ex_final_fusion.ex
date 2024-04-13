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

  def read(_arg1, _arg2), do: err()
  def embedding(_arg1, _arg2), do: err()
  def embedding_batch(_arg1, _arg2), do: err()

  def len(_arg1), do: err()
  def dims(_arg1), do: err()
  def metadata(_arg1), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
