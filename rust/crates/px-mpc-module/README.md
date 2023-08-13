rustup target add x86_64-apple-ios

Add following lines to .zshrc file
export OPENSSL_DIR=/usr/local/opt/openssl@1.1
export OPENSSL_SUPPORT=-I$(OPENSSL_DIR)/include -L$(OPENSSL_DIR)/lib
reference: https://stackoverflow.com/questions/29651258/install-openssl-devel-on-mac

Config cargo
./configure --prefix=/usr/local/curl --host=x86_64-apple-ios --without-ssl
./configure --prefix /usr/local/curl \
   --enable-static --disable-shared \
   --with-secure-transport \
   --host=x86_64-apple-darwin \
   CFLAGS="-arch x86_64 -target x86_64-apple-ios-simulator -isysroot $(xcrun -sdk iphonesimulator --show-sdk-path) -miphonesimulator-version-min=15.0"

reference:
https://stackoverflow.com/questions/28124221/error-linking-with-cc-failed-exit-code-1
https://stackoverflow.com/questions/41580504/how-to-install-libcurl-under-macos10-12-and-use-for-xcode
https://stackoverflow.com/questions/76607938/cross-compile-libcurl-to-run-on-ios-simulator



cargo build -p px-mpc-module --target x86_64-apple-ios
cargo build -p px-mpc-module --target x86_64-apple-ios --release

cbindgen --lang c --crate px-mpc-module --output ../ios/PyxelMPCModule/mpc-module.h
