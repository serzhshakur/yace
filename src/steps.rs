use cucumber::{given, then, when};

use crate::world::MyWorld;

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
