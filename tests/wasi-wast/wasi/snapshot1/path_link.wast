(wasi_test "path_link.wasm"
  (map_dirs "act5:test_fs/hamlet/act5")
  (temp_dirs "temp")
  (assert_return (i64.const 0))
  (assert_stdout "ACT V\nSCENE I. A churchyard.\n\n    Enter two Clowns, with spades,\nACT V\nSCENE I. A churchyard.\n\n    Enter two Clowns, with spades,\nPath still exists\n")
)