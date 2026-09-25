mod recorder;

use alfred_core::AlfredModule;
use alfred_core::log::{debug, warn};
use alfred_core::message::{Message, MessageType};
use alfred_core::tokio;
use porcupine::PorcupineBuilder;
use crate::recorder::Recorder;

const MODULE_NAME: &str = "wake_word";

fn get_porcupine_library(module: &AlfredModule) -> Option<String> {
    let library_path = module.config.get_module_value("library_path");
    module.config.get_module_value("porcupine_library_path")
        .or_else(|| library_path.map(|path| path + "libpv_porcupine.so"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut module = AlfredModule::new(MODULE_NAME, env!("CARGO_PKG_VERSION")).await?;
    let access_key = module.config.get_module_value("porcupine_access_key").expect("Porcupine access-key not found");
    let porcupine_library_path = get_porcupine_library(&module);
    let device_name = module.config.get_module_value("device_name");

    let ppn_model = module.config.get_module_value("ppn_model").expect("Porcupine model file not found");
    let lang_model = module.config.get_module_value("lang_model").expect("Porcupine model file not found");

    let mut porcupine_builder = PorcupineBuilder::new_with_keyword_paths(access_key, &[ppn_model]);
    let porcupine = match porcupine_library_path {
        Some(lib) => porcupine_builder.library_path(lib),
        None => &mut porcupine_builder,
    }
    .model_path(lang_model)
    .init()
    .expect("Unable to create Porcupine");

    debug!(
        "Devices available: {:?}",
        Recorder::available_devices().expect("Unable to get the list of available devices")
    );
    let recorder = Recorder::new(device_name.as_deref(), usize::try_from(porcupine.frame_length())?)
        .expect("Failed to initialize recorder");

    recorder.start().expect("Failed to start audio recording");

    debug!("Listening for wake words...");

    loop {
        let frame = recorder.read().expect("Failed to read audio frame");

        let keyword_index = porcupine.process(&frame);
        match keyword_index { 
            Err(e) => warn!("Failed to process audio frame: {e}"),
            Ok(keyword_index) => {
                if keyword_index >= 0 {
                    let mut message = Message::empty();
                    message.message_type = MessageType::Audio;
                    module
                        .send_event(MODULE_NAME, "triggered", &message)
                        .await?;
                    debug!("Detected {keyword_index}");
                }
            }
        }
    }
}
