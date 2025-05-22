echo "开始构建项目..."
cargo build --release

echo "创建 simple_db 文件夹..."
rm -rf simple_db
rm submit.zip
mkdir -p simple_db/target/release

echo "复制必要文件到 simple_db 文件夹..."
cp -r src simple_db/
cp target/release/simple_db simple_db/target/release
cp Cargo.lock simple_db/
cp Cargo.toml simple_db/

echo "创建 ZIP 归档文件..."
zip -r submit.zip simple_db

echo "完成！生成的文件是 submit.zip"
