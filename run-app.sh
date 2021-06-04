cargo build --all \
  && RUST_BACKTRACE=1 cargo watch -i "*/app/**" -i -x "run"
