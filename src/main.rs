extern crate specs;
#[macro_use]
extern crate specs_derive;

use specs::Component; 
use specs::VecStorage;
use specs::World;
use specs::System;
use specs::ReadStorage;
use specs::WriteStorage;
use specs::RunNow;
use specs::DispatcherBuilder;

#[derive(Component, Debug)]
#[component(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[component(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;
        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>);
    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        use specs::Join;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();

    let ball = world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    let mut hello_world = HelloWorld;
    //hello_world.run_now(&world.res);
    
    let mut dispatcher = DispatcherBuilder::new()
        .add(hello_world, "hello_world", &[])
        .build();
    


}