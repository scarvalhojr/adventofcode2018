use regex::Regex;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

pub struct Point {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Point {
    pub fn move_forward(&mut self) {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;
    }

    pub fn move_backward(&mut self) {
        self.x_pos -= self.x_vel;
        self.y_pos -= self.y_vel;
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(
            r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>"
        ).unwrap();

        let groups = pattern
            .captures(s)
            .ok_or(Error::new(ErrorKind::InvalidData, "Invalid format"))?;

        let numbers: Vec<i32> = groups
            .iter()
            .skip(1)
            .map(|val| {
                val.unwrap()
                    .as_str()
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))
            })
            .collect::<Result<_, _>>()?;

        Ok(Point {
            x_pos: numbers[0],
            y_pos: numbers[1],
            x_vel: numbers[2],
            y_vel: numbers[3],
        })
    }
}

struct Canvas {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Canvas {
    fn get_dimensions(&self) -> (usize, usize) {
        (
            (1 + self.max_x - self.min_x) as usize,
            (1 + self.max_y - self.min_y) as usize,
        )
    }

    fn get_size(&self) -> u64 {
        let (dim_x, dim_y) = self.get_dimensions();
        dim_x as u64 * dim_y as u64
    }
}

pub struct Image {
    points: Vec<Point>,
}

impl Image {
    pub fn new(points: Vec<Point>) -> Self {
        Image { points }
    }

    fn move_forward(&mut self) {
        for point in self.points.iter_mut() {
            point.move_forward();
        }
    }

    fn move_backward(&mut self) {
        for point in self.points.iter_mut() {
            point.move_backward();
        }
    }

    fn get_canvas(&self) -> Canvas {
        Canvas {
            min_x: self.points.iter().map(|p| p.x_pos).min().unwrap(),
            max_x: self.points.iter().map(|p| p.x_pos).max().unwrap(),
            min_y: self.points.iter().map(|p| p.y_pos).min().unwrap(),
            max_y: self.points.iter().map(|p| p.y_pos).max().unwrap(),
        }
    }

    fn get_size(&self) -> u64 {
        self.get_canvas().get_size()
    }

    pub fn plot(&self) -> Vec<String> {
        let canvas = self.get_canvas();
        let (dim_x, dim_y) = canvas.get_dimensions();
        let mut lines: Vec<Vec<char>> = vec![vec!['.'; dim_x]; dim_y];
        for point in self.points.iter() {
            let x = (point.y_pos - canvas.min_y) as usize;
            let y = (point.x_pos - canvas.min_x) as usize;
            lines[x][y] = '#';
        }
        lines
            .into_iter()
            .map(|line| line.into_iter().collect())
            .collect()
    }
}

pub fn part1_and_2(image: &mut Image) -> usize {
    let mut prev_size = image.get_size();
    let mut size;
    let mut time = 0;
    loop {
        image.move_forward();
        size = image.get_size();
        if size > prev_size {
            image.move_backward();
            return time;
        }
        prev_size = size;
        time += 1;
    }
}
