(wasi_test "wasi_sees_virtual_root.wasm"
  (map_dirs "act1:test_fs/hamlet/act1" "act2:test_fs/hamlet/act2" "act1-again:test_fs/hamlet/act1")
  (assert_return (i64.const 0))
  (assert_stdout "\"/act1\"\n\"/act1-again\"\n\"/act2\"\n\"/act1\"\n\"/act1-again\"\n\"/act2\"\n\"/act1\"\n\"/act1-again\"\n\"/act2\"\n\"/act1\"\n\"/act1-again\"\n\"/act2\"\nROOT IS SAFE\n")
)