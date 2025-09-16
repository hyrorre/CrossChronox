use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::gamepad::gilrs::GilrsBuilder;
use ggez::input::gamepad::{GamepadContext, Gilrs};
use ggez::winit::event::MouseButton;
use ggez::winit::keyboard::{KeyCode, PhysicalKey};
use ggez::{Context, ContextBuilder, GameResult};

fn main() -> Result<(), ggez::GameError> {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game)
}

struct MyGame {
    gilrs: Gilrs,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let gilrs = GilrsBuilder::new().with_default_filters(false).build().unwrap();
        MyGame { gilrs }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...
        while let Some(event) = self.gilrs.next_event() {
            println!("{:?}", event);
        }

        if ctx.mouse.button_just_pressed(MouseButton::Left) {
            println!("Left Click");
        }

        if ctx.mouse.button_just_pressed(MouseButton::Right) {
            println!("Right Click");
        }

        if ctx.keyboard.is_physical_key_just_pressed(&PhysicalKey::Code(KeyCode::Enter)) {
            println!("Enter");
        }

        if ctx.keyboard.is_physical_key_just_pressed(&PhysicalKey::Code(KeyCode::Space)) {
            println!("Space");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }
}
