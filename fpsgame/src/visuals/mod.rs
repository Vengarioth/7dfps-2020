
use bevy::{prelude::*, render::{mesh::shape, pipeline::{BlendDescriptor, BlendFactor, BlendOperation, ColorStateDescriptor, ColorWrite, PipelineDescriptor, RenderPipeline}, render_graph::{AssetRenderResourcesNode, RenderGraph, SystemNode, base}, renderer::RenderResources, shader::{asset_shader_defs_system, ShaderDefs, ShaderStage, ShaderStages}, texture::TextureFormat}, type_registry::TypeUuid};

#[derive(RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "620f651b-adbe-464b-b740-ba0e547282bb"]
pub struct PetBodyMaterial {
    pub color0: Color,
    pub color1: Color,
    pub color2: Color,
    pub texture: Option<Handle<Texture>>,
}

fn prepare_pipeline<T>(
    material_name: &'static str, glsl: &str, 
    shaders: &mut Assets<Shader>, 
    graph: &mut RenderGraph, 
    pipelines: &mut Assets<PipelineDescriptor>) -> Handle<PipelineDescriptor>
where T : RenderResources + TypeUuid {
    let mut glsl = glsl.split("//FRAGMENT_SHADER");
    let vert = glsl.next().unwrap();
    let frag = glsl.next().expect("glsl file must contain \"//FRAGMENT_SHADER\" comment indicating where to split");

    let pipeline_desc = PipelineDescriptor::default_config(ShaderStages {
        vertex:        shaders.add(Shader::from_glsl(ShaderStage::Vertex, vert)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, frag))),
    });

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to our shader
    graph.add_system_node(
        material_name,
        AssetRenderResourcesNode::<T>::new(true), //this <T> is somehow related to Assets<T>
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures the material runs before the main pass
    graph
        .add_node_edge(material_name, base::node::MAIN_PASS)
        .unwrap();

    pipelines.add(pipeline_desc)
}

#[derive(Default, TypeUuid)]
#[uuid = "cdda7e26-8285-4e8b-bcb0-6873917dc248"]
pub struct PetVisualsShared {
    pipeline_petbody: Handle<PipelineDescriptor>,
    pipeline_petface: Handle<PipelineDescriptor>, //not available yet

    // ear_meshes:  Vec<Handle<Mesh>>,
    // foot_meshes: Vec<Handle<Mesh>>,
    // //... 

    // pattern_textures: Vec<Handle<Texture>>,
}

pub fn setup_pet_visuals(
    mut commands: Commands,
    mut petvisuals: ResMut<PetVisualsShared>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PetBodyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    //i'll deal with file loading later ok?
    let glsl = include_str!("../../assets/shaders/pet_body.glsl");
    petvisuals.pipeline_petbody = prepare_pipeline::<PetBodyMaterial>(
        "PetBodyMaterial", glsl, &mut shaders, &mut render_graph, &mut pipelines);

    //TODO: do the same for Pet-Face like below
    // let glsl = include_str!("../../assets/shaders/pet_face.glsl");
    // petvisuals.pipeline_petface = prepare_pipeline::<PetFaceMaterial>(
    //     "PetFaceMaterial", glsl, &mut shaders, &mut render_graph, &mut pipelines);

    //TODO: load textures, meshes into PetVisualsShared
}