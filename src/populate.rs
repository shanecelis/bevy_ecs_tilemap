use bevy::prelude::*;
use crate::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn populate_transforms(
    tiles_query:
        Query<
            (
                Entity,
                &TilePos,
                &TilemapId,
            ),
            With<Transform>,
    >,
    mut changed_tiles_query:
        Query<
            (
                Entity,
                &TilePos,
                &TilemapId,
            ),
            (
                Changed<TilePos>,
                With<Transform>,
            ),
    >,
    tilemap_query:
        Query<(
            &TilemapTileSize,
            &TilemapGridSize,
            &TilemapType,
            &TilemapSize,
            &TilemapAnchor,
        )>,
    changed_tilemap_query:
        Query<
            &TileStorage,
            Or<(
                Added<TilemapType>,
                Changed<TilemapType>,
                Changed<TilemapTileSize>,
                Changed<TilemapGridSize>,
                Changed<TilemapSize>,
                Changed<TilemapAnchor>,
            )>,
        >,
    mut commands: Commands,
) {
    let mut populate_transforms = Vec::new();
    // Check for map changes.
    for storage in &changed_tilemap_query {
        for (
            id,
            tile_pos,
            tilemap_id,
        ) in tiles_query.iter_many(storage.iter().filter_map(|x| *x))
        {
            let (tile_size, grid_size, map_type, map_size, anchor) = tilemap_query.get(tilemap_id.0).unwrap();
            let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(0.0);
            let anchor_offset = anchor.from_map(map_size, grid_size, tile_size, map_type);
            // *transform = anchor_offset * Transform::from_translation(tile_center);
            populate_transforms.push((id, anchor_offset * Transform::from_translation(tile_center)));
        }
    }

    // Process all tiles
    for (
        id,
        tile_pos,
        tilemap_id,
        // mut transform,
    ) in &mut changed_tiles_query
    {
        let (tile_size, grid_size, map_type, map_size, anchor) = tilemap_query.get(tilemap_id.0).unwrap();
        let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
        let anchor_offset = anchor.from_map(map_size, grid_size, tile_size, map_type);
        // *transform = anchor_offset * Transform::from_translation(tile_center);
        populate_transforms.push((id, anchor_offset * Transform::from_translation(tile_center)));
    }
    commands.insert_batch(populate_transforms);
}
