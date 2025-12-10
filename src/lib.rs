use zed_extension_api as zed;

struct UmkaExtension;

impl zed::Extension for UmkaExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(UmkaExtension);
