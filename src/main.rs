use std::collections::LinkedList;
use alfred_rs::connection::Sender;
use alfred_rs::error::Error;
use alfred_rs::interface_module::InterfaceModule;
use alfred_rs::log::debug;
use alfred_rs::message::{Message, MessageType};
use alfred_rs::tokio;
use porcupine::PorcupineBuilder;
use pv_recorder::PvRecorderBuilder;

const MODULE_NAME: &'static str = "wake-word";

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let module = InterfaceModule::new(MODULE_NAME.to_string()).await?;
    let mut publisher = module.connection.publisher;
    let access_key = module.config.get_module_value("porcupine_access_key".to_string()).expect("Porcupine access-key not found");
    let library_path = module.config.get_module_value("library_path".to_string());
    let mut porcupine_library_path = None;
    let mut recorder_library_path = None;
    if library_path.is_some() {
        let library_path = library_path.unwrap();
        porcupine_library_path = Some(library_path.clone() + "libpv_porcupine.so");
        recorder_library_path = Some(library_path.clone() + "libpv_recorder.so");
    }
    porcupine_library_path = module.config.get_module_value("porcupine_library_path".to_string()).or(porcupine_library_path);
    recorder_library_path = module.config.get_module_value("recorder_library_path".to_string()).or(recorder_library_path);
    let device_name = module.config.get_module_value("device_name".to_string());

    let ppn_model = module.config.get_module_value("ppn_model".to_string()).expect("Porcupine model file not found");
    let lang_model = module.config.get_module_value("lang_model".to_string()).expect("Porcupine model file not found");
    let callback_topic = module.config.get_module_value("callback_topic".to_string()).expect("Callback not found");

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
    debug!("{:?}", recorder_builder.get_available_devices().unwrap());
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
            message.response_topics = LinkedList::from([
                "stt".to_string(),
                "openai".to_string(),
                "tts".to_string(),
                "audio_out".to_string(),
            ]);
            message.message_type = MessageType::AUDIO;
            publisher.send(callback_topic.clone(), &message).await?;
            debug!("Detected {}", keyword_index);
        }
    }
}
