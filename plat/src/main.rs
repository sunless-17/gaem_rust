use macroquad::prelude::*;

enum State {
  Start,
  Stop,
}

struct Player {
  pos: u8,
  spd: u8,
  state: State,
}

impl Player {
  fn new() -> Player {
    let mut state_machine = StateMachine::new();

    // update function will be called on each state_machine.update()
    state.insert(Self::ST_NORMAL, State::new().update(Self::update_normal));

    // coroutine will be started each time player enter dash state
    state.insert(
      Self::ST_DASH,
      State::new()
        .update(Self::update_dash)
        .coroutine(Self::dash_coroutine),
    );
    Player {
      pos: 8,
      spd: 8,
      state: State::Start,
    }
  }

  // dash state
  fn dash(&mut self) {
    self.dashes = 0;
    // during the dash player has completely different behaviour
    // changing the playr's state to DASH
    self.state_machine.set_state(Self::ST_DASH);
  }

  //jumping state
  fn jump(&mut self) {
    self.spd.y = self.jump_speed;
    // during jump player is behaving exactly as usual
    // so the state do not change here
  }

  // normal movement
  fn update_normal(&mut self, room: &mut Room, dt: f32) {
    if is_key_pressed(KeyCode::A) {
      self.start_dash();
      return;
    }

    if is_key_pressed(KeyCode::S) {
      self.jump();
      return;
    }

    // running
    todo!();

    // gravity
    todo!();
  }

  // game over and game state
  fn update(&mut self, room: &mut Room) {
    // physics: apply spd to self.pos with some collisions
    todo!();

    // win conditions check
    todo!();

    // lose conditions check
    todo!();

    // camera update
    todo!();

    self.state_machine.update(room);
  }
}

// animation controller

#[macroquad::main("platt")]
async fn main() {
  // repeat frames infinitely
  loop {
    // defaults to black
    clear_background(DARKPURPLE);

    // completes the first frame
    next_frame().await
  }
}
