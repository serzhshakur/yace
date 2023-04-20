use cucumber::{given, then, when, World};

#[derive(Debug, World, Default)]
#[world(init = Self::new)]
pub struct MyWorld {
    count: i32,
}

impl MyWorld {
    fn new() -> Self {
        Self { count: 0 }
    }
}

#[given(expr = "I start with {int} items")]
async fn init(world: &mut MyWorld, count: i32) {
    world.count = count;
}

#[when(expr = "I add {int} items")]
async fn add_count(world: &mut MyWorld, count: i32) {
    world.count += count;
}

#[then(expr = "I have {int} items")]
async fn check_result(world: &mut MyWorld, count: i32) {
    assert_eq!(world.count, count);
}
