(wasi_test "fd_close.wasm"
  (map_dirs ".:test_fs/hamlet")
  (assert_return (i64.const 0))
  (assert_stdout "Successfully closed file!\nSuccessfully closed stderr!\nSuccessfully closed stdin!\n")
)