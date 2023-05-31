cd backend
cargo doc --document-private-items --release --no-deps
cd ..

cd dbcli
cargo doc --document-private-items --release --no-deps
cd ..

mkdir -p Doc/code/bin
rm -rf Doc/code/bin/*
cp -r target/doc Doc/code/bin

cd adminpage
cargo doc --document-private-items --release --no-deps
mkdir -p ../Doc/code/adminpage
rm -rf ../Doc/code/adminpage/*
cp -r target/doc ../Doc/code/adminpage
cd ..

cd studentpage
cargo doc --document-private-items --release --no-deps
mkdir -p ../Doc/code/studentpage
rm -rf ../Doc/code/studentpage/*
cp -r target/doc ../Doc/code/studentpage
cd ..

