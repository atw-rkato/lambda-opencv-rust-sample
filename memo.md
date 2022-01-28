# OpenCVのインストール

## Mac

```shell
xcode-select --install
brew install opencv
ln -s /Library/Developer/CommandLineTools/usr/lib/libclang.dylib ~/.rustup/toolchains/stable-x86_64-apple-darwin/lib/libclang.dylib
export PKG_CONFIG_PATH=/usr/local/Cellar/opencv/4.5.4_1/lib/pkgconfig
or
export PKG_CONFIG_PATH=/usr/local/Cellar/opencv/$(ls -vrr1 /usr/local/Cellar/opencv | head -n 1)/lib/pkgconfig
```

## Debian

```shell
apt update && apt install -y clang libopencv-dev
export LLVM_CONFIG_PATH=/usr/lib/llvm-11/bin/llvm-config
ln -s /usr/lib/llvm-11/lib/libclang.so.1 /usr/lib/llvm-11/lib/libclang.so
export LIBCLANG_PATH=/usr/lib/llvm-11/lib/libclang.so
```

## AmazonLinux2

```shell
yum update && yum install -y make cmake3 clang unzip
ln -s /usr/bin/cmake3 /usr/bin/cmake
curl -o opencv.zip -L https://github.com/opencv/opencv/archive/4.x.zip -w "\n" 
unzip opencv.zip
mkdir -p opencv-build && cd opencv-build
cmake -D CMAKE_BUILD_TYPE=RELEASE \
      -D CMAKE_INSTALL_PREFIX=/usr/local \
      -D WITH_TBB=ON \
      ../opencv-4.x
cmake --build . --parallel 12
make install
ln -s /usr/lib64/libclang.so.11.1 /usr/lib64/libclang.so
export LIBCLANG_PATH=/usr/lib64/libclang.so
export LD_LIBRARY_PATH=/usr/local/lib64/:$LD_LIBRARY_PATH
```
