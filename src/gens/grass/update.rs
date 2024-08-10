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
    let (plyr_e, player_trans) = player.get_single().unwrap();
    let x = player_trans.translation.x;
    let z = player_trans.translation.z;
    if grass.is_empty() {
        let mut grass_grid = GrassGrid(HashMap::new());
        // generate grid of grass
        for i in -GRID_SIZE_HALF..=GRID_SIZE_HALF {
            for j in -GRID_SIZE_HALF..=GRID_SIZE_HALF {
                let a = x + i as f32 * GRASS_TILE_SIZE_1;
                let b = z + j as f32 * GRASS_TILE_SIZE_1;
                grass_grid.0.insert((a as i32, b as i32), true);
                let contains_player = (player_trans.translation.x - a).abs()
                    < GRASS_TILE_SIZE_1 / 2.
                    && (player_trans.translation.z - b).abs() < GRASS_TILE_SIZE_1 / 2.;
                let color = if contains_player { RED } else { PURPLE };
                let (main_mat, main_grass, main_data) = generate_grass(
                    &mut meshes,
                    &mut materials,
                    a,
                    b,
                    NUM_GRASS_1,
                    GRASS_TILE_SIZE_1,
                );
                commands.spawn(main_mat)
                    .insert(main_grass)
                    .insert(main_data)
                    .insert(ContainsPlayer(contains_player))
                    // .insert(NotShadowReceiver)
                    // .insert(ShowAabbGizmo {color: Some(color)})
                    ;
            }
        }
        commands.spawn(grass_grid);
    } else {
        let thread_pool = AsyncComputeTaskPool::get();
        let mut grass_grid = grid.get_single_mut().unwrap();
        let elapsed_time = time.elapsed_seconds_f64();
        let mut grass_w_player: Option<Entity> = None;
        for (ent, mh, grass_data, grass_trans, visibility, mut contains_player) in grass.iter_mut()
        {
            // remove or add ContainsPlayer if applicable
            if (player_trans.translation.x - grass_trans.translation.x).abs()
                >= GRASS_TILE_SIZE_1 / 2.
                || (player_trans.translation.z - grass_trans.translation.z).abs()
                    >= GRASS_TILE_SIZE_1 / 2.
            {
                if contains_player.0 {
                    *contains_player = ContainsPlayer(false);
                }
            } else {
                if !contains_player.0 {
                    *contains_player = ContainsPlayer(true);
                    // generate new grass
                    for i in -GRID_SIZE_HALF..=GRID_SIZE_HALF {
                        for j in -GRID_SIZE_HALF..=GRID_SIZE_HALF {
                            let a = grass_trans.translation.x + i as f32 * GRASS_TILE_SIZE_1;
                            let b = grass_trans.translation.z + j as f32 * GRASS_TILE_SIZE_1;
                            if let false =
                                *grass_grid.0.get(&(a as i32, b as i32)).unwrap_or(&false)
                            {
                                grass_grid.0.insert((a as i32, b as i32), true);
                                // todo: async way
                                let transform = Transform::from_xyz(a, 0., b);

                                let task_entity = commands.spawn_empty().id();
                                let task = thread_pool.spawn(async move {
                                    let mut command_queue = CommandQueue::default();
                                    let (mesh, grass_data) =
                                        generate_grass_mesh(a, b, NUM_GRASS_1, GRASS_TILE_SIZE_1);

                                    command_queue.push(move |world: &mut World| {
                                        let (grass_mesh_handle, grass_mat_handle) = {
                                            let mut system_state = SystemState::<(
                                                ResMut<Assets<Mesh>>,
                                                ResMut<
                                                    Assets<
                                                        ExtendedMaterial<
                                                            StandardMaterial,
                                                            GrassMaterialExtension,
                                                        >,
                                                    >,
                                                >,
                                            )>::new(
                                                world
                                            );
                                            let (mut meshes, mut mats) =
                                                system_state.get_mut(world);

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
                                            // .insert(ShowAabbGizmo {color: Some(Color::PURPLE)})
                                            .remove::<GenGrassTask>();
                                    });

                                    command_queue
                                });

                                // spawn a task marked GenGrassTask in the world to be handled by handle_tasks fn when complete
                                commands.entity(task_entity).insert(GenGrassTask(task));
                            }
                        }
                    }
                }
            }
            if contains_player.0 {
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
            } else if (player_trans.translation.x - grass_trans.translation.x).abs()
                > DESPAWN_DISTANCE
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
            }
        }

        if let Some(grass_w_player) = grass_w_player {
            // update aabb color
            // commands.get_entity(grass_w_player).unwrap().insert(AabbGizmo {color: Some(Color::RED)});
        }
    }
}

pub fn handle_tasks(mut commands: Commands, mut grass_tasks: Query<&mut GenGrassTask>) {
    for mut task in &mut grass_tasks {
        if let Some(mut commands_queue) = block_on(poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
    }
}
