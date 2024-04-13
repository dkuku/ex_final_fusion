defmodule ExFinalFusionTest do
  use ExUnit.Case

  doctest ExFinalFusion

  describe "loads files" do
    test "fifu" do
      assert {:ok, ref} = ExFinalFusion.Native.read("test/testdata/similarity.fifu", :fifu)
      assert {:ok, _emb} = ExFinalFusion.Native.embedding(ref, "Berlin")
    end

    test "fasttext" do
      assert {:ok, ref} =
               ExFinalFusion.Native.read("test/testdata/fasttext.bin", :fasttext)

      assert {:ok, _emb} = ExFinalFusion.Native.embedding(ref, "Berlin")
    end

    test "text" do
      assert {:ok, ref} =
               ExFinalFusion.Native.read("test/testdata/similarity.nodims", :text)

      assert {:ok, _emb} = ExFinalFusion.Native.embedding(ref, "Berlin")
    end

    test "text_dims" do
      assert {:ok, ref} =
               ExFinalFusion.Native.read("test/testdata/similarity.txt", :text_dims)

      assert {:ok, _emb} = ExFinalFusion.Native.embedding(ref, "Berlin")
    end

    test "word2vec" do
      assert {:ok, ref} =
               ExFinalFusion.Native.read("test/testdata/similarity.bin", :word2vec)

      assert {:ok, _emb} = ExFinalFusion.Native.embedding(ref, "Berlin")
    end
  end

  describe "metadata" do
    setup do
      {:ok, ref} = ExFinalFusion.Native.read("test/testdata/similarity.fifu", :fifu)
      [ref: ref]
    end

    test "len", %{ref: ref} do
      assert 41 == ExFinalFusion.Native.len(ref)
    end

    test "words_len", %{ref: ref} do
      assert 41 == ExFinalFusion.Native.words_len(ref)
    end

    test "vocab_len", %{ref: ref} do
      assert 41 == ExFinalFusion.Native.vocab_len(ref)
    end

    test "dims", %{ref: ref} do
      assert 100 == ExFinalFusion.Native.dims(ref)
    end

    test "metadata", %{ref: ref} do
      assert nil == ExFinalFusion.Native.metadata(ref)
    end

    test "words", %{ref: ref} do
      assert {:ok,
              [
                "Berlin",
                "Potsdam",
                "Hamburg",
                "Leipzig",
                "Dresden",
                "München",
                "Düsseldorf",
                "Bonn",
                "Stuttgart",
                "Weimar",
                "Berlin-Charlottenburg",
                "Rostock",
                "Karlsruhe",
                "Chemnitz",
                "Breslau",
                "Wiesbaden",
                "Hannover",
                "Mannheim",
                "Kassel",
                "Köln",
                "Danzig",
                "Erfurt",
                "Dessau",
                "Bremen",
                "Charlottenburg",
                "Magdeburg",
                "Neuruppin",
                "Darmstadt",
                "Jena",
                "Wien",
                "Heidelberg",
                "Dortmund",
                "Stettin",
                "Schwerin",
                "Neubrandenburg",
                "Greifswald",
                "Göttingen",
                "Braunschweig",
                "Berliner",
                "Warschau",
                "Berlin-Spandau"
              ]} ==
               ExFinalFusion.Native.words(ref)
    end

    test "idx", %{ref: ref} do
      assert {:word, [0]} == ExFinalFusion.Native.idx(ref, "Berlin")
      assert {:word, [23]} == ExFinalFusion.Native.idx(ref, "Bremen")
      assert nil == ExFinalFusion.Native.idx(ref, "Bucharest")
    end
  end
end
