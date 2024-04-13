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

    test "dims", %{ref: ref} do
      assert 100 == ExFinalFusion.Native.dims(ref)
    end

    test "metadata", %{ref: ref} do
      assert nil == ExFinalFusion.Native.metadata(ref)
    end
  end
end
