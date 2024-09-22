BIN_FILE := alfred-wake-word

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
	mkdir bin/models
	cp models/alfred_it_linux_v3_0_0.ppn bin/models/alfred_it_linux_v3_0_0.ppn
	cp models/porcupine_params_it.pv bin/models/porcupine_params_it.pv
install-aarch64: clean-bin aarch64
	mkdir bin
	cp target/aarch64-unknown-linux-gnu/release/${BIN_FILE} bin/
	cp target/aarch64-unknown-linux-gnu/release/build/pv_porcupine-*/out/lib/raspberry-pi/cortex-a53-aarch64/libpv_porcupine.so bin/libpv_porcupine.so
	cp target/aarch64-unknown-linux-gnu/release/build/pv_recorder-507fd7d37a2f5f71/out/lib/raspberry-pi/cortex-a53-aarch64/libpv_recorder.so bin/libpv_recorder.so
	mkdir bin/models
	cp models/alfred_it_raspberry-pi_v3_0_0.ppn bin/models/alfred_it_raspberry-pi_v3_0_0.ppn
	cp models/porcupine_params_it.pv bin/models/porcupine_params_it.pv

clean: clean-target clean-bin
clean-target:
	rm -rf target
clean-bin:
	rm -rf bin
