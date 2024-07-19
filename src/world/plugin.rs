use super::{gen::WorldGenerator, WORLD_SIZE};
use bevy::{
  app::{Plugin, Startup, Update},
  asset::{Assets, Handle},
  input::keyboard::KeyboardInput,
  math::Vec3,
  pbr::{DirectionalLightBundle, MaterialMeshBundle, StandardMaterial},
  prelude::{
    Commands, Component, EventReader, KeyCode, Query, Res, ResMut, Resource,
    With,
  },
  render::mesh::Mesh,
  transform::components::Transform,
};
use rand::Rng;

pub struct WorldPlugin;

#[derive(Resource)]
struct DebugMeshes([Handle<Mesh>; 4], Handle<Mesh>);

#[derive(Component)]
struct DebugMesh;

fn init(
  mut commands: Commands,
  mut mesh_assets: ResMut<Assets<Mesh>>,
  mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
  let mut rng = rand::thread_rng();
  let seed = rng.gen();
  let gen = WorldGenerator::new(seed, WORLD_SIZE);
  let [cont_mesh, erosion_mesh, peaks_mesh, terrain_mesh] = gen.debug_meshes();
  let world_mesh = gen.generate().mesh();

  let cont_mesh_handle = mesh_assets.add(cont_mesh);
  let erosion_mesh_handle = mesh_assets.add(erosion_mesh);
  let peaks_mesh_handle = mesh_assets.add(peaks_mesh);
  let terrain_mesh_handle = mesh_assets.add(terrain_mesh);
  let world_mesh_handle = mesh_assets.add(world_mesh);

  let material = StandardMaterial::default();
  let material_handle = material_assets.add(material);

  commands.insert_resource(DebugMeshes(
    [
      cont_mesh_handle,
      erosion_mesh_handle,
      peaks_mesh_handle,
      terrain_mesh_handle,
    ],
    world_mesh_handle.clone(),
  ));
  commands.spawn((
    DebugMesh,
    MaterialMeshBundle {
      mesh: world_mesh_handle,
      material: material_handle,
      ..Default::default()
    },
  ));
  commands.spawn(DirectionalLightBundle {
    transform: Transform::from_xyz(25.0, 15.0, 25.0)
      .looking_at(Vec3::ZERO, Vec3::Y),
    ..Default::default()
  });
}

fn change_debug_mesh(
  mut kbdev: EventReader<KeyboardInput>,
  debug_meshes: Res<DebugMeshes>,
  mut q_debug_mesh: Query<&mut Handle<Mesh>, With<DebugMesh>>,
) {
  let mut debug_mesh = q_debug_mesh.single_mut();
  for ev in kbdev.read() {
    if ev.state.is_pressed() {
      match ev.key_code {
        KeyCode::Digit1 => {
          *debug_mesh = debug_meshes.0[0].clone();
        }
        KeyCode::Digit2 => {
          *debug_mesh = debug_meshes.0[1].clone();
        }
        KeyCode::Digit3 => {
          *debug_mesh = debug_meshes.0[2].clone();
        }
        KeyCode::Digit4 => {
          *debug_mesh = debug_meshes.0[3].clone();
        }
        KeyCode::Digit0 => {
          *debug_mesh = debug_meshes.1.clone();
        }
        _ => {}
      }
    }
  }
}

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_systems(Startup, init);
    app.add_systems(Update, change_debug_mesh);
  }
}
