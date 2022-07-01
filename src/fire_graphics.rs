use bevy::ecs::system::lifetimeless::SRes;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::{self, RenderAsset, RenderAssets};
use bevy::render::render_resource::std140::{AsStd140, Std140};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::{render_resource::*, RenderApp, RenderStage};
use bevy::sprite::{Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle};
use bevy::{prelude::*, sprite::Material2d};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::collections::HashMap;

use crate::item::WorldObject;
use crate::prelude::*;

pub struct FireGraphicsPlugin;

pub const MAX_FIRES: usize = 64;

#[derive(Clone, Component, Default, AsStd140, Copy)]
struct FireGpu {
    position: Vec2,
    strength: f32,
}
#[derive(Clone, Component, Inspectable)]
struct Fire {
    strength: f32,
}

///Resource containing all active fires
#[derive(Clone, Default)]
pub struct ActiveFires {
    fires: HashMap<Entity, FireGpu>,
}

impl ActiveFires {
    fn insert(&mut self, fire_entity: Entity, position: Vec2, strength: f32) {
        self.fires
            .insert(fire_entity, FireGpu { position, strength });
    }
    fn remove(&mut self, fire_entity: Entity) {
        self.fires.remove(&fire_entity);
    }

    fn get_fire_gpus(&self) -> Vec<FireGpu> {
        self.fires.values().cloned().collect()
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
            .add_startup_system(spawn_fire_overlay)
            .register_inspectable::<Fire>();
        app.sub_app_mut(RenderApp)
            .add_system_to_stage(RenderStage::Extract, extract_fire)
            .add_system_to_stage(RenderStage::Queue, prepare_fire);
    }
}

fn extract_fire(mut commands: Commands, fires: Res<ActiveFires>) {
    commands.insert_resource(fires.clone());
}

fn prepare_fire(
    render_queue: Res<RenderQueue>,
    active_fires: Res<ActiveFires>,
    assets: Res<RenderAssets<FireMaterial>>,
) {
    for asset in assets.values() {
        let mut fires = [FireGpu::default(); MAX_FIRES];
        let data = active_fires.get_fire_gpus();
        for (i, fire) in data.iter().enumerate() {
            fires[i] = *fire;
        }
        render_queue.write_buffer(&asset.buffer, 0, fires.as_std140().as_bytes());
    }
}

fn update_fire_overlay(
    changed_fires: Query<
        (Entity, &Fire, &Transform),
        Or<((Changed<Transform>, With<Fire>), Changed<Fire>)>,
    >,
    mut active_fires: ResMut<ActiveFires>,
) {
    for (entity, fire, transform) in changed_fires.iter() {
        active_fires.insert(entity, transform.translation.truncate(), fire.strength);
    }
}

fn remove_fire_from_overlay(
    removed_fire: RemovedComponents<Fire>,
    mut active_fires: ResMut<ActiveFires>,
) {
    for fire in removed_fire.iter() {
        active_fires.remove(fire);
    }
}

pub fn spawn_fire(
    commands: &mut Commands,
    active_fires: &mut ResMut<ActiveFires>,
    graphics: &Res<Graphics>,
    position: Vec2,
) {
    let fire = WorldObject::CampFire.spawn(commands, graphics, position);

    commands
        .entity(fire)
        .insert(Fire { strength: 7.0 })
        .insert(Name::new("Fire"));

    active_fires.insert(fire, position, 7.0);
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
    buffer: Buffer,
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
                    ty: BufferBindingType::Uniform,
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
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<FireMaterial>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline): &mut bevy::ecs::system::SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, render_asset::PrepareAssetError<Self::ExtractedAsset>> {
        let mut fires = [FireGpu::default(); MAX_FIRES];
        let data = extracted_asset.active_fires;
        for (i, fire) in data.iter().enumerate() {
            fires[i] = *fire;
        }

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: fires.as_std140().as_bytes(),
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Ok(FireMaterialGpu { bind_group, buffer })
    }
}
