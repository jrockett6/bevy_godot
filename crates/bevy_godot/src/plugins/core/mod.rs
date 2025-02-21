use crate::prelude::*;
use bevy::ecs::system::SystemParam;
use iyes_loopless::condition::ConditionalSystemDescriptor;
use iyes_loopless::prelude::*;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

pub mod godot_ref;
pub use godot_ref::*;

pub mod transforms;
pub use transforms::{Transform, Transform2D, *};

pub mod scene_tree;
pub use scene_tree::*;

pub mod collisions;
pub use collisions::*;

pub mod signals;
pub use signals::*;

pub mod input_event;
pub use input_event::*;

pub struct GodotCorePlugin;

impl Plugin for GodotCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy::core::CorePlugin::default())
            .add_plugin(bevy::log::LogPlugin::default())
            .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
            .add_plugin(bevy::time::TimePlugin)
            .add_plugin(bevy::hierarchy::HierarchyPlugin)
            .add_plugin(GodotSceneTreePlugin)
            .add_plugin(GodotTransformsPlugin)
            .add_plugin(GodotCollisionsPlugin)
            .add_plugin(GodotSignalsPlugin)
            .add_plugin(GodotInputEventPlugin);
    }
}

/// Bevy Resource that is available when the app is updated through `_process` callback
#[derive(Resource)]
pub struct GodotVisualFrame;

/// Bevy Resource that is available when the app is updated through `_physics_process` callback
#[derive(Resource)]
pub struct GodotPhysicsFrame;

/// Adds `as_physics_system` that schedules a system only for the physics frame
pub trait AsPhysicsSystem<Params> {
    #[allow(clippy::wrong_self_convention)]
    fn as_physics_system(self) -> ConditionalSystemDescriptor;
}

impl<Params, T: IntoSystem<(), (), Params>> AsPhysicsSystem<Params> for T {
    fn as_physics_system(self) -> ConditionalSystemDescriptor {
        self.run_if_resource_exists::<GodotPhysicsFrame>()
    }
}

/// Adds `as_visual_system` that schedules a system only for the frame
pub trait AsVisualSystem<Params> {
    #[allow(clippy::wrong_self_convention)]
    fn as_visual_system(self) -> ConditionalSystemDescriptor;
}

impl<Params, T: IntoSystem<(), (), Params>> AsVisualSystem<Params> for T {
    fn as_visual_system(self) -> ConditionalSystemDescriptor {
        self.run_if_resource_exists::<GodotVisualFrame>()
    }
}

#[deprecated]
pub type SystemDelta<'w, 's> = SystemDeltaTimer<'w, 's>;

/// SystemParam to keep track of an independent delta time
///
/// Not every system runs on a Bevy update and Bevy can be updated multiple
/// during a "frame".
#[derive(SystemParam)]
pub struct SystemDeltaTimer<'w, 's> {
    last_time: Local<'s, Option<Instant>>,
    #[system_param(ignore)]
    marker: PhantomData<&'w ()>,
}

impl<'w, 's> SystemDeltaTimer<'w, 's> {
    /// Returns the time passed since the last invocation
    pub fn delta(&mut self) -> Duration {
        let now = Instant::now();
        let last_time = self.last_time.unwrap_or(now);

        *self.last_time = Some(now);

        now - last_time
    }

    pub fn delta_seconds(&mut self) -> f32 {
        self.delta().as_secs_f32()
    }

    pub fn delta_seconds_f64(&mut self) -> f64 {
        self.delta().as_secs_f64()
    }
}

pub trait FindEntityByNameExt<T> {
    fn find_entity_by_name(self, name: &str) -> Option<T>;
}

impl<'a, T: 'a, U> FindEntityByNameExt<T> for U
where
    U: Iterator<Item = (&'a Name, T)>,
{
    fn find_entity_by_name(mut self, name: &str) -> Option<T> {
        self.find_map(|(ent_name, t)| (ent_name.as_str() == name).then_some(t))
    }
}
