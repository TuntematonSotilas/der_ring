use bevy::prelude::*;

fn main() {
     App::new()
          .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    //resolution: (500., 300.).into(),
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    
                    ..default()
                }),
                ..default() 
          })).run();
}
