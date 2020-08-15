use specs::{Component, NullStorage, VecStorage, World, WorldExt};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub paths: Vec<String>,
}

impl Renderable {
    pub fn new_static(path: String) -> Self {
        Self { paths: vec![path] }
    }

    pub fn new_animated(paths: Vec<String>) -> Self {
        Self { paths }
    }
    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("invalid renderabled"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn path(&self, path_index: usize) -> String {
        // Mod the path index with size
        self.paths[path_index % self.paths.len()].clone()
    }
}

pub enum RenderableKind {
    Static,
    Animated,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(PartialEq)]
pub enum BoxColour {
    Red,
    Blue,
    Grey,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub colour: BoxColour,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub colour: BoxColour,
}

impl Display for BoxColour {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
            BoxColour::Grey => "grey",
        })?;
        Ok(())
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

// Register components with the world
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}
