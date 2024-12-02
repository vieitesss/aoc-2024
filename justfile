list:
  just -l

run n:
  cargo run --release {{n}}

get n:
  ./get.sh {{n}}

test n:
  cargo test day{{n}}
