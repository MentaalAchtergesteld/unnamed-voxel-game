use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct FlycamPlugin;

impl Plugin for FlycamPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FlycamSettings::default())
            .insert_resource(InputState::default())
            .add_systems(Startup, (spawn_flycam, initial_grab_cursor))
            .add_systems(Update, (flycam_move, flycam_look, toggle_camera_movement));
    }
}

#[derive(Resource)]
pub struct FlycamSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for FlycamSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

#[derive(Component)]
struct Flycam;

fn spawn_flycam(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 16., 0.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Flycam,
    ));
}

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn toggle_camera_movement(
    mouse_button: Res<Input<MouseButton>>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window_query.get_single_mut() {
        if mouse_button.just_pressed(MouseButton::Left) {
            let new_cursor_position = Vec2::new(window.width() / 2., window.height() / 2.);
            window.set_cursor_position(Some(new_cursor_position));
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `toggle_camera_movement`!");
    }
}

fn initial_grab_cursor(mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window_query.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `intiial_grab_cursor`!");
    }
}

fn flycam_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    settings: Res<FlycamSettings>,
    mut flycam_query: Query<&mut Transform, With<Flycam>>,
) {
    if let Ok(window) = primary_window_query.get_single() {
        for mut transform in flycam_query.iter_mut() {
            let mut velocity = Vec3::ZERO;

            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => match key {
                        KeyCode::W => velocity += transform.forward(),
                        KeyCode::S => velocity -= transform.forward(),
                        KeyCode::A => velocity -= transform.right(),
                        KeyCode::D => velocity += transform.right(),
                        KeyCode::Space => velocity += Vec3::Y,
                        KeyCode::ShiftLeft => velocity -= Vec3::Y,
                        _ => (),
                    },
                }
            }

            velocity = velocity.normalize_or_zero();

            let modifier = if keys.pressed(KeyCode::ControlLeft) {
                4.
            } else {
                1.
            };

            transform.translation += velocity * time.delta_seconds() * settings.speed * modifier;
        }
    } else {
        warn!("Primary window not found for `flycam_move`!");
    }
}

fn flycam_look(
    settings: Res<FlycamSettings>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut flycam_query: Query<&mut Transform, With<Flycam>>,
) {
    if let Ok(window) = primary_window_query.get_single() {
        let mut delta_state = state.as_mut();
        for mut transform in flycam_query.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        delta_state.pitch -=
                            (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        delta_state.yaw -=
                            (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }
            }

            delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
        }
    } else {
        warn!("Primary window not found for `flycam_look`!");
    }
}
