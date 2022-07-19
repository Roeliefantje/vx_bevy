use crate::voxel::{storage::VoxelBuffer, ChunkKey, ChunkShape, Voxel, CHUNK_LENGTH_U};

use super::noise::Heightmap;

mod layered;
pub use layered::*;

mod test;
pub use test::*;

/// A trait representing a terrain generator for a biome.
/// A biome can be defined as a collection of features that are applied on top of the terrain.
pub trait BiomeTerrainGenerator: 'static + Sync + Send {
    /// Carve the terrain using the materials for the biome.
    fn carve_terrain(
        &self,
        chunk_key: ChunkKey,
        heightmap: Heightmap<CHUNK_LENGTH_U, CHUNK_LENGTH_U>,
        buffer: &mut VoxelBuffer<Voxel, ChunkShape>,
    );

    /// Decorate the terrain with this biome specific features (e.g. flowers, trees, ores etc).
    fn decorate_terrain(
        &self,
        chunk_key: ChunkKey,
        heightmap: Heightmap<CHUNK_LENGTH_U, CHUNK_LENGTH_U>,
        buffer: &mut VoxelBuffer<Voxel, ChunkShape>,
    );
}

/// Utility trait for boxing biome generators.
pub trait IntoBoxedTerrainGenerator: BiomeTerrainGenerator + Sized {
    fn into_boxed_generator(self) -> Box<Self>;
}

impl<T: BiomeTerrainGenerator> IntoBoxedTerrainGenerator for T {
    fn into_boxed_generator(self) -> Box<Self> {
        Box::new(self)
    }
}
