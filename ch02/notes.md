<!-- run draw image concurrency -->
cargo build --release
time target/release/ch02 mandel.png 4000x3000 -1.20,0.35 -1,0.20


<!-- Run quick replace -->
echo "Hello, world" > test.txt
cargo run "world" "Rust" test.txt test-modified.txt
cat test-modified.txt