use kissa::{EventType, KissaPlugin, KissaResult, KissaSender, TestEventType};
#[derive(Default)]
struct Plugin;

impl KissaPlugin for Plugin {
    fn name(&self) -> &'static str {
        "{{project-name}}"
    }

    fn on_event(
        &self,
        _event: &EventType,
        _sender: KissaSender<EventType>,
    ) -> KissaResult<EventType> {
        Ok(())
    }
}

#[no_mangle]
fn _create() -> Box<dyn KissaPlugin> {
    Box::new(Plugin::default())
}
