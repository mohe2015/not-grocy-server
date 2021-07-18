# Massive compile time improvements:
git clone https://github.com/bjorn3/rustc_codegen_cranelift.git
cd rustc_codegen_cranelift
./y.rs prepare
./y.rs build
cd ..
rustc_codegen_cranelift/build/cargo build



cargo install cargo-llvm-lines
cargo llvm-lines | head -20

https://endler.dev/2020/rust-compile-times/



cargo install cargo-udeps && cargo +nightly udeps


cargo update


cargo install cargo-outdated
cargo outdated -wR

cargo tree
cargo tree --duplicate

cargo install cargo-audit
cargo audit


#cargo rustc -- -Zself-profile
cargo +nightly -Z timings


cargo install cargo-bloat --no-default-features 
cargo bloat -n 10
cargo bloat --crates

https://perf.rust-lang.org/


https://github.com/the-lean-crate/cargo-diet
cargo install cargo-diet



