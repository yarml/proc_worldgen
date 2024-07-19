use bevy::{
  render::mesh::{Indices, Mesh, PrimitiveTopology},
  utils::HashMap,
};
use std::cmp::{max, min};
use tile::{Tile, TileType};

mod gen;
pub mod plugin;
mod tile;

const WORLD_SIZE: u32 = 64;

struct World {
  radius: isize,
  tiles: HashMap<(isize, isize), Tile>,
}

impl World {
  fn mesh(&self) -> Mesh {
    let radius = self.radius;
    let indices = {
      let mut indices = vec![];
      let mut base = 0;
      for q in -radius..=radius {
        let rmin = max(-radius, -q - radius);
        let rmax = min(radius, radius - q);
        for _ in rmin..=rmax {
          // Suboptimal on purpose, we aren't drawing hexagons, we are drawing pyramids with a hexagonal base
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
            0 + base,
            6 + base,
            5 + base,
            0 + base,
            1 + base,
            6 + base,
          ]);
          base += 7;
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
        let tile = self.tiles.get(&(q, r)).expect("Tile not found");
        let sqrt3 = 3f32.sqrt();
        let center_x = TILE_SIZE * sqrt3 * q as f32 + sqrt3 / 2.0 * r as f32;
        let center_z = TILE_SIZE * 1.5 * r as f32;
        let color = match tile.ttype {
          TileType::Ocean => [0.0, 0.0, 1.0, 1.0],
          TileType::Sea => [0.0, 0.0, 0.5, 1.0],
          TileType::Lake => [0.0, 0.0, 0.0, 1.0],
          TileType::LowLand => [0.0, 1.0, 0.0, 1.0],
          TileType::Plateau => [0.5, 0.5, 0.0, 1.0],
          TileType::Mountain => [1.0, 0.0, 0.0, 1.0],
        };

        let y = tile.elevation as f32 / 128.0;
        positions.push([center_x, y, center_z]);
        normals.push([0.0, 1.0, 0.0]);
        colors.push(color);
        for angle in [0f32, 60.0, 120.0, 180.0, 240.0, 300.0] {
          let rel_x = angle.to_radians().sin() * TILE_SIZE;
          let rel_z = -angle.to_radians().cos() * TILE_SIZE;

          let x = center_x + rel_x;
          let z = center_z + rel_z;

          // Adjust the y position based on the slope
          let slope = (tile.slopes[(angle / 60.0) as usize]) / 128.0;
          let y = y - slope as f32;

          positions.push([x, y, z]);
          normals.push([
            slope as f32 * angle.to_radians().sin(),
            1.0,
            slope as f32 * angle.to_radians().cos(),
          ]);
          colors.push(color);
        }
      }
    }

    Mesh::new(PrimitiveTopology::TriangleList, Default::default())
      .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
      .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
      .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
      .with_inserted_indices(Indices::U32(indices))
  }
}
