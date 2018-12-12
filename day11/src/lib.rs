pub struct FuelGrid {
    cells: Vec<Vec<i32>>,
    dim: usize,
}

impl FuelGrid {
    pub fn new(dim: usize, serial_num: usize) -> Self {
        FuelGrid {
            cells: (1..=dim)
                .map(|y| {
                    (1..=dim)
                        .map(|x| FuelGrid::calc_power_level(x, y, serial_num))
                        .collect()
                })
                .collect(),
            dim,
        }
    }

    fn calc_power_level(pos_x: usize, pos_y: usize, serial_num: usize) -> i32 {
        let rack_id = pos_x + 10;
        (((rack_id * (serial_num + pos_y * rack_id)) / 100) % 10) as i32 - 5
    }

    pub fn get_power_level(&self, pos_x: usize, pos_y: usize) -> i32 {
        // Grid positions start at 1 (internally, they are 0-based)
        if pos_x > 0 && pos_y > 0 {
            self.cells
                .get(pos_y - 1)
                .map_or(0, |row| *row.get(pos_x - 1).unwrap_or(&0))
        } else {
            0
        }
    }

    pub fn max_square_size(&self, size: usize) -> (usize, usize, i32) {
        if self.dim > size {
            (0..self.dim - size)
                .flat_map(|y| {
                    (0..self.dim - size)
                        // Grid positions start at 1
                        // (internally, they are 0-based)
                        .map(move |x| {
                            (x + 1, y + 1, self.square_power(x, y, size))
                        })
                })
                .max_by_key(|(_, _, total)| *total)
                .unwrap_or((0, 0, 0))
        } else {
            (0, 0, 0)
        }
    }

    fn square_power(&self, pos_x: usize, pos_y: usize, size: usize) -> i32 {
        // Positions here are 0-based
        self.cells[pos_y..pos_y + size]
            .iter()
            .map(|row| row[pos_x..pos_x + size].iter().sum::<i32>())
            .sum()
    }

    pub fn max_square(&self) -> (usize, usize, usize, i32) {
        // Assuming the grid has at least one value > 0
        let mut row_sum = vec![0; self.dim];
        let mut max_x = 0;
        let mut max_y = 0;
        let mut max_size = 0;
        let mut max_total = 0;
        for start_x in 0..self.dim {
            for val in row_sum.iter_mut() {
                *val = 0;
            }
            for end_x in start_x..self.dim {
                for (pos_y, val) in row_sum.iter_mut().enumerate() {
                    *val += self.cells[pos_y][end_x];
                }
                let size = end_x - start_x + 1;
                let (start_y, total) = FuelGrid::max_sub_array(&row_sum, size);
                if total > max_total {
                    max_total = total;
                    max_x = start_x;
                    max_y = start_y;
                    max_size = size;
                }
            }
        }
        // Grid positions start at 1 (internally, they are 0-based)
        (max_x + 1, max_y + 1, max_size, max_total)
    }

    fn max_sub_array(array: &[i32], size: usize) -> (usize, i32) {
        let mut max_total = array[0..size].iter().sum();
        let mut max_start = 0;
        let mut total = max_total;
        for start in 1..=array.len() - size {
            total += array[start + size - 1] - array[start - 1];
            if total > max_total {
                max_total = total;
                max_start = start;
            }
        }
        (max_start, max_total)
    }
}
