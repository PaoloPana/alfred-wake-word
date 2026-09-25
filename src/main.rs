mod recorder;

use std::error::Error;
use std::str::FromStr;
use alfred_core::AlfredModule;
use alfred_core::log::debug;
use alfred_core::message::{Message, MessageType};
use alfred_core::tokio;
use rustpotter::{Rustpotter, RustpotterConfig, SampleFormat};
use crate::recorder::{Recorder, SAMPLE_RATE};

const MODULE_NAME: &str = "wake_word";

fn get_value<T: FromStr>(module: &AlfredModule, key: &str) -> Result<Option<T>, Box<dyn Error>> {
    Ok(module.config.get_module_value(key)
        .map(|value| value.parse::<T>().map_err(|_| format!("Invalid value for {key}: {value}")))
        .transpose()?)
}

fn get_rustpotter_config(module: &AlfredModule) -> Result<RustpotterConfig, Box<dyn Error>> {
    let mut config = RustpotterConfig::default();
    config.fmt.sample_rate = usize::try_from(SAMPLE_RATE)?;
    config.fmt.sample_format = SampleFormat::I16;
    config.fmt.channels = 1;
    if let Some(threshold) = get_value(module, "threshold")? {
        config.detector.threshold = threshold;
    }
    if let Some(avg_threshold) = get_value(module, "avg_threshold")? {
        config.detector.avg_threshold = avg_threshold;
    }
    if let Some(min_scores) = get_value(module, "min_scores")? {
        config.detector.min_scores = min_scores;
    }
    config.filters.gain_normalizer.enabled = get_value(module, "gain_normalizer")?.unwrap_or(true);
    config.filters.band_pass.enabled = get_value(module, "band_pass")?.unwrap_or(false);
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut module = AlfredModule::new(MODULE_NAME, env!("CARGO_PKG_VERSION")).await?;
    let device_name = module.config.get_module_value("device_name");
    let wakeword_model = module.config.get_module_value("wakeword_model").expect("Wake word model file not found");

    let mut rustpotter = Rustpotter::new(&get_rustpotter_config(&module)?)?;
    rustpotter.add_wakeword_from_file(MODULE_NAME, &wakeword_model)?;

    debug!(
        "Devices available: {:?}",
        Recorder::available_devices().expect("Unable to get the list of available devices")
    );
    let recorder = Recorder::new(device_name.as_deref(), rustpotter.get_samples_per_frame())
        .expect("Failed to initialize recorder");

    recorder.start().expect("Failed to start audio recording");

    debug!("Listening for wake words...");

    loop {
        let frame = recorder.read().expect("Failed to read audio frame");
        if let Some(detection) = rustpotter.process_samples(frame) {
            let mut message = Message::empty();
            message.message_type = MessageType::Audio;
            module
                .send_event(MODULE_NAME, "triggered", &message)
                .await?;
            debug!("Detected {} (score: {}, avg score: {})", detection.name, detection.score, detection.avg_score);
        }
    }
}
