cd backend
cargo build --release
cd ..

# TODO: build database_cli

cd adminpage
trunk build --release
cd ..

cd studentpage
trunk build --release
cd ..

mkdir -p res
cp adminpage/dist/* res
cp studentpage/dist/* res
rm res/index.html

if [ -f "secret.config.toml" ];
then
  echo "Using existing file"
else
  cp secret.example.config.toml secret.config.toml
fi