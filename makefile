.PHONY: build run setup all api watch build-yew build-tauri watch-yew watch-tauri web clean-yew clean-tauri clean cli build-cli

all: build

# Install required build tools and dependencies
setup:
	cargo install tauri-cli
	cargo install trunk
	rustup target add wasm32-unknown-unknown

# Install Bolt CLI
install-cli:
	cd cli && cargo install --path .

# Build Bolt Desktop App
build: build-yew-tauri build-tauri
	cp -r ./tauri/target/release/bundle ./target

# Build Bolt CLI
build-cli: build-yew-cli
	cd cli && cargo build --release

# Run Bolt Desktop App in debug mode
run: build-yew-tauri watch-tauri

# Run Bolt CLI in debug mode
run-cli: build-yew-cli
	cd cli && BOLT_DEV=1 cargo run

build-yew: build-yew-cli build-yew-tauri

build-yew-tauri:
	cd yew && trunk build -d ../tauri/dist --filehash false
	cd yew && cp ./script.js ../tauri/dist
	mkdir ./tauri/dist/icon/
	cp -r ./icon/* ./tauri/dist/icon/ 
	
build-yew-cli:
	cd yew && trunk build -d ../tauri/dist --filehash false
	cd yew && cp ./script.js ../tauri/dist
	mkdir ./tauri/dist/icon/
	cp -r ./icon/* ./tauri/dist/icon/

build-tauri:
	cd tauri && cargo tauri build

watch-tauri:
	cargo tauri dev

publish:
	cd lib_bolt && cargo publish
	cd cli && cargo publish

# Clean temporary build files
clean: clean-yew clean-tauri clean-cli clean-lib

clean-yew:
	cd yew && cargo clean

clean-tauri:
	cd tauri && cargo clean

clean-cli:
	cd cli && cargo clean

clean-lib:
	cd lib_bolt && cargo clean
