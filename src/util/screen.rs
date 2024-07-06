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
                    if place && (self.screen[i][self.wid-2] == 0 || self.screen[i][self.wid-2] == 5) {
                        self.screen[i][self.wid-1] = 2;
                    }
                    else {
                        self.screen[i][self.wid-1] = 0;
                    }
                    if self.screen[i][self.wid-2] == 1 {
                        self.screen[i][self.wid-1] = 4;
                        place = !place;
                    }
                }
            }
            else {
                for i in 1..self.hei-1 {
                    if self.screen[i][self.wid-2] == 4 || self.screen[i][self.wid-2] == 1 {
                        self.screen[i][self.wid-1] = 1;
                    }
                    else if self.screen[i][self.wid-2] == 2 || self.screen[i][self.wid-2] == 5{
                        self.screen[i][self.wid-1] = 5;
                    }
                    else {
                        self.screen[i][self.wid-1] = 0;
                    }
                }
            }
        }
        else {
            if self.counter >= self.blankwid {
                self.counter = 0;
                self.midp = true;

                let mut rng = rand::thread_rng();
                let blank = 8;
                let randr: usize = rng.gen_range(1..self.hei-1-blank);

                for i in 1..self.hei-1 {
                    if i > randr && i < randr+blank {
                        self.screen[i][self.wid-1] = 0;
                    }
                    else if i == randr || i == randr+blank {
                        self.screen[i][self.wid-1] = 4;
                    }
                    else {
                        self.screen[i][self.wid-1] = 2;
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

    fn check(&mut self) {
        if self.py == 0 || self.py == self.hei-1 {
            self.ended = true;
            return
        }

        self.ended = self.screen[self.hei-1-self.py][self.px] != 0
    }

    pub fn print(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for (i, row) in self.screen.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                {
                    if i == self.hei-1-self.py && j == self.px {
                        if self.ended {
                            stdout
                                .queue(cursor::MoveTo(j as u16, i as u16))?
                                .queue(style::Print("%"))?;
                        }
                        else {
                            stdout
                                .queue(cursor::MoveTo(j as u16, i as u16))?
                                .queue(style::Print("*"))?;
                        }
                        continue;
                    }
                }
                match col {
                    1 => {
                        stdout
                            .queue(cursor::MoveTo(j as u16, i as u16))?
                            .queue(style::Print("-"))?;
                    }
                    2 => {
                        stdout
                            .queue(cursor::MoveTo(j as u16, i as u16))?
                            .queue(style::Print("|"))?;
                    }
                    3 => {
                        stdout
                            .queue(cursor::MoveTo(j as u16, i as u16))?
                            .queue(style::Print("*"))?;
                    }
                    4 => {
                        stdout
                            .queue(cursor::MoveTo(j as u16, i as u16))?
                            .queue(style::Print("+"))?;
                    }
                    5 => {
                        stdout
                            .queue(cursor::MoveTo(j as u16, i as u16))?
                            .queue(style::Print("X"))?;
                    }
                    _ => {}
                }
            }
        }

        if self.ended {
            stdout
                .queue(cursor::MoveTo(0, 0))?
                .queue(style::Print("!!!  PRESS 'R' TO RESTART  !!!"))?;
            stdout
                .queue(cursor::MoveTo(0, self.hei as u16 - 1))?
                .queue(style::Print("!!!   PRESS 'Q' TO QUIT   !!!"))?;
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn mvt(&mut self) {
        self.v = 2.7;
    }
}
