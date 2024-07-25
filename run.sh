cargo build --release
gcc main.c -L./target/x86_64-unknown-none/release -ldevsw -o main
./main