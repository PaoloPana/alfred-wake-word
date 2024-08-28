build:
	cargo build
build_release:
	cargo build --release
aarch64:
	if [ -d "bin" ] ; then rm -rf bin/; fi
	mkdir bin
	cross build --release --target aarch64-unknown-linux-gnu
	cp target/aarch64-unknown-linux-gnu/release/alfred-wake-word bin/
	cp target/aarch64-unknown-linux-gnu/release/build/pv_porcupine-*/out/lib/raspberry-pi/cortex-a53-aarch64/libpv_porcupine.so bin/libpv_porcupine.so
	cp target/aarch64-unknown-linux-gnu/release/build/pv_recorder-507fd7d37a2f5f71/out/lib/raspberry-pi/cortex-a53-aarch64/libpv_recorder.so bin/libpv_recorder.so
	mkdir bin/models
	cp models/alfred_it_raspberry-pi_v3_0_0.ppn bin/models/alfred_it_raspberry-pi_v3_0_0.ppn
	cp models/porcupine_params_it.pv bin/models/porcupine_params_it.pv
