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

  def from_file(_arg1), do: err()
  def get_embeddings(_arg1, _arg2), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
