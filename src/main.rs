use pancurses::{endwin, Input, Window};

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}
//{{{
struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}
//}}}
struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(window: &Window) -> Result<Game, String> {
        let (max_y, max_x) = window.get_max_yx();
        if max_y < 1 || max_x < 10 {
            return Err(String::from("Window is too small, exiting."));
        }

        let frame = Frame {
            height: max_y as u32 - 2,
            width: max_x as u32 - 2,
        };
        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        };

        Ok(Game { frame, ball })
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }

        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

fn main() -> Result<(), String> {
    let window = pancurses::initscr();
    window.timeout(33);

    let mut game = Game::new(&window)?;

    loop {
        window.clear();
        window.border('|', '|', '-', '-', '+', '+', '+', '+');
        window.mvaddch(game.ball.y as i32 + 1, game.ball.x as i32 + 1, 'o');
        window.mv(0, 0);
        window.refresh();

        match window.getch() {
            Some(Input::Character('q')) => {
                endwin();
                println!("Thanks for playing!");
                return Ok(());
            }
            Some(Input::KeyResize) => {
                game = Game::new(&window)?;
            }
            _ => game.step(),
        }
    }
}
