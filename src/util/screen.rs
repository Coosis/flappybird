use std::cmp::{max, min};
use std::io::{self, Write};
use rand::Rng;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self}
};


//0 is ' '
//1 is '-'
//2 is '|'
//3 is '*'
//4 is '+'
//5 is 'X'
pub struct Screen {
    wid: usize,
    hei: usize,
    screen: Vec<Vec<usize>>,
    px: usize,
    py: usize,
    v: f32,
    g: f32,
    blankwid: usize,
    pillerwid: usize,
    midp: bool,
    counter: usize,
    ended: bool,
}

impl Screen {
    pub fn new(wid: usize, hei: usize) -> Screen {
        let mut screen: Vec<Vec<usize>> = vec![vec![0; wid]; hei];
        for c in &mut screen[0] {
            *c = 1;
        }
        for c in &mut screen[(hei-1) as usize] {
            *c = 1;
        }
        Screen { 
            wid,
            hei,
            screen, 
            px: min(7, wid/4),
            py: hei/2,
            v: 0.0,
            g: 0.6,
            blankwid: 9,
            pillerwid: 5,
            midp: false,
            counter: 0,
            ended: false,
        }
    }

    //reset the screen to effectively restart the game
    pub fn reset(&mut self) {
        let mut screen: Vec<Vec<usize>> = vec![vec![0; self.wid]; self.hei];
        for c in &mut screen[0] {
            *c = 1;
        }
        for c in &mut screen[(self.hei-1) as usize] {
            *c = 1;
        }
        self.screen = screen;
        self.px = min(7, self.wid/4);
        self.py = self.hei/2;
        self.v = 0.0;
        self.midp = false;
        self.counter = 0;
        self.ended = false;
    }

    //update the screen(managing player, getting next frame)
    pub fn update(&mut self) {
        let _ = self.print();

        if self.ended {
            return;
        }

        if self.v >= 0.0 {
            self.py += self.v as usize;
            self.py = min(self.py, self.hei-1);
        }
        else {
            if self.py >= (-self.v) as usize {
                self.py -= (-self.v) as usize;
            }
            else { self.py = 0; }
        }

        self.v -= self.g;
        self.v = self.v.max(-5.0);
        self.next();
        self.check();
    }

    //getting next frame by constructing a new col, and moving all cols 1 to the left
    fn next(&mut self) {
        for i in 0..self.screen.len() {
            for j in 0..self.screen[i].len()-1 {
                self.screen[i][j] = self.screen[i][j+1];
            }
        }
        self.counter += 1;
        if self.midp {
            if self.counter >= self.pillerwid {
                let mut place = true;
                self.counter = 0;
                self.midp = false;
                for i in 1..self.hei-1 {
                    //the tile to the left of the current-to-be-decided tile
                    let left = self.screen[i][self.wid-2];
                    //mut ref of current tile
                    let cur = &mut self.screen[i][self.wid-1];

                    if place && (left == 0 || left == 5) {
                        *cur = 2;
                    }
                    else {
                        *cur = 0;
                    }
                    if left == 1 {
                        *cur = 4;
                        place = !place;
                    }
                }
            }
            else {
                for i in 1..self.hei-1 {
                    //the tile to the left of the current-to-be-decided tile
                    let left = self.screen[i][self.wid-2];
                    //mut ref of current tile
                    let cur = &mut self.screen[i][self.wid-1];

                    if left == 4 || left == 1 {
                        *cur = 1;
                    }
                    else if left == 2 || left == 5{
                        *cur = 5;
                    }
                    else {
                        *cur = 0;
                    }
                }
            }
        }
        else {
            if self.counter >= self.blankwid {
                self.counter = 0;
                self.midp = true;

                let mut rng = rand::thread_rng();
                //the opening's size for a single piller
                let blank = 8;
                let randr: usize = rng.gen_range(1..self.hei-1-blank);

                for i in 1..self.hei-1 {
                    //mut ref of current tile
                    let cur = &mut self.screen[i][self.wid-1];

                    if i > randr && i < randr+blank {
                        *cur = 0;
                    }
                    else if i == randr || i == randr+blank {
                        *cur = 4;
                    }
                    else {
                        *cur = 2;
                    }
                }
            }
            else {
                for i in 1..self.hei-1 {
                    self.screen[i][self.wid-1] = 0;
                }
            }
        }
    }

    //whether player hit a wall/bound
    fn check(&mut self) {
        if self.py == 0 || self.py == self.hei-1 {
            self.ended = true;
            return
        }

        self.ended = self.screen[self.hei-1-self.py][self.px] != 0
    }

    //printing to terminal
    pub fn print(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for (i, row) in self.screen.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                {
                    if i == self.hei-1-self.py && j == self.px {
                        if self.ended {
                            let _ = mvprint(j, i, "%");
                        }
                        else {
                            let _ = mvprint(j, i, "*");
                        }
                        continue;
                    }
                }
                match col {
                    1 => {
                        let _ = mvprint(j, i, "-");
                    }
                    2 => {
                        let _ = mvprint(j, i, "|");
                    }
                    3 => {
                        let _ = mvprint(j, i, "*");
                    }
                    4 => {
                        let _ = mvprint(j, i, "+");
                    }
                    5 => {
                        let _ = mvprint(j, i, "X");
                    }
                    _ => {}
                }
            }
        }

        if self.ended {
            let _ = mvprint(0, 0, "!!!  PRESS 'R' TO RESTART  !!!");
            let _ = mvprint(0, self.hei - 1, "!!!   PRESS 'Q' TO QUIT   !!!");
        }

        stdout.flush()?;
        Ok(())
    }


    pub fn mvt(&mut self) {
        self.v = 2.7;
    }
}

fn mvprint(x: usize, y: usize, content: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout
        .queue(cursor::MoveTo(x as u16, y as u16))?
        .queue(style::Print(content))?;
    Ok(())
}
