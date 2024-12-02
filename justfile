list:
  just -l

run n:
  cargo run --release {{n}}

test n:
  cargo test day{{n}}
