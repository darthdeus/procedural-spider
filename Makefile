wasm:
	rustup target add wasm32-unknown-unknown
	cargo build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/procedural-spider.wasm build/

upload: wasm
	butler push build darthdeus/procedural-spider:html5
