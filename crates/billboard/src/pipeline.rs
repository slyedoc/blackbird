use crate::text::RenderBillboard;
use crate::{BILLBOARD_SHADER_HANDLE, Billboard};
use bevy::asset::AssetId;
use bevy::core_pipeline::core_3d::Transparent3d;
use bevy::ecs::query::ROQueryItem;
use bevy::ecs::system::lifetimeless::{Read, SRes};
use bevy::ecs::system::{SystemParamItem, SystemState};
use bevy::image::BevyDefault;
use bevy::log::error;
use bevy::math::Mat4;
use bevy::prelude::{
    AssetEvent, Commands, Component, Entity, FromWorld, Image, Mesh, Msaa, Query, Res, ResMut,
    Resource, With, World, default,
};
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::mesh::allocator::MeshAllocator;
use bevy::render::mesh::{
    MeshVertexBufferLayoutRef, PrimitiveTopology, RenderMesh, RenderMeshBufferInfo,
};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    DrawFunctions, PhaseItemExtraIndex, RenderCommand, RenderCommandResult, SetItemPipeline,
    TrackedRenderPass, ViewSortedRenderPhases,
};
use bevy::render::render_resource::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingResource, BindingType,
    BlendComponent, BlendFactor, BlendOperation, BlendState, BufferBindingType, ColorTargetState,
    ColorWrites, CompareFunction, DepthStencilState, FragmentState, FrontFace, MultisampleState,
    PipelineCache, PolygonMode, PrimitiveState, RenderPipelineDescriptor, SamplerBindingType,
    ShaderStages, ShaderType, SpecializedMeshPipeline, SpecializedMeshPipelineError,
    SpecializedMeshPipelines, TextureFormat, TextureSampleType, TextureViewDimension, VertexState,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::GpuImage;
use bevy::render::view::{
    ExtractedView, RenderVisibleEntities, ViewTarget, ViewUniform, ViewUniformOffset, ViewUniforms,
};
use bevy::sprite::SpriteAssetEvents;
use bevy::utils;

#[derive(Clone, Copy, ShaderType, Component)]
pub struct BillboardUniform {
    pub(crate) transform: Mat4,
}

#[derive(Clone, Copy, Component, Debug)]
pub struct RenderBillboardMesh {
    pub id: AssetId<Mesh>,
}

#[derive(Clone, Copy, Component, Debug)]
pub struct RenderBillboardImage {
    pub id: AssetId<Image>,
}

#[derive(Resource, Default)]
pub struct BillboardImageBindGroups {
    values: utils::HashMap<AssetId<Image>, BindGroup>,
}

#[derive(Resource)]
pub struct BillboardBindGroup {
    value: BindGroup,
}

#[derive(Component)]
pub struct BillboardViewBindGroup {
    value: BindGroup,
}

// Reference:
// https://github.com/bevyengine/bevy/blob/release-0.9.1/crates/bevy_sprite/src/mesh2d/mesh.rs#L282
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    // NOTE: Apparently quadro drivers support up to 64x MSAA.
    // MSAA uses the highest 3 bits for the MSAA log2(sample count) to support up to 128x MSAA.
    pub struct BillboardPipelineKey: u32 {
        const TEXT               = 0;
        const TEXTURE            = (1 << 0);
        const DEPTH              = (1 << 1);
        const LOCK_Y             = (1 << 2);
        const LOCK_ROTATION      = (1 << 3);
        const HDR                = (1 << 4);
        const MSAA_RESERVED_BITS = Self::MSAA_MASK_BITS << Self::MSAA_SHIFT_BITS;
    }
}

impl BillboardPipelineKey {
    const MSAA_MASK_BITS: u32 = 0b111;
    const MSAA_SHIFT_BITS: u32 = 32 - Self::MSAA_MASK_BITS.count_ones();

    pub fn from_msaa_samples(msaa_samples: u32) -> Self {
        let msaa_bits =
            (msaa_samples.trailing_zeros() & Self::MSAA_MASK_BITS) << Self::MSAA_SHIFT_BITS;
        Self::from_bits_retain(msaa_bits)
    }
    pub fn msaa_samples(&self) -> u32 {
        1 << ((self.bits() >> Self::MSAA_SHIFT_BITS) & Self::MSAA_MASK_BITS)
    }
}

pub fn prepare_billboard_view_bind_groups(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    billboard_pipeline: Res<BillboardPipeline>,
    view_uniforms: Res<ViewUniforms>,
    views: Query<Entity, With<ExtractedView>>,
) {
    let Some(binding) = view_uniforms.uniforms.binding() else {
        return;
    };

    for entity in views.iter() {
        commands.entity(entity).insert(BillboardViewBindGroup {
            value: render_device.create_bind_group(
                Some("billboard_view_bind_group"),
                &billboard_pipeline.view_layout,
                &[BindGroupEntry {
                    binding: 0,
                    resource: binding.clone(),
                }],
            ),
        });
    }
}

pub fn prepare_billboard_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    billboard_pipeline: Res<BillboardPipeline>,
    billboard_uniforms_buffer: Res<ComponentUniforms<BillboardUniform>>,
) {
    let Some(binding) = billboard_uniforms_buffer.uniforms().binding() else {
        return;
    };

    commands.insert_resource(BillboardBindGroup {
        value: render_device.create_bind_group(
            Some("billboard_bind_group"),
            &billboard_pipeline.billboard_layout,
            &[BindGroupEntry {
                binding: 0,
                resource: binding,
            }],
        ),
    });
}

pub fn queue_billboard_texture(
    mut views: Query<(Entity, &ExtractedView, &RenderVisibleEntities, &Msaa)>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    mut image_bind_groups: ResMut<BillboardImageBindGroups>,
    mut billboard_pipelines: ResMut<SpecializedMeshPipelines<BillboardPipeline>>,
    render_device: Res<RenderDevice>,
    transparent_draw_functions: Res<DrawFunctions<Transparent3d>>,
    billboard_pipeline: Res<BillboardPipeline>,
    (gpu_images, gpu_meshes): (Res<RenderAssets<GpuImage>>, Res<RenderAssets<RenderMesh>>),
    events: Res<SpriteAssetEvents>,
    billboards: Query<(
        &BillboardUniform,
        &RenderBillboardMesh,
        &RenderBillboardImage,
        &RenderBillboard,
    )>,
) {
    // If an image has changed, the GpuImage has (probably) changed
    for event in &events.images {
        match event {
            AssetEvent::Unused { .. }
            | AssetEvent::Added { .. }
            | AssetEvent::LoadedWithDependencies { .. } => None,
            AssetEvent::Modified { id } | AssetEvent::Removed { id } => {
                image_bind_groups.values.remove(id)
            }
        };
    }

    for (view_entity, view, visible_entities, msaa) in &mut views {
        let Some(transparent_phase) = transparent_render_phases.get_mut(&view_entity) else {
            continue;
        };

        let draw_transparent_billboard = transparent_draw_functions
            .read()
            .get_id::<DrawBillboard>()
            .unwrap();

        let rangefinder = view.rangefinder3d();

        for visible_entity in visible_entities.iter::<With<Billboard>>() {
            let Ok((uniform, mesh, image, billboard)) = billboards.get(visible_entity.0) else {
                continue;
            };
            let Some(gpu_image) = gpu_images.get(image.id) else {
                continue;
            };
            let Some(gpu_mesh) = gpu_meshes.get(mesh.id) else {
                continue;
            };

            let mut key = BillboardPipelineKey::from_msaa_samples(msaa.samples());

            if billboard.depth.0 {
                key |= BillboardPipelineKey::DEPTH;
            }

            if billboard.lock_axis.map_or(false, |lock| lock.y_axis) {
                key |= BillboardPipelineKey::LOCK_Y;
            }
            if billboard.lock_axis.map_or(false, |lock| lock.rotation) {
                key |= BillboardPipelineKey::LOCK_ROTATION;
            }

            if view.hdr {
                key |= BillboardPipelineKey::HDR;
            }

            let pipeline_id = billboard_pipelines.specialize(
                &mut pipeline_cache,
                &billboard_pipeline,
                key,
                &gpu_mesh.layout,
            );

            let pipeline_id = match pipeline_id {
                Ok(id) => id,
                Err(err) => {
                    error!("{err:?}");
                    continue;
                }
            };

            let distance = rangefinder.distance(&uniform.transform);

            image_bind_groups.values.entry(image.id).or_insert_with(|| {
                render_device.create_bind_group(
                    Some("billboard_texture_bind_group"),
                    &billboard_pipeline.texture_layout,
                    &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::TextureView(&gpu_image.texture_view),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::Sampler(&gpu_image.sampler),
                        },
                    ],
                )
            });

            transparent_phase.add(Transparent3d {
                pipeline: pipeline_id,
                entity: *visible_entity,
                draw_function: draw_transparent_billboard,
                batch_range: 0..1,
                extra_index: PhaseItemExtraIndex::NONE,
                distance,
            });
        }
    }
}

#[derive(Resource, Clone)]
pub struct BillboardPipeline {
    view_layout: BindGroupLayout,
    billboard_layout: BindGroupLayout,
    texture_layout: BindGroupLayout,
}

impl FromWorld for BillboardPipeline {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(Res<RenderDevice>,)> = SystemState::new(world);

        let (render_device,) = system_state.get(world);

        let view_layout = render_device.create_bind_group_layout(
            "billboard_view_layout",
            &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(ViewUniform::min_size()),
                },
                count: None,
            }],
        );

        let billboard_layout = render_device.create_bind_group_layout(
            "billboard_layout",
            &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(BillboardUniform::min_size()),
                },
                count: None,
            }],
        );

        let texture_layout = render_device.create_bind_group_layout(
            "billboard_texture_layout",
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        );

        Self {
            view_layout,
            billboard_layout,
            texture_layout,
        }
    }
}

impl SpecializedMeshPipeline for BillboardPipeline {
    type Key = BillboardPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        const DEF_VERTEX_COLOR: &str = "VERTEX_COLOR";
        const DEF_LOCK_Y: &str = "LOCK_Y";
        const DEF_LOCK_ROTATION: &str = "LOCK_ROTATION";

        let mut shader_defs = Vec::with_capacity(4);
        let mut attributes = Vec::with_capacity(4);

        attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
        attributes.push(Mesh::ATTRIBUTE_UV_0.at_shader_location(1));

        let layout = layout.0.as_ref();

        if layout.contains(Mesh::ATTRIBUTE_COLOR) {
            shader_defs.push(DEF_VERTEX_COLOR.into());
            attributes.push(Mesh::ATTRIBUTE_COLOR.at_shader_location(2));
        }

        let vertex_buffer_layout = layout.get_layout(&attributes)?;

        let depth_compare = if key.contains(BillboardPipelineKey::DEPTH) {
            CompareFunction::Greater
        } else {
            CompareFunction::Always
        };

        if key.contains(BillboardPipelineKey::LOCK_Y) {
            shader_defs.push(DEF_LOCK_Y.into());
        }
        if key.contains(BillboardPipelineKey::LOCK_ROTATION) {
            shader_defs.push(DEF_LOCK_ROTATION.into());
        }

        Ok(RenderPipelineDescriptor {
            label: Some("billboard_pipeline".into()),
            layout: vec![
                self.view_layout.clone(),
                self.billboard_layout.clone(),
                self.texture_layout.clone(),
            ],
            vertex: VertexState {
                shader: BILLBOARD_SHADER_HANDLE,
                entry_point: "vertex".into(),
                buffers: vec![vertex_buffer_layout],
                shader_defs: shader_defs.clone(),
            },
            fragment: Some(FragmentState {
                shader: BILLBOARD_SHADER_HANDLE,
                entry_point: "fragment".into(),
                shader_defs,
                targets: vec![Some(ColorTargetState {
                    format: if key.contains(BillboardPipelineKey::HDR) {
                        ViewTarget::TEXTURE_FORMAT_HDR
                    } else {
                        TextureFormat::bevy_default()
                    },
                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::One,
                            operation: BlendOperation::Add,
                        },
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare,
                stencil: default(),
                bias: default(),
            }),
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: false,
        })
    }
}

pub struct SetBillboardViewBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent3d> for SetBillboardViewBindGroup<I> {
    type Param = ();
    type ViewQuery = (Read<ViewUniformOffset>, Read<BillboardViewBindGroup>);
    type ItemQuery = ();

    fn render<'w>(
        _item: &Transparent3d,
        (view_uniform, billboard_mesh_bind_group): ROQueryItem<'w, Self::ViewQuery>,
        _item_query: Option<ROQueryItem<'w, Self::ItemQuery>>,
        _param: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(I, &billboard_mesh_bind_group.value, &[view_uniform.offset]);

        RenderCommandResult::Success
    }
}

pub struct SetBillboardBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent3d> for SetBillboardBindGroup<I> {
    type Param = SRes<BillboardBindGroup>;
    type ViewQuery = ();
    type ItemQuery = Read<DynamicUniformIndex<BillboardUniform>>;

    fn render<'w>(
        _item: &Transparent3d,
        _view: ROQueryItem<'w, Self::ViewQuery>,
        billboard_index: Option<ROQueryItem<'w, Self::ItemQuery>>,
        billboard_bind_group: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let billboard_bind_group = billboard_bind_group.into_inner();

        let Some(billboard_index) = billboard_index else {
            return RenderCommandResult::Skip;
        };

        pass.set_bind_group(I, &billboard_bind_group.value, &[billboard_index.index()]);

        RenderCommandResult::Success
    }
}

pub struct SetBillboardTextureBindGroup<const I: usize>;
impl<const I: usize> RenderCommand<Transparent3d> for SetBillboardTextureBindGroup<I> {
    type Param = SRes<BillboardImageBindGroups>;
    type ViewQuery = ();
    type ItemQuery = Read<RenderBillboardImage>;

    fn render<'w>(
        _item: &Transparent3d,
        _view: ROQueryItem<'w, Self::ViewQuery>,
        billboard_texture: Option<ROQueryItem<'w, Self::ItemQuery>>,
        images: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let images = images.into_inner();

        let Some(billboard_texture) = billboard_texture else {
            return RenderCommandResult::Skip;
        };

        let bind_group = images.values.get(&billboard_texture.id).unwrap();
        pass.set_bind_group(I, bind_group, &[]);

        RenderCommandResult::Success
    }
}

pub struct DrawBillboardMesh;
impl RenderCommand<Transparent3d> for DrawBillboardMesh {
    type Param = (SRes<RenderAssets<RenderMesh>>, SRes<MeshAllocator>);
    type ViewQuery = ();
    type ItemQuery = Read<RenderBillboardMesh>;

    fn render<'w>(
        _item: &Transparent3d,
        _view: ROQueryItem<'w, Self::ViewQuery>,
        mesh: Option<ROQueryItem<'w, Self::ItemQuery>>,
        (meshes, mesh_allocator): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Some(mesh) = mesh else {
            return RenderCommandResult::Skip;
        };

        let Some(gpu_mesh) = meshes.into_inner().get(mesh.id) else {
            return RenderCommandResult::Skip;
        };

        let mesh_allocator = mesh_allocator.into_inner();
        let Some(vertex_buffer_slice) = mesh_allocator.mesh_vertex_slice(&mesh.id) else {
            return RenderCommandResult::Skip;
        };

        pass.set_vertex_buffer(0, vertex_buffer_slice.buffer.slice(..));

        match &gpu_mesh.buffer_info {
            RenderMeshBufferInfo::Indexed {
                index_format,
                count,
            } => {
                let Some(index_buffer_slice) = mesh_allocator.mesh_index_slice(&mesh.id) else {
                    return RenderCommandResult::Skip;
                };

                pass.set_index_buffer(index_buffer_slice.buffer.slice(..), 0, *index_format);
                pass.draw_indexed(
                    index_buffer_slice.range.start..(index_buffer_slice.range.start + count),
                    vertex_buffer_slice.range.start as i32,
                    0..1,
                );
            }
            RenderMeshBufferInfo::NonIndexed => {
                pass.draw(
                    vertex_buffer_slice.range.start
                        ..(vertex_buffer_slice.range.start + gpu_mesh.vertex_count),
                    0..1,
                );
            }
        }

        RenderCommandResult::Success
    }
}

pub type DrawBillboard = (
    SetItemPipeline,
    SetBillboardViewBindGroup<0>,
    SetBillboardBindGroup<1>,
    SetBillboardTextureBindGroup<2>,
    DrawBillboardMesh,
);
