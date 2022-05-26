use starry_extension_interface::{Extension,EmitSender, ListenSender};

#[no_mangle]
pub fn new() -> Box<dyn Extension> {
    Box::new(ExtensionSayHello {})
}

pub struct ExtensionSayHello {}

impl Extension for ExtensionSayHello {
    fn id(&self) -> &str {
        "3a90790e"
    }

    fn info(&self) -> &str {
        "{}"
    }

    fn load(
        &self,
        _emit_sender: &EmitSender,
        _listen_sender: &ListenSender,
    ) {
        todo!()
    }

    fn unload(&self) {
        println!("[{}] Unload instance!", self.id());
    }    
}
