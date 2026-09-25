# Wake word models

The wake word is detected with [rustpotter](https://github.com/GiviMAD/rustpotter).
Put the model here and set its path as `wakeword_model` in `config.toml`.

## Creating the "alfred" model

Install the CLI (`--locked` is required, newer dependency versions do not build):

```sh
cargo install rustpotter-cli --locked
```

Record 3 to 8 samples of the wake word, each containing only the word "Alfred" with little
silence around it, ideally with the same microphone used by the module:

```sh
rustpotter-cli devices                       # pick the device index
rustpotter-cli record -i <index> --ms 1500 alfred1.wav
rustpotter-cli record -i <index> --ms 1500 alfred2.wav
# ...
```

Build the wake word reference and check it in real time:

```sh
rustpotter-cli build --name alfred --path models/alfred.rpw alfred*.wav
rustpotter-cli spot -i <index> models/alfred.rpw
```

A reference (`.rpw`) recognises the voice(s) it was recorded with. To detect other speakers,
record samples from all of them, or train a model (`.rpn`) from a larger set of positive and
negative samples with `rustpotter-cli train` (see the rustpotter documentation).

Detection can be tuned in `config.toml` with `threshold`, `avg_threshold` and `min_scores`.
