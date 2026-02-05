use {bevy::prelude::*, rapier::prelude::*};

#[cfg(__debug__)]
use bevy::diagnostic;
#[cfg(__debug__)]
use rapier::render::RapierDebugRenderPlugin;

mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Update, close_on_esc);

    #[cfg(__debug__)]
    {
        use crate::utils::debug::ui::{setup_debug_ui, toggle_debug_ui, update_debug_ui};

        app.add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(diagnostic::EntityCountDiagnosticsPlugin::default())
            .add_plugins(diagnostic::SystemInformationDiagnosticsPlugin::default())
            .insert_resource(utils::debug::ui::DebugVisible(false));

        app.add_systems(Startup, setup_debug_ui)
            .add_systems(Update, toggle_debug_ui)
            .add_systems(Update, update_debug_ui);

        println!("Debug Mode Enabled");
    }

    app.run();
}

fn close_on_esc(key: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    #[cfg(not(feature = "headless"))]
    if key.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    #[cfg(feature = "headless")]
    {
        std::thread::sleep(std::time::Duration::from_secs_f32(3));

        exit.write(AppExit::Success);
    }
}
