(wasi_test "readlink.wasm"
  (preopens "test_fs")
  (assert_return (i64.const 0))
  (assert_stdout "true\n../act1/scene2.txt\nSCENE II. A room of state in the castle.\n\n    Enter KING CLAUDIUS, QUEEN GERTRUDE, HAMLET, POLONIUS, LAERTES, VOLTIMAND, CORNELI\n")
  (assert_stderr "[/Users/mark/wasi-tests/wasi/tests/readlink.rs:10] &p = \"test_fs/hamlet/bookmarks/2019-07-16\"\n")
)