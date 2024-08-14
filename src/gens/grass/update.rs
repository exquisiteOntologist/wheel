use bevy::color::palettes::css::{PURPLE, RED};
use bevy::ecs::system::SystemState;
use bevy::ecs::world::CommandQueue;
use bevy::pbr::ExtendedMaterial;

use bevy::tasks::{block_on, poll_once, AsyncComputeTaskPool};
use bevy::{prelude::*, utils::HashMap};

use crate::components::characters::player::resources::PlayerCharacter;
use crate::resources::ContainsPlayer;
use crate::utils::perlin::PerlinNoiseEntity;

use super::constants::{
    DESPAWN_DISTANCE, GRASS_TILE_SIZE_1, GRID_SIZE_HALF, NUM_GRASS_1, WIND_SIM_TRIGGER_DISTANCE,
};
use super::gen_init::{generate_grass, generate_grass_mesh};
use super::materials::{grass_material, GrassMaterialExtension};
use super::resources::{GenGrassTask, Grass, GrassData, GrassGrid};

pub fn update_grass(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, GrassMaterialExtension>>>,
    mut grass: Query<
        (
            Entity,
            &Handle<Mesh>,
            &GrassData,
            &Transform,
            &ViewVisibility,
            &mut ContainsPlayer,
        ),
        With<Grass>,
    >,
    mut grid: Query<&mut GrassGrid>,
    perlin: Res<PerlinNoiseEntity>,
    time: Res<Time>,
    player: Query<(Entity, &Transform), With<PlayerCharacter>>,
) {
    let (_, player_trans) = player.get_single().unwrap();
    let x = player_trans.translation.x;
    let z = player_trans.translation.z;

    if grass.is_empty() {
        update_grass_empty(
            &mut commands,
            &mut meshes,
            &mut materials,
            player_trans,
            x,
            z,
        );
        return;
    }

    let mut grass_grid = grid.get_single_mut().unwrap();
    // let elapsed_time = time.elapsed_seconds_f64();
    let mut grass_w_player: Option<Entity> = None;
    let mut grasses_without_player: Vec<Entity> = Vec::new();
    for (ent, mh, _grass_data, grass_trans, visibility, mut contains_player) in grass.iter_mut() {
        // because we are despawning distant grass
        let mut exists = true;

        // println!("gt {}", grass_trans.translation);

        let player_outside_tile = (player_trans.translation.x - grass_trans.translation.x).abs()
            >= GRASS_TILE_SIZE_1 / 2.
            || (player_trans.translation.z - grass_trans.translation.z).abs()
                >= GRASS_TILE_SIZE_1 / 2.;

        // remove or add ContainsPlayer if applicable
        if player_outside_tile {
            if contains_player.0 {
                // it no longer contains the player (contains_player outdated)
                *contains_player = ContainsPlayer(false);
            }
        } else {
            if !contains_player.0 {
                // it now contains the player (contains_player outdated)
                *contains_player = ContainsPlayer(true);
                // Generate the grass now that it contains the player
                update_grass_generate_grid(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    grass_trans,
                    &mut grass_grid,
                    player_trans,
                    contains_player.0,
                );
            }
        }

        if contains_player.0 {
            // ok so this is reliable
            grass_w_player = Some(ent);
        }
        // simulate wind only if close enough and if visible
        if (player_trans.translation.x - grass_trans.translation.x).abs()
            < WIND_SIM_TRIGGER_DISTANCE
            && (player_trans.translation.z - grass_trans.translation.z).abs()
                < WIND_SIM_TRIGGER_DISTANCE
            && visibility.get()
        {
            // if let Some(mesh) = meshes.get_mut(mh) {
            //     apply_wind(mesh, grass_data, &perlin, elapsed_time, player_trans.translation.xz());
            // }
        } else if (player_trans.translation.x - grass_trans.translation.x).abs() > DESPAWN_DISTANCE
            || (player_trans.translation.z - grass_trans.translation.z).abs() > DESPAWN_DISTANCE
        {
            grass_grid.0.insert(
                (
                    grass_trans.translation.x as i32,
                    grass_trans.translation.z as i32,
                ),
                false,
            );
            commands.get_entity(ent).unwrap().despawn_recursive();
            exists = false;
        }

        // by now any despawned grass should have been despawned
        if exists && !contains_player.0 {
            grasses_without_player.push(ent);
        }
    }

    // for grass_not_with_player in grasses_without_player {
    //     // (debugging) make purple so if player leaves grass it is not still red
    //     if let Some(mut nwp_ent) = commands.get_entity(grass_not_with_player) {
    //         nwp_ent.insert(ShowAabbGizmo {
    //             color: Some(Color::Srgba(PURPLE)),
    //         });
    //     };
    // }

    // if let Some(grass_w_player) = grass_w_player {
    //     // (debugging) update aabb color
    //     commands
    //         .get_entity(grass_w_player)
    //         .unwrap()
    //         .insert(ShowAabbGizmo {
    //             color: Some(Color::Srgba(RED)),
    //         });
    // }
}

/// This spawns some grass if there is none.
/// It usually should not be repeatedly called.
fn update_grass_empty(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ExtendedMaterial<StandardMaterial, GrassMaterialExtension>>>,
    player_trans: &Transform,
    x: f32,
    z: f32,
) {
    let mut grass_grid = GrassGrid(HashMap::new());
    // generate grid of grass
    for i in -GRID_SIZE_HALF..=GRID_SIZE_HALF {
        for j in -GRID_SIZE_HALF..=GRID_SIZE_HALF {
            let a = x + i as f32 * GRASS_TILE_SIZE_1;
            let b = z + j as f32 * GRASS_TILE_SIZE_1;
            grass_grid.0.insert((a as i32, b as i32), true);
            // Note this is for when the grass is empty
            // in the other section the value comes from the grass query
            let contains_player = (player_trans.translation.x - a).abs() < GRASS_TILE_SIZE_1 / 2.
                && (player_trans.translation.z - b).abs() < GRASS_TILE_SIZE_1 / 2.;
            // let color = if contains_player { RED } else { PURPLE };
            let (main_mat, main_grass, main_data) = generate_grass(
                &mut meshes,
                &mut materials,
                a,
                b,
                NUM_GRASS_1,
                GRASS_TILE_SIZE_1,
            );
            commands
                .spawn(main_mat)
                .insert(main_grass)
                .insert(main_data)
                .insert(ContainsPlayer(contains_player));
            // .insert(NotShadowReceiver)
            // .insert(ShowAabbGizmo {
            //     color: Some(Color::Srgba(color)),
            // });
        }
    }
    commands.spawn(grass_grid);
}

/// This generates a grid of grass.
/// Depending on how many grids are being generated at once,
/// this can be an extremely expensive operation.
fn update_grass_generate_grid(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ExtendedMaterial<StandardMaterial, GrassMaterialExtension>>>,
    grass_trans: &Transform,
    mut grass_grid: &mut Mut<GrassGrid>,
    player_trans: &Transform,
    has_player: bool,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    // generate new grass for a single tile
    'loop_i: for i in -GRID_SIZE_HALF..=GRID_SIZE_HALF {
        for j in -GRID_SIZE_HALF..=GRID_SIZE_HALF {
            let a = grass_trans.translation.x + i as f32 * GRASS_TILE_SIZE_1;
            let b = grass_trans.translation.z + j as f32 * GRASS_TILE_SIZE_1;

            if let false = *grass_grid.0.get(&(a as i32, b as i32)).unwrap_or(&false) {
                grass_grid.0.insert((a as i32, b as i32), true);
                let transform = Transform::from_xyz(a, 0., b);

                let task_entity = commands.spawn_empty().id();
                let task = thread_pool.spawn(async move {
                    let mut command_queue = CommandQueue::default();

                    // in addition to density, we can customize the number of vertices that go to generate_single_blade_verts
                    let (mesh, grass_data) =
                        generate_grass_mesh(a, b, NUM_GRASS_1, GRASS_TILE_SIZE_1);

                    command_queue.push(move |world: &mut World| {
                        let (grass_mesh_handle, grass_mat_handle) = {
                            let mut system_state = SystemState::<(
                                ResMut<Assets<Mesh>>,
                                ResMut<
                                    Assets<
                                        ExtendedMaterial<StandardMaterial, GrassMaterialExtension>,
                                    >,
                                >,
                            )>::new(world);
                            let (mut meshes, mut mats) = system_state.get_mut(world);

                            (
                                meshes.add(mesh),
                                mats.add(ExtendedMaterial {
                                    base: grass_material(),
                                    extension: GrassMaterialExtension {},
                                }),
                            )
                        };

                        world
                            .entity_mut(task_entity)
                            .insert(MaterialMeshBundle {
                                mesh: grass_mesh_handle,
                                material: grass_mat_handle,
                                transform,
                                ..default()
                            })
                            .insert(Grass)
                            .insert(grass_data)
                            .insert(ContainsPlayer(false))
                            // .insert(NotShadowReceiver)
                            // .insert(ShowAabbGizmo {
                            //     color: Some(Color::Srgba(PURPLE)),
                            // })
                            .remove::<GenGrassTask>();
                    });

                    command_queue
                });

                // spawn a task marked GenGrassTask in the world to be handled by handle_tasks fn when complete
                commands.entity(task_entity).insert(GenGrassTask(task));
            }
        }
    } // END grass generation
}

pub fn handle_tasks(mut commands: Commands, mut grass_tasks: Query<&mut GenGrassTask>) {
    for mut task in &mut grass_tasks {
        if let Some(mut commands_queue) = block_on(poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
    }
}
