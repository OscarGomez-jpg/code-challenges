use macroquad::prelude::*;

fn generate_points(root: &Vec2, is_h: bool) -> (Vec2, Vec2) {
    if is_h {
        (
            vec2(root.x - root.x / 8.0, root.y),
            vec2(root.x + root.x / 8.0, root.y),
        )
    } else {
        (
            vec2(root.x, root.y - root.y / 8.0),
            vec2(root.x, root.y + root.y / 8.0),
        )
    }
}

fn make_tree(tree: &mut Vec<(Vec2, Vec2)>, depth: i32, is_h: bool) {
    if depth - 1 <= 0 {
        return;
    }

    let root = tree.len() - 1;

    let (first, second) = generate_points(&tree[root].0, is_h);
    tree.push((first, second));

    let (first, second) = generate_points(&tree[root].1, is_h);
    tree.push((first, second));

    make_tree(tree, depth - 1, !is_h);
    make_tree(tree, depth - 1, !is_h);
}

#[macroquad::main("Binary tree")]
async fn main() {
    let first: Vec2 = vec2(
        screen_width() / 2.,
        screen_height() / 2. + screen_height() / 4.,
    );
    let second: Vec2 = vec2(
        screen_width() / 2.,
        screen_height() / 2. - screen_height() / 4.,
    );
    let mut tree: Vec<(Vec2, Vec2)> = Vec::new();
    tree.push((first, second));

    make_tree(&mut tree, 5, true);

    //println!("{:?}", tree);

    loop {
        for set in &tree {
            draw_line(set.0.x, set.0.y, set.1.x, set.1.y, 1.1, WHITE);
        }

        next_frame().await;
    }
}
