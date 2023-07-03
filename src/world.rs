use cucumber::World;

#[derive(Debug, World, Default)]
#[world(init = Self::new)]
pub struct MyWorld {
    pub count: i32,
}

impl MyWorld {
    fn new() -> Self {
        Self { count: 0 }
    }
}
