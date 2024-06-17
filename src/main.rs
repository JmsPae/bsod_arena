use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;
use iyes_perf_ui::PerfUiPlugin;
use lower_levels::GamePlugin;

use bevy::prelude::*;
use bevy::window::{MonitorSelection, PresentMode, WindowPosition, WindowResolution};


fn main() {
    App::new()
            .add_plugins((
                bevy::diagnostic::FrameTimeDiagnosticsPlugin,
                bevy::diagnostic::EntityCountDiagnosticsPlugin,
                bevy::diagnostic::SystemInformationDiagnosticsPlugin,
                DefaultPlugins.set(
                    WindowPlugin {
                        primary_window: Some(
                            Window {
                                title: "Lower Levels".into(),
                                resolution: WindowResolution::new(1280., 720.)/*.with_scale_factor_override(1.0)*/,
                                present_mode: PresentMode::AutoNoVsync,
                                position: WindowPosition::Centered(MonitorSelection::Index(2)),
                                ..Default::default()
                            }
                        ),
                        ..Default::default()
                    }
                ).disable::<PipelinedRenderingPlugin>(),
                PerfUiPlugin,
                GamePlugin
            ))
        .run();
}
