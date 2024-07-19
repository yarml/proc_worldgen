mod camera;
mod world;

use bevy::{app::App, DefaultPlugins};
use camera::CameraPlugin;
use world::plugin::WorldPlugin;

fn main() {
  App::new().add_plugins((DefaultPlugins, CameraPlugin, WorldPlugin)).run();
}
