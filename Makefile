.PHONY: watch

install:
	cargo install cargo-watch
watch: 
	cargo watch --features "live-reload" --why -w "templates/" -w "src/" -x run
	
