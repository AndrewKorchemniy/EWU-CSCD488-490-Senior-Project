echo "main code (common, backend, database, dbcli)"
cargo fmt -- --check
cargo clippy
echo "student page code"
cd studentpage
cargo fmt -- --check
cargo clippy
cd ..
echo "admin page code"
cd adminpage
cargo fmt -- --check
cargo clippy
cd ..