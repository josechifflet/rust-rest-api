start-server:
	cargo watch -q -c -w src/ -x run

install:
	cargo add actix-web
	cargo add actix-cors
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add uuid --features v4
	# HotReload
	cargo install cargo-watch
	cargo install cargo-module