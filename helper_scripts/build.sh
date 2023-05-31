cd backend
cargo build --release
cd ..

cd dbcli
cargo build --release
cd ..

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

echo "server config file"
if [ -f "server.config.toml" ];
then
  echo "Using existing file"
else
  echo "Using example file"
  cp server.example.config.toml server.config.toml
fi

echo "secret config file"
if [ -f "secret.config.toml" ];
then
  echo "Using existing file"
else
  echo "Using example file"
  cp secret.example.config.toml secret.config.toml
fi