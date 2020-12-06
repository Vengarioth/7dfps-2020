use bevy::{
    app::AppExit, 
    input::keyboard::KeyCode, 
    prelude::*,
};

pub fn toggle_cursor_and_exit(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: ResMut<Windows>,
    mut app_exit_events: ResMut<Events<AppExit>>,
){
    if let Some(window) = windows.get_primary_mut() {
        if keyboard_input.just_pressed(KeyCode::Escape)
        {
            if window.cursor_locked()
            {
                //unlock the cursor if it's locked
                window.set_cursor_lock_mode(false);
                window.set_cursor_visibility(true);
            }
            else
            {
                //exit the app if the cursor is unlocked
                app_exit_events.send(AppExit);
            }
        }

        if mouse_button_input.just_pressed(MouseButton::Left) && 
           !window.cursor_locked()
        {
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
        }
    }
}