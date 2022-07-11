use crate::voxel::{
    material::VoxelMaterial,
    materials::{Dirt, Grass, Sand},
    Voxel,
};

use super::LayeredBiomeTerrainGenerator;

pub struct BasicDesertBiomeTerrainGenerator;

impl LayeredBiomeTerrainGenerator for BasicDesertBiomeTerrainGenerator {
    fn fill_strata(&self, layer: u32) -> Voxel {
        match layer {
            _ => Sand::into_voxel(),
        }
    }
}

pub struct BasicPlainsBiomeTerrainGenerator;

impl LayeredBiomeTerrainGenerator for BasicPlainsBiomeTerrainGenerator {
    fn fill_strata(&self, layer: u32) -> Voxel {
        match layer {
            0..=1 => Grass::into_voxel(),
            _ => Dirt::into_voxel(),
        }
    }
}
