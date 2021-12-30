use crate::Lines;

pub struct Solver {}

fn get_target_range(input: &str) -> (i32, i32) {
    let vec = input
        .trim_matches(',')
        .split("=")
        .skip(1)
        .next()
        .unwrap()
        .split("..")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    (vec[0], vec[1])
}

fn simulate(
    (mut dx, mut dy): (i32, i32),
    (tx_start, tx_end): (i32, i32),
    (ty_bottom, ty_top): (i32, i32),
) -> bool {
    let mut x = 0;
    let mut y = 0;

    loop {
        // Speed
        x += dx;
        y += dy;

        // Drag
        if dx > 0 {
            dx -= 1;
        } else if dx < 0 {
            dx += 1;
        }

        // Gravity
        dy -= 1;

        // Check HIT
        if x >= tx_start && x <= tx_end && y >= ty_bottom && y <= ty_top {
            return true;
        }

        // Check missed
        if (x > tx_end && dx >= 0) || (y < ty_bottom && dy <= 0) {
            return false;
        }
    }
}

// NOTE: target area is alway in positive x values and negative y values

fn find_boundaries(
    (tx_start, tx_end): (i32, i32),
    (ty_bottom, _): (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    // Maximum x reached for a given initial dx is the sum of integers from
    // 0 to dx. We solve "dx(dx+1) / 2 = tx_start" to find minimum dx that
    // will reach the target
    let dx_min = (1.0 / 2.0) * (f32::sqrt(8.0 * (tx_start as f32) + 1.0) - 1.0);
    let dx_min = dx_min.floor() as i32;

    // Max dx is tx_end, as if we shoot faster we will miss the target on the
    // first step and there is no way to turn around
    let dx_max = tx_end;

    // Min dy is ty_bottom, as if we shoot faster, we will miss the target on the
    // first step and gravity will only get us deeper
    let dy_min = ty_bottom;

    // If we only reason about y axis, when shooting up, the probe always
    // comes back at altitute 0 with a velocity of v = -dy - 1. To not
    // overshoot the target, we need |v| <= |ty_bottom|. Maximum velocity is the
    // v = ty_bottom.
    //
    // Solving -dy_max - 1 = ty_bottom we get:
    let dy_max = -ty_bottom - 1;

    ((dx_min, dx_max), (dy_min, dy_max))
}

impl crate::Solver for Solver {
    fn solve_part1(self: &mut Self, mut lines: Lines) -> String {
        let line = lines.next().unwrap();

        let mut coords = line.split(" ").skip(2);

        let target_x = get_target_range(coords.next().unwrap());
        let target_y = get_target_range(coords.next().unwrap());

        let ((dx_min, dx_max), (dy_min, dy_max)) = find_boundaries(target_x, target_y);

        let mut y_max = i32::MIN;

        for dy in dy_min..=dy_max {
            for dx in dx_min..=dx_max {
                if simulate((dx, dy), target_x, target_y) {
                    let max = if dy <= 0 { 0 } else { (dy * (dy + 1)) / 2 };
                    y_max = y_max.max(max);
                }
            }
        }

        y_max.to_string()
    }

    fn solve_part2(self: &mut Self, mut lines: Lines) -> String {
        let line = lines.next().unwrap();

        let mut coords = line.split(" ").skip(2);

        let target_x = get_target_range(coords.next().unwrap());
        let target_y = get_target_range(coords.next().unwrap());

        let ((dx_min, dx_max), (dy_min, dy_max)) = find_boundaries(target_x, target_y);

        let mut count = 0;

        for dx in dx_min..=dx_max {
            for dy in dy_min..=dy_max {
                if simulate((dx, dy), target_x, target_y) {
                    count += 1;
                }
            }
        }

        count.to_string()
    }
}
