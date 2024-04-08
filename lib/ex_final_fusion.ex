defmodule ExFinalFusion do
  @moduledoc """
  Documentation for `ExFinalFusion`.
  """
  use Rustler,
    otp_app: :ex_final_fusion,
    crate: :exfinalfusion

  def from_file(_arg1),
    do: :erlang.nif_error(:nif_not_loaded)
end
