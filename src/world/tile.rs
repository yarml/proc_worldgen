pub const TILE_SIZE: f32 = 1.0;

#[derive(Clone)]
pub struct TileCoord {
  pub q: isize,
  pub r: isize,
}

#[derive(Clone)]
pub enum TileType {
  Ocean,
  Sea,
  Lake,
  LowLand,
  Plateau,
  Mountain,
}

#[derive(Clone)]
pub struct Tile {
  pub coord: TileCoord,
  pub ttype: TileType,
  pub elevation: f64,
  pub slopes: [f64; 6],
}
