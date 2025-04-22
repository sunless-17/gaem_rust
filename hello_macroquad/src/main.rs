use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 200.0;

// track squares and circles
#[derive(Debug)]
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}
impl Shape {
    // register collisions
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }
    // wtf?
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("bullet_hell")]
async fn main() {
    // time per frame, usefull in adjusting stuffs per frame
    let delta_time = get_frame_time();

    // player values
    let mut circle = Shape {
        size: 16.0,
        speed: MOVEMENT_SPEED,
        // make sure the position is at the center
        // - 50.0 bottom
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };

    // enemy squares
    let mut squares = vec![];

    // default game not over
    let mut gameover = false;

    // bullets
    let mut bullets = vec![];

    // repeat frames infinitely
    loop {
        // defaults to black
        clear_background(DARKPURPLE);

        // create new square values and push to vec
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
                collided: false,
            });
        }
        // debug production of the enemies
        println!("{:?}", squares);

        // shooting player
        if is_key_pressed(KeyCode::Space) {
            bullets.push(Shape {
                x: circle.x,
                y: circle.y,
                speed: circle.speed * 2.0,
                size: 5.0,
                collided: false,
            });
        }

        // change struct collided value of squares and bullet
        for square in squares.iter_mut() {
            for bullet in bullets.iter_mut() {
                if bullet.collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                }
            }
        }

        // check bullet collision and remove the square
        squares.retain(|square| !square.collided);
        bullets.retain(|bullet| !bullet.collided);

        // check collision
        // TODO: check circle
        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        // game over screen
        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        // reset game
        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        // if gameover == true freeze the movement speed of player and creation of squares
        if !gameover {
            // TODO: rewrite using match
            if is_key_down(KeyCode::Right) {
                circle.x += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += MOVEMENT_SPEED * delta_time;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= MOVEMENT_SPEED * delta_time;
            }
            // create more squares with new speeds based on the delta time and shape speeds
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            // bullet movement
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }
        }

        // drawing player
        draw_circle(circle.x, circle.y, circle.size, RED);

        // drawing the squares based on the values on the vector
        // TODO: changing colors
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                BLUE,
            );
        }

        // drawing bullets
        // TODO: draw circle lines for outline
        // TODO: reloading time
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }

        // checks whether the values in the vector will be kept (if the y ordinates are less, will be cleaned)
        squares.retain(|square| square.y < screen_height() + square.size);
        bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

        // the size of the player is never zero (avoid going off-screen)
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        // completes the first frame
        next_frame().await
    }
}
