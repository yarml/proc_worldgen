use std::cmp::{max, min};

use bevy::{
  render::mesh::{Indices, Mesh, PrimitiveTopology},
  utils::hashbrown::HashMap,
};
use noise::{Fbm, NoiseFn, Perlin};
use rand::{rngs::StdRng, RngCore, SeedableRng};

use super::{
  tile::{Tile, TileCoord, TileType, TILE_SIZE},
  World,
};

const CONTINANTELNESS_SCALE: f64 = 64.0;
const EROSION_SCALE: f64 = 64.0;
const PEAKS_SCALE: f64 = 16.0;
const TERRAIN_SCALE: f64 = 32.0;

pub struct WorldGenerator {
  radius: u32,
  rng: StdRng,
  continantelness: Fbm<Perlin>,
  erosion: Fbm<Perlin>,
  peaks: Fbm<Perlin>,
  terrain: Fbm<Perlin>,
}

impl WorldGenerator {
  pub fn new(seed: u32, radius: u32) -> Self {
    let base_seed = seed.wrapping_mul(radius).wrapping_mul(0xAFB333FE);
    let mut rng = StdRng::seed_from_u64(base_seed as u64);
    let continantelness_seed = base_seed.wrapping_mul(rng.next_u32());
    let erosion_seed = base_seed.wrapping_mul(rng.next_u32());
    let peaks_seed = base_seed.wrapping_mul(rng.next_u32());
    let terrain_seed = base_seed.wrapping_mul(rng.next_u32());
    Self {
      radius,
      rng,
      continantelness: Fbm::<Perlin>::new(continantelness_seed),
      erosion: Fbm::<Perlin>::new(erosion_seed),
      peaks: Fbm::<Perlin>::new(peaks_seed),
      terrain: Fbm::<Perlin>::new(terrain_seed),
    }
  }

  pub fn debug_meshes(&self) -> [Mesh; 4] {
    let cont_mesh = generate_debug_mesh(
      self.radius as isize,
      self.continantelness.clone(),
      CONTINANTELNESS_SCALE,
    );
    let erosion_mesh = generate_debug_mesh(
      self.radius as isize,
      self.erosion.clone(),
      EROSION_SCALE,
    );
    let peaks_mesh = generate_debug_mesh(
      self.radius as isize,
      self.peaks.clone(),
      PEAKS_SCALE,
    );
    let terrain_mesh = generate_debug_mesh(
      self.radius as isize,
      self.terrain.clone(),
      TERRAIN_SCALE,
    );

    [cont_mesh, erosion_mesh, peaks_mesh, terrain_mesh]
  }

  pub fn generate(&self) -> World {
    let mut tiles = HashMap::new();
    let radius = self.radius as isize;
    for q in -radius..=radius {
      let rmin = max(-radius, -q - radius);
      let rmax = min(radius, radius - q);
      for r in rmin..=rmax {
        let sqrt3 = 3f32.sqrt();
        let center_x = TILE_SIZE * sqrt3 * q as f32 + sqrt3 / 2.0 * r as f32;
        let center_z = TILE_SIZE * 1.5 * r as f32;

        let elevation = self.elevation(center_x as f64, center_z as f64);

        // [0,10) -> Ocean
        // [10,20) -> Sea
        // [20, 128) -> LowLand
        // [128,224) -> Plateau
        // [240,256) -> Mountain

        let ttype = if elevation < 10.0 {
          TileType::Ocean
        } else if elevation < 20.0 {
          TileType::Sea
        } else if elevation < 128.0 {
          TileType::LowLand
        } else if elevation < 224.0 {
          TileType::Plateau
        } else {
          TileType::Mountain
        };

        // Calculate slopes
        let mut slopes = [0.0; 6];
        for i in 0..6 {
          let angle = i as f32 * 60.0;
          let rel_x = angle.to_radians().sin() * TILE_SIZE;
          let rel_z = -angle.to_radians().cos() * TILE_SIZE;

          let x = center_x + rel_x;
          let z = center_z + rel_z;

          let neighbor_elevation = self.elevation(x as f64, z as f64);
          slopes[i] = elevation - neighbor_elevation;
        }

        tiles.insert(
          (q, r),
          Tile {
            coord: TileCoord { q, r },
            ttype,
            elevation,
            slopes,
          },
        );
      }
    }
    World { radius, tiles }
  }

  fn elevation_range(&self, x: f64, z: f64) -> (usize, usize) {
    let cont_x = x / CONTINANTELNESS_SCALE;
    let cont_z = z / CONTINANTELNESS_SCALE;

    let erosion_x = x / EROSION_SCALE;
    let erosion_z = z / EROSION_SCALE;

    let peaks_x = x / PEAKS_SCALE;
    let peaks_z = z / PEAKS_SCALE;

    let continantelness = self.continantelness.get([cont_x, cont_z]);
    let erosion = self.erosion.get([erosion_x, erosion_z]);
    let peaks = self.peaks.get([peaks_x, peaks_z]);

    if continantelness < 0.0 {
      (0, 10)
    } else if continantelness < 0.2 {
      (10, 20)
    } else {
      if erosion < -0.3 {
        if peaks.abs() < 0.1 {
          (240, 256)
        } else {
          (80, 224)
        }
      } else {
        if continantelness < 0.9 {
          (20, 144)
        } else {
          (96, 208)
        }
      }
    }
  }
  fn elevation(&self, x: f64, z: f64) -> f64 {
    let (min, max) = self.elevation_range(x, z);
    let terrain_x = x / TERRAIN_SCALE;
    let terrain_z = z / TERRAIN_SCALE;
    let terrain = self.terrain.get([terrain_x, terrain_z]);
    let terrain = (terrain + 1.0) / 1.0;
    terrain * (max - min) as f64 + min as f64
  }
}

fn generate_debug_mesh<N: NoiseFn<f64, 2>>(
  radius: isize,
  noise: N,
  perlin_scale: f64,
) -> Mesh {
  let indices = {
    let mut indices = vec![];
    let mut base = 0;
    for q in -radius..=radius {
      let rmin = max(-radius, -q - radius);
      let rmax = min(radius, radius - q);
      for _ in rmin..=rmax {
        indices.append(&mut vec![
          0 + base,
          2 + base,
          1 + base,
          0 + base,
          3 + base,
          2 + base,
          0 + base,
          4 + base,
          3 + base,
          0 + base,
          5 + base,
          4 + base,
        ]);
        base += 6;
      }
    }
    indices
  };

  const TILE_SIZE: f32 = 1.0;

  let mut positions = vec![];
  let mut normals = vec![];
  let mut colors = vec![];

  for q in -radius..=radius {
    let rmin = max(-radius, -q - radius);
    let rmax = min(radius, radius - q);
    for r in rmin..=rmax {
      let sqrt3 = 3f32.sqrt();
      let center_x = TILE_SIZE * sqrt3 * q as f32 + sqrt3 / 2.0 * r as f32;
      let center_z = TILE_SIZE * 1.5 * r as f32;

      let perlin_x = (center_x as f64) / perlin_scale;
      let perlin_z = (center_z as f64) / perlin_scale;
      let noise = noise.get([perlin_x, perlin_z]);

      let y = 0.0;
      for angle in [0f32, 60.0, 120.0, 180.0, 240.0, 300.0] {
        let rel_x = angle.to_radians().sin() * TILE_SIZE;
        let rel_z = -angle.to_radians().cos() * TILE_SIZE;

        let x = center_x + rel_x;
        let z = center_z + rel_z;

        positions.push([x, y, z]);
        normals.push([0.0, 1.0, 0.0]);
        if noise < 0.0 {
          colors.push([0.0, 0.0, -noise as f32, 1.0]);
        } else {
          colors.push([noise as f32, 0.0, 0.0, 1.0]);
        }
      }
    }
  }

  Mesh::new(PrimitiveTopology::TriangleList, Default::default())
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U32(indices))
}
