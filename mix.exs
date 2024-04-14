defmodule ExFinalFusion.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_final_fusion,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package(),
      description: description()
    ]
  end

  defp description do
    "ex_final_fusion allows to read word embeddings in formats: finalfusion, fastText, floret, GloVe, word2vec"
  end

  defp package do
    [
      # These are the default files included in the package
      files: [
        "lib",
        "mix.exs",
        "native/exfinalfusion_native/.cargo",
        "native/exfinalfusion_native/src",
        "native/exfinalfusion_native/Cargo*",
        "README*"
      ],
      licenses: ["Apache-2.0", "MIT"],
      links: %{
        "GitHub" => "https://github.com/dkuku/ex_final_fusion",
        "Crate GitHub" => "https://github.com/finalfusion/finalfusion-rust",
        "Original project homepage" => "https://finalfusion.github.io/"
      }
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.32.1", runtime: false},
      {:ex_doc, "~> 0.31", only: :dev, runtime: false},
      {:styler, "~> 0.11", only: [:dev, :test], runtime: false},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false}
    ]
  end
end
