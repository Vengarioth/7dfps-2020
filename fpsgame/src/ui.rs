use bevy::{asset::LoadState, prelude::*};
pub struct SplashScreen{
    loaded: bool,
    fade_in: bool,
}

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
                .add(ColorMaterial {
                    texture: asset_server.load("images/TitleSplash.png").into(),
                    color: Color::hex("FFFFFF00").unwrap(),
                }),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        }).with(SplashScreen {
            fade_in: true,
            loaded: false,
        });
}

pub fn update_splash_screen(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut splash_query: Query<(Entity, &Handle<ColorMaterial>, &mut SplashScreen)>
){
    for (entity, material_handle, mut splash_screen) in splash_query.iter_mut()
    {
        let mat = materials.get_mut(material_handle).unwrap();
        if let Some(texture_handle) = &mat.texture {
            let texture_load_state = asset_server.get_load_state(texture_handle);
            match texture_load_state {
                LoadState::Loaded => {
                    splash_screen.loaded = true
                },
                _ => {},
            }
        }
        if splash_screen.loaded
        {
            if splash_screen.fade_in {
                mat.color.set_a(mat.color.a() + (0.5 * time.delta_seconds));
                if mat.color.a() >= 1.0 {
                    splash_screen.fade_in = false;
                }
            } else {
                mat.color.set_a(mat.color.a() - (0.5 * time.delta_seconds));
                if mat.color.a() <= 0.0 {
                    commands.despawn(entity);
                }
            }
        }
    }
}