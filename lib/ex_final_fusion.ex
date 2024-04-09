defmodule ExFinalFusion do
  @moduledoc """
  Documentation for `ExFinalFusion`.
  """
end

defmodule ExFinalFusion.Native do
  use Rustler,
    otp_app: :ex_final_fusion,
    crate: "exfinalfusion_native"

  def from_file(_arg1), do: err()
  def get_embeddings(_arg1, _arg2), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
