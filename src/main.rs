use std::collections::LinkedList;
use alfred_rs::connection::Sender;
use alfred_rs::error::Error;
use alfred_rs::interface_module::InterfaceModule;
use alfred_rs::log::debug;
use alfred_rs::message::{Message, MessageType};
use alfred_rs::tokio;
use porcupine::{Porcupine, PorcupineBuilder};
use pv_recorder::PvRecorderBuilder;

const MODULE_NAME: &'static str = "wake-word";

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let module = InterfaceModule::new(MODULE_NAME.to_string()).await?;
    let mut publisher = module.connection.publisher;
    let access_key = module.config.get_module_value("porcupine_access_key".to_string()).expect("Porcupine access-key not found");
    let ppn_model = module.config.get_module_value("ppn_model".to_string()).expect("Porcupine model file not found");
    let lang_model = module.config.get_module_value("lang_model".to_string()).expect("Porcupine model file not found");
    let callback_topic = module.config.get_module_value("callback_topic".to_string()).expect("Callback not found");

    let porcupine: Porcupine = PorcupineBuilder::
        //new_with_keywords(access_key, &[BuiltinKeywords::Porcupine])
        new_with_keyword_paths(access_key, &[ppn_model])
        .model_path(lang_model)
        .init().expect("Unable to create Porcupine");

    let recorder = PvRecorderBuilder::new(porcupine.frame_length() as i32)
        .device_index(0)
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
