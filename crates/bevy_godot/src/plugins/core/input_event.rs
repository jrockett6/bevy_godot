use crate::prelude::{
    godot_prelude::{InputEvent as GodotInputEvent, InputEventKey, SubClass},
    *,
};

pub struct GodotInputEventPlugin;

impl Plugin for GodotInputEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::First,
            write_input_events
                .before(Events::<InputEvent>::update_system)
                .before(Events::<UnhandledInputEvent>::update_system),
        )
        .add_event::<InputEvent>()
        .add_event::<UnhandledInputEvent>()
        .add_event::<KeyInputEvent>();
    }
}

/// An input event from the `_input` callback
#[derive(Debug)]
pub struct InputEvent(Ref<GodotInputEvent>);

impl InputEvent {
    pub fn get<T: SubClass<GodotInputEvent>>(&self) -> TRef<T> {
        self.try_get().unwrap()
    }

    pub fn try_get<T: SubClass<GodotInputEvent>>(&self) -> Option<TRef<T>> {
        unsafe { self.0.assume_safe().cast() }
    }
}

/// An input event from the `_unhandled_input` callback
#[derive(Debug)]
pub struct UnhandledInputEvent(Ref<GodotInputEvent>);

impl UnhandledInputEvent {
    pub fn get<T: SubClass<GodotInputEvent>>(&self) -> TRef<T> {
        self.try_get().unwrap()
    }

    pub fn try_get<T: SubClass<GodotInputEvent>>(&self) -> Option<TRef<T>> {
        unsafe { self.0.assume_safe().cast() }
    }
}

/// An input event from the `_unhandled_key_input` callback
#[derive(Debug)]
pub struct KeyInputEvent(Ref<InputEventKey>);

impl KeyInputEvent {
    pub fn get(&self) -> TRef<InputEventKey> {
        self.try_get().unwrap()
    }

    pub fn try_get(&self) -> Option<TRef<InputEventKey>> {
        unsafe { self.0.assume_safe().cast() }
    }
}

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputEventType {
    Unhandled,
    Normal,
    Key,
}

#[doc(hidden)]
pub struct InputEventReader(pub std::sync::mpsc::Receiver<(InputEventType, Ref<GodotInputEvent>)>);

fn write_input_events(
    events: NonSendMut<InputEventReader>,
    mut unhandled_evts: EventWriter<UnhandledInputEvent>,
    mut normal_evts: EventWriter<InputEvent>,
    mut key_evts: EventWriter<KeyInputEvent>,
) {
    events.0.try_iter().for_each(|(tpe, event)| match tpe {
        InputEventType::Unhandled => unhandled_evts.send(UnhandledInputEvent(event)),
        InputEventType::Normal => normal_evts.send(InputEvent(event)),
        InputEventType::Key => {
            let event = event.cast::<InputEventKey>().unwrap();
            key_evts.send(KeyInputEvent(event))
        }
    });
}
