BIN_FILE := alfred-wake-word
LINT_PARAMS := $(shell cat .lints | cut -f1 -d"#" | tr '\n' ' ')

build:
	cargo build
build-release:
	cargo build --release
aarch64:
	cross build --release --target aarch64-unknown-linux-gnu

install: clean-bin build
	mkdir bin
	cp target/debug/${BIN_FILE} bin/
	cp target/debug/build/pv_porcupine-*/out/lib/linux/x86_64/libpv_porcupine.so bin/libpv_porcupine.so
	cp target/debug/build/pv_recorder-*/out/lib/linux/x86_64/libpv_recorder.so bin/libpv_recorder.so
	cp -r models bin/models
install-aarch64: clean-bin aarch64
	mkdir bin
	cp target/aarch64-unknown-linux-gnu/release/${BIN_FILE} bin/
	cp target/aarch64-unknown-linux-gnu/release/build/pv_porcupine-*/out/lib/raspberry-pi/cortex-a53-aarch64/libpv_porcupine.so bin/libpv_porcupine.so
	cp target/aarch64-unknown-linux-gnu/release/build/pv_recorder-*/out/lib/raspberry-pi/cortex-a53-aarch64/libpv_recorder.so bin/libpv_recorder.so
	cp -r models bin/models

clean: clean-target clean-bin
clean-target:
	rm -rf target
clean-bin:
	rm -rf bin

clippy:
	cargo clippy --all-targets --all-features -- -D warnings $(LINT_PARAMS)

clippy-fix:
	__CARGO_FIX_YOLO=1 cargo clippy --fix --allow-staged -- -D warnings $(LINT_PARAMS)
