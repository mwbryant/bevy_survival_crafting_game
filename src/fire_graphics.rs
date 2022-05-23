use bevy::ecs::system::lifetimeless::SRes;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::{self, RenderAsset};
use bevy::render::render_resource::std430::AsStd430;
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::sprite::{Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle};
use bevy::{prelude::*, sprite::Material2d};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::collections::HashMap;

use crate::prelude::{CameraFollower, TILE_SIZE};

pub struct FireGraphicsPlugin;

#[derive(Clone, Component, Default, AsStd430)]
struct FireGpu {
    position: Vec2,
    strength: f32,
    //BUG? Is this a bevy bug with padding?
    //Crashes with following message when test is removed:
    //Caused by:
    //In a RenderPass
    //note: encoder = `<CommandBuffer-(0, 1, Vulkan)>`
    //In a draw command, indexed:true indirect:false
    //note: render pipeline = `transparent_mesh2d_pipeline`
    //Buffer is bound with size 12 where the shader expects 16 in group[1] compact index 0
    test: f32,
}
#[derive(Clone, Component, Inspectable)]
struct Fire {
    strength: f32,
}

///Resource containing all active fires
#[derive(Clone, Default)]
struct ActiveFires {
    fires: HashMap<u32, FireGpu>,
}

impl ActiveFires {
    fn insert(&mut self, fire_id: u32, position: Vec2, strength: f32) {
        self.fires.insert(
            fire_id,
            FireGpu {
                position,
                strength,
                test: 0.0,
            },
        );
    }
    fn remove(&mut self, fire_id: u32) {
        self.fires.remove(&fire_id);
    }

    fn get_fire_gpus(&self) -> Vec<FireGpu> {
        self.fires.values().map(|v| v.clone()).collect()
    }
}

#[derive(Clone, TypeUuid, Default)]
#[uuid = "88d1793a-f685-4f16-aad8-42e9a07dd6b8"]
struct FireMaterial {
    active_fires: Vec<FireGpu>,
}

impl Plugin for FireGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<FireMaterial>::default())
            .init_resource::<ActiveFires>()
            .add_system(update_fire_overlay)
            .add_system_to_stage(CoreStage::PostUpdate, remove_fire_from_overlay)
            .add_startup_system(spawn_fire)
            .add_startup_system(spawn_fire_overlay)
            .register_inspectable::<Fire>();
    }
}

fn update_fire_overlay(
    mut material_assets: ResMut<Assets<FireMaterial>>,
    mut overlay: Query<&mut Handle<FireMaterial>>,
    changed_fires: Query<
        (Entity, &Fire, &Transform),
        Or<((Changed<Transform>, With<Fire>), Changed<Fire>)>,
    >,
    mut active_fires: ResMut<ActiveFires>,
) {
    for (entity, fire, transform) in changed_fires.iter() {
        active_fires.insert(entity.id(), transform.translation.truncate(), fire.strength);
    }
    let mut overlay = overlay.single_mut();
    //FIXME Do not create handles every frame, solve this using the same technique as the animate shader example
    *overlay = material_assets.add(FireMaterial {
        active_fires: active_fires.get_fire_gpus(),
    });
}

fn remove_fire_from_overlay(
    removed_fire: RemovedComponents<Fire>,
    mut active_fires: ResMut<ActiveFires>,
) {
    for fire in removed_fire.iter() {
        active_fires.remove(fire.id());
    }
}

fn spawn_fire(mut commands: Commands, mut active_fires: ResMut<ActiveFires>) {
    let mut fire = commands
        .spawn_bundle(TransformBundle::default())
        .insert(Fire { strength: 2.0 })
        .insert(Name::new("Fire"))
        .id();
    active_fires.insert(
        fire.id(),
        TransformBundle::default().local.translation.truncate(),
        2.0,
    );
    fire = commands
        .spawn_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0.5, 1.0, 0.0,
        )))
        .insert(Fire { strength: 1.0 })
        .insert(Name::new("Fire"))
        .id();
    active_fires.insert(fire.id(), Vec2::new(0.5, 1.0), 1.0);
    fire = commands
        .spawn_bundle(TransformBundle::from_transform(Transform::from_xyz(
            -1.5, 2.0, 0.0,
        )))
        .insert(Fire { strength: 4.0 })
        .insert(Name::new("Fire"))
        .id();
    active_fires.insert(fire.id(), Vec2::new(-1.5, 2.0), 4.0);
}

fn spawn_fire_overlay(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<FireMaterial>>,
    active_fires: Res<ActiveFires>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets
                .add(Mesh::from(shape::Quad::new(Vec2::splat(TILE_SIZE * 100.0))))
                .into(),
            material: material_assets.add(FireMaterial {
                active_fires: active_fires.get_fire_gpus(),
            }),
            ..default()
        })
        .insert(CameraFollower { offset: -10.0 })
        .insert(Name::new("Overlay"));
}

struct FireMaterialGpu {
    bind_group: BindGroup,
}

impl Material2d for FireMaterial {
    fn bind_group(
        material: &<Self as bevy::render::render_asset::RenderAsset>::PreparedAsset,
    ) -> &bevy::render::render_resource::BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(
        render_device: &bevy::render::renderer::RenderDevice,
    ) -> bevy::render::render_resource::BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        //TODO move to startup system (in main.rs?)
        //It would be nice to remove this on release
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("fire_overlay.wgsl"))
    }
}

impl RenderAsset for FireMaterial {
    type ExtractedAsset = FireMaterial;
    type PreparedAsset = FireMaterialGpu;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<FireMaterial>>,
        SRes<RenderQueue>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline, queue): &mut bevy::ecs::system::SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, render_asset::PrepareAssetError<Self::ExtractedAsset>> {
        //FIXME support gpus that don't support storages,
        // see how bevy handles gpu lights
        let mut storage_buffer = StorageBuffer::<FireGpu>::default();

        let mut data = extracted_asset.active_fires;
        storage_buffer.append(&mut data);

        storage_buffer.write_buffer(render_device, queue);

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: storage_buffer.binding().unwrap(),
            }],
        });

        Ok(FireMaterialGpu { bind_group })
    }
}
