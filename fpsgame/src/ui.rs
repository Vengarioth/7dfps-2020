use bevy::prelude::*;
pub struct SplashScreen;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        // splash
        .spawn(ImageComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            material: materials
                .add(asset_server.load("images/TitleSplash.png").into()),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        }).with(SplashScreen);
}

pub fn update_splash_screen(
    materials: ResMut<Assets<ColorMaterial>>,
    textures: Res<Assets<Texture>>,
    mut splash_query: Query<(&Handle<ColorMaterial>, &Handle<Texture>)>
){
    for(material_handle, texture) in splash_query.iter_mut()
    {
        let mat = materials.get(material_handle).unwrap();
        let new_mat = ColorMaterial{
            color: mat.color,
            texture: texture, 
        };
    }
}