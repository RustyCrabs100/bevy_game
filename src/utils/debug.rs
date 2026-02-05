#![cfg(__debug__)]

pub mod ui {
    use bevy::{color::palettes::css::BLUE, diagnostic, prelude::*};

    #[derive(Resource)]
    pub struct DebugVisible(pub bool);

    #[derive(Component)]
    pub struct DebugUiRoot;

    pub fn setup_debug_ui(mut commands: Commands) {
        commands
            .spawn((
                DebugUiRoot,
                Node {
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    width: Val::Auto,
                    height: Val::Auto,
                    display: Display::None,
                    position_type: PositionType::Absolute,
                    overflow: Overflow::visible(),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Visibility::Hidden,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Diagnostics"),
                    TextFont {
                        font_size: 60.0,
                        ..Default::default()
                    },
                    TextColor(BLUE.into()),
                    Visibility::Inherited,
                ));
            });
    }

    pub fn toggle_debug_ui(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut visible: ResMut<DebugVisible>,
        mut query: Query<&mut Visibility, With<DebugUiRoot>>,
    ) {
        if keyboard.just_pressed(KeyCode::F3) {
            if let Ok(mut node_) = query.single_mut() {
                *node_ = match *node_ {
                    Visibility::Hidden => Visibility::Visible,
                    _ => Visibility::Hidden,
                };
            }

            visible.0 = !visible.0;

            for mut v in &mut query {
                *v = if visible.0 {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }

    pub fn update_debug_ui(
        diagnostics: Res<diagnostic::DiagnosticsStore>,
        mut query: Query<&mut Text, With<DebugUiRoot>>,
        visible: Res<DebugVisible>,
    ) {
        if !visible.0 {
            return;
        }

        let fps = diagnostics
            .get(&diagnostic::FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|d| d.smoothed())
            .unwrap_or(0.0);

        let frame_time = diagnostics
            .get(&diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME)
            .and_then(|d| d.smoothed())
            .unwrap_or(0.0);

        let entity_count = diagnostics
            .get(&diagnostic::EntityCountDiagnosticsPlugin::ENTITY_COUNT)
            .and_then(|d| d.smoothed())
            .unwrap_or(0.0);

        for mut text in &mut query {
            text.0 = format!(
                " FPS: {:.1} \n Frame Time: {:.2}ms \n Entities Active: {:.0} ",
                fps, frame_time, entity_count
            );
        }
    }
}
