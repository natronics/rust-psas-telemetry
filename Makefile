build:
	cargo clean --package psas-telemetry
	rm -r src/messages/*.rs
	cargo build

update_docs:
	rm -rf docs
	cargo doc
	mv target/doc ./docs
	echo '<meta http-equiv="refresh" content="0; url=psas_telemetry/" />' > ./docs/index.html

clean:
	cargo clean --package psas-telemetry
	rm -r src/messages/*.rs
	rm -rf docs

.PHONY: build update_docs clean
