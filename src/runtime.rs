use v8;

pub struct RuntimeState {
    pub context: v8::Global<v8::Context>,
}
