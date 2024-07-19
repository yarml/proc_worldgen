use bevy::{
  app::{Plugin, Startup, Update},
  input::mouse::MouseWheel,
  math::Vec3,
  prelude::{
    Bundle, Camera3dBundle, Commands, Component, EventReader, Query, With,
  },
  render::camera::{Camera, Projection},
  transform::components::Transform,
  window::{PrimaryWindow, Window},
};

const CAMERA_SPEED: f32 = 0.4;
const CAMERA_SPEED_DEPTH: f32 = 1.4;
const WINDOW_EDGE: f32 = 100.0;

pub struct CameraPlugin;

#[derive(Component)]
struct CameraMarker;

#[derive(Bundle)]
struct CameraBundle {
  marker: CameraMarker,
  bundle: Camera3dBundle,
}

fn setup_camera(mut commands: Commands) {
  let transform =
    Transform::from_xyz(0.0, 12.0, -25.0).looking_at(Vec3::ZERO, Vec3::Y);
  commands.spawn(CameraBundle {
    marker: CameraMarker,
    bundle: Camera3dBundle {
      transform,
      projection: Projection::Perspective(Default::default()),
      ..Default::default()
    },
  });
}

fn input(
  q_win: Query<&Window, With<PrimaryWindow>>,
  mut q_camera: Query<&mut Transform, With<Camera>>,
) {
  let mut camera_motion = Vec3::ZERO;
  let win = q_win.single();
  if let Some(cursor) = win.cursor_position() {
    if cursor.y < WINDOW_EDGE {
      camera_motion.z = 1.0;
    } else if cursor.y > win.height() - WINDOW_EDGE {
      camera_motion.z = -1.0;
    }
    if cursor.x < WINDOW_EDGE {
      camera_motion.x = -1.0;
    } else if cursor.x > win.width() - WINDOW_EDGE {
      camera_motion.x = 1.0;
    }
  }

  let mut camera = q_camera.single_mut();
  let mut forward = camera.forward().normalize();
  forward.y = 0.0;

  let mut right = camera.right().normalize();
  right.y = 0.0;

  camera.translation += forward * camera_motion.z * CAMERA_SPEED;
  camera.translation += right * camera_motion.x * CAMERA_SPEED;
}

fn scroll_events(
  mut scroll_reader: EventReader<MouseWheel>,
  mut q_camera: Query<&mut Transform, With<Camera>>,
) {
  let mut motion = 0.0;
  for ev in scroll_reader.read() {
    motion += ev.y;
  }

  let mut camera = q_camera.single_mut();
  let forward = camera.forward().normalize();
  camera.translation += forward * motion * CAMERA_SPEED_DEPTH;
}

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_systems(Startup, setup_camera);
    app.add_systems(Update, (input, scroll_events));
  }
}
