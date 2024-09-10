use alfred_rs::connection::Sender;
use alfred_rs::error::Error;
use alfred_rs::interface_module::InterfaceModule;
use alfred_rs::log::debug;
use alfred_rs::message::{Message, MessageType};
use alfred_rs::tokio;
use porcupine::PorcupineBuilder;
use pv_recorder::PvRecorderBuilder;

const MODULE_NAME: &'static str = "wake-word";

fn get_libraries(module: &InterfaceModule) -> (Option<String>, Option<String>) {
    let library_path = module.config.get_module_value("library_path".to_string());
    let mut porcupine_library_path = None;
    let mut recorder_library_path = None;
    if library_path.is_some() {
        let library_path = library_path.unwrap();
        porcupine_library_path = Some(library_path.clone() + "libpv_porcupine.so");
        recorder_library_path = Some(library_path.clone() + "libpv_recorder.so");
    }
    (
        module.config.get_module_value("porcupine_library_path".to_string()).or(porcupine_library_path),
        module.config.get_module_value("recorder_library_path".to_string()).or(recorder_library_path)
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let mut module = InterfaceModule::new(MODULE_NAME.to_string()).await?;
    let access_key = module.config.get_module_value("porcupine_access_key".to_string()).expect("Porcupine access-key not found");
    let (porcupine_library_path, recorder_library_path) = get_libraries(&module);
    let device_name = module.config.get_module_value("device_name".to_string());

    let ppn_model = module.config.get_module_value("ppn_model".to_string()).expect("Porcupine model file not found");
    let lang_model = module.config.get_module_value("lang_model".to_string()).expect("Porcupine model file not found");

    let mut porcupine_builder = PorcupineBuilder::
        new_with_keyword_paths(access_key, &[ppn_model]);
    let porcupine = match porcupine_library_path {
        Some(lib) => porcupine_builder.library_path(lib),
        None => &mut porcupine_builder
    }.model_path(lang_model)
        .init().expect("Unable to create Porcupine");

    let mut recorder_builder = PvRecorderBuilder::
        new(porcupine.frame_length() as i32);
    let recorder_builder = match recorder_library_path {
        Some(lib) => recorder_builder.library_path(lib.as_ref()),
        None => &mut recorder_builder
    };
    let device_index = match device_name {
        Some(device_name) => recorder_builder.get_available_devices().unwrap().iter().position(|el| el.as_str() == device_name).unwrap() as i32,
        None => 0
    };
    debug!("Devices available: {:?}", recorder_builder.get_available_devices().unwrap());
    let recorder = recorder_builder.device_index(device_index)
        .init()
        .expect("Failed to initialize pvrecorder");

    recorder.start().expect("Failed to start audio recording");

    debug!("Listening for wake words...");

    loop {
        let frame = recorder.read().expect("Failed to read audio frame");

        let keyword_index = porcupine.process(&frame).unwrap();
        if keyword_index >= 0 {
            let mut message = Message::empty();
            message.message_type = MessageType::AUDIO;
            module.send_event(MODULE_NAME.to_string(), "triggered".to_string(), &message).await?;
            debug!("Detected {}", keyword_index);
        }
    }
}
