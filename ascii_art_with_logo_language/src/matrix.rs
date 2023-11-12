#[derive(Debug)]
pub enum Instruction {
    ClearScreen(char),
    SetChar(char),
    Forward(i16),
    Right(i16),
    Left(i16),
    PenUp,
    PenDown,
}

#[derive(Default, Debug)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,
    default_char: char,
    draw_char: char,
    drawing: bool,
    direction: i16,
    position: Position,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![]],
            default_char: ' ',
            draw_char: '#',
            direction: 0,
            drawing: true,
            position: Default::default(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearScreen(default_char) => self.default_char = default_char,
            Instruction::SetChar(draw_char) => self.draw_char = draw_char,
            Instruction::Forward(count) => self.go_forward(count),
            Instruction::Right(degrees) => self.direction = (self.direction + degrees) % 360,
            Instruction::Left(degrees) => self.direction -= degrees,
            Instruction::PenUp => self.drawing = false,
            Instruction::PenDown => self.drawing = true,
        };
        while self.direction < 0 {
            self.direction += 360;
        }
    }

    pub fn display_grid(&self) {
        for line in self.grid.iter() {
            println!("{}", line.iter().collect::<String>());
        }
    }
}

impl Grid {
    fn go_forward(&mut self, count: i16) {
        let direction = (self.direction / 90).abs();
        for _ in 0..count {
            if self.drawing {
                self.draw_on_position();
            }
            if direction % 2 == 0 {
                let up_or_down = direction - 1;
                self.position.y += up_or_down;
            } else {
                let left_or_right = -(direction - 2);
                self.position.x += left_or_right;
            }
        }
    }

    fn draw_on_position(&mut self) {
        if self.position.y < 0 {
            while self.position.y < 0 {
                self.grid.insert(0, vec![]);
                self.position.y += 1;
            }
        } else if self.grid.len() <= self.position.y as usize {
            let fill_count = self.position.y as usize - self.grid.len() + 1;
            self.grid.append(&mut vec![vec![]; fill_count]);
        }

        if self.position.x < 0 {
            while self.position.x < 0 {
                for elt in &mut self.grid {
                    elt.insert(0, self.default_char);
                }
                self.position.x += 1;
            }
            self.grid[self.position.y as usize][self.position.x as usize] = self.draw_char;
        }

        let row = &mut self.grid[self.position.y as usize];
        if row.len() <= self.position.x as usize {
            let fill_count = self.position.x as usize - row.len();
            row.append(&mut vec![self.default_char; fill_count]);
            row.push(self.draw_char);
        } else {
            row[self.position.x as usize] = self.draw_char;
        }
    }
}
