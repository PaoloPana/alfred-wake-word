use alfred_rs::connection::Sender;
use alfred_rs::interface_module::InterfaceModule;
use alfred_rs::log::{debug, warn};
use alfred_rs::message::{Message, MessageType};
use alfred_rs::tokio;
use porcupine::PorcupineBuilder;
use pv_recorder::PvRecorderBuilder;

const MODULE_NAME: &str = "wake_word";

fn get_libraries(module: &InterfaceModule) -> (Option<String>, Option<String>) {
    let library_path = module.config.get_module_value("library_path");
    let porcupine_library_path = library_path.clone().map(|path| path + "libpv_porcupine.so");
    let recorder_library_path = library_path.map(|path| path + "libpv_recorder.so");
    (
        module.config.get_module_value("porcupine_library_path").or(porcupine_library_path),
        module.config.get_module_value("recorder_library_path").or(recorder_library_path),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let mut module = InterfaceModule::new(MODULE_NAME).await?;
    let access_key = module.config.get_module_value("porcupine_access_key").expect("Porcupine access-key not found");
    let (porcupine_library_path, recorder_library_path) = get_libraries(&module);
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

    let mut recorder_builder = PvRecorderBuilder::new(i32::try_from(porcupine.frame_length())?);
    let recorder_builder = match recorder_library_path {
        Some(lib) => recorder_builder.library_path(lib.as_ref()),
        None => &mut recorder_builder,
    };
    let device_index = device_name.map_or(0, |device_name| {
        i32::try_from(recorder_builder
            .get_available_devices()
            .expect("Devices not found")
            .iter()
            .position(|el| el.as_str() == device_name)
            .expect("Device name not found"))
            .expect("Device index not found")
    });
    debug!(
        "Devices available: {:?}",
        recorder_builder.get_available_devices().expect("Unable to get the list of available devices")
    );
    let recorder = recorder_builder
        .device_index(device_index)
        .init()
        .expect("Failed to initialize pvrecorder");

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
                    message.message_type = MessageType::AUDIO;
                    module
                        .send_event(MODULE_NAME, "triggered", &message)
                        .await?;
                    debug!("Detected {}", keyword_index);
                }
            }
        }
    }
}
