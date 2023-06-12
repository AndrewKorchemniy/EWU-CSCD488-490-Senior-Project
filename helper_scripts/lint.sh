echo "main code (common, backend, database, dbcli)"
cargo fmt -- --check
cargo clippy -- -D warnings
echo "student page code"
cd studentpage
cargo fmt -- --check
cargo clippy -- -D warnings
cd ..
echo "admin page code"
cd adminpage
cargo fmt -- --check
cargo clippy -- -D warnings
cd ..