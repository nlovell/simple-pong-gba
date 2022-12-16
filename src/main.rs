// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{display, syscall};

use agb::{include_aseprite,
    display::object::{Graphics, Tag}
};

//Import the sprites into this constant. This holds the sprite and palette data
// in a way that is manageable by AGB.

const GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");

// Define easy way of referencing the sprites

const PADDLE_END: &Tag = GRAPHICS.tags().get("Paddle End");
const PADDLE_MID: &Tag = GRAPHICS.tags().get("Paddle Mid");
const BALL: &Tag = GRAPHICS.tags().get("Ball");

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    //get the OAM manager
    let object = gba.display.object.get();

    //create an object with the ball sprite
    let mut ball = object.object_sprite(BALL.sprite(0));

    //place this somewhere on screen
    ball.set_x(50).set_y(50).show();

    let mut ball_x = 50;
    let mut ball_y = 50;
    let mut x_velocity = 1;
    let mut y_velocity = 1;

    loop {
        // this will calculate the new position and enforce the position of the ball
        // remains on screen at all times

        ball_x = (ball_x + x_velocity).clamp(0, agb::display::WIDTH - 16);
        ball_y = (ball_y + y_velocity).clamp(0, agb::display::HEIGHT - 16);

        //check if ball reaches edge of screen and reverse it's direction

        if ball_x == 0 || ball_x == agb::display::WIDTH-16 {
        x_velocity = -x_velocity
        }

        if ball_y == 0 || ball_y == agb::display::HEIGHT-16 {
                y_velocity = -y_velocity
        }

        //set position of ball to match new calculated position

       ball.set_x(ball_x as u16).set_y(ball_y as u16);

       agb::display::busy_wait_for_vblank();
       object.commit();
    }
}
