(wasi_test "fs_sandbox_test.wasm"
  (assert_return (i64.const 0))
  (assert_stdout "Reading the parent directory was okay? false\n")
)