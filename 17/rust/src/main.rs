use itertools::Itertools;

static TARGET_X_START: i32 = 60;
static TARGET_X_END: i32 = 94;
static TARGET_Y_END: i32 = -171;
static TARGET_Y_START: i32 = -136;

fn main() {
    let valid_inital_velocities: Vec<(i32, i32)> = (0..100)
        .cartesian_product(-171..171)
        .filter(|(dx, dy)| valid_initial_velocity(*dx, *dy))
        .collect();
    let max_height = valid_inital_velocities
        .iter()
        .map(|(_, dy)| max_height(*dy))
        .max()
        .expect("invalid input");
    println!("part 1: {}", max_height);
    println!("part 2: {}", valid_inital_velocities.len());
}

fn valid_initial_velocity(mut dx: i32, mut dy: i32) -> bool {
    let (mut x, mut y) = (0, 0);
    while x <= TARGET_X_END && y >= TARGET_Y_END {
        if x >= TARGET_X_START && y <= TARGET_Y_START {
            return true;
        }
        x += dx;
        y += dy;
        dy -= 1;
        dx = i32::max(dx - 1, 0);
    }
    false
}

fn max_height(dy: i32) -> i32 {
    (1..=dy).sum()
}
