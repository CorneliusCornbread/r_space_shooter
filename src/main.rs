extern crate amethyst;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },

    winit::
    {
        Event, 
        KeyboardInput, 
        VirtualKeyCode, 
        WindowEvent
    },

    utils::application_root_dir,
};

struct GameState;

fn main() -> amethyst::Result<()> 
{
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config = assets_dir.join("config/display.ron");

    
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
    )?;
    

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();
    Ok(())
}

impl SimpleState for GameState {
    fn on_start(&mut self, _: StateData<'_, GameData<'_, '_>>) 
    {
        println!("Starting game!");
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans 
    {
        if let StateEvent::Window(event) = &event 
        {
            match event 
            {
                Event::WindowEvent { event, .. } => match event 
                {
                    WindowEvent::KeyboardInput 
                    {
                        input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. }, ..
                    } |
                    WindowEvent::CloseRequested => Trans::Quit,
                    _ => Trans::None,
                },
                _ => Trans::None,
            }
        } 
        else 
        {
            Trans::None
        }
    }

    /*
    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans 
    {
        println!("Computing some more whoop-ass...");
        Trans::Quit
    }*/
}