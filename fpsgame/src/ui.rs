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
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut splash_query: Query<(Entity, &Handle<ColorMaterial>, &SplashScreen)>
){
    for (entity, material_handle, _splash_screen) in splash_query.iter_mut()
    {
        let mat = materials.get_mut(material_handle).unwrap();
        mat.color.set_a(mat.color.a() - 0.01);
        if mat.color.a() <= 0.0 {
            commands.despawn(entity);
        }
    }
}