pub struct Neightbors<const DIAG: bool> {
    width: usize,
    height: usize,
    point: (i32, i32),
    current: u32,
}

pub type NeightborsDiag = Neightbors<true>;
pub type NeightborsNoDiag = Neightbors<false>;

impl<const DIAG: bool> Neightbors<DIAG> {
    pub fn new(width: usize, height: usize, (x, y): (usize, usize)) -> Self {
        Neightbors {
            width,
            height,
            point: (x as i32, y as i32),
            current: 0,
        }
    }
}

impl Iterator for Neightbors<false> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x1, y1) = self.point;
        let (x2, y2) = match self.current {
            0 => (x1 - 1, y1),
            1 => (x1, y1 + 1),
            2 => (x1 + 1, y1),
            3 => (x1, y1 - 1),
            _ => return None,
        };

        self.current += 1;

        if x2 < 0 || y2 < 0 || y2 >= self.height as i32 || x2 >= self.width as i32 {
            return self.next();
        }

        Some((x2 as usize, y2 as usize))
    }
}

impl Iterator for Neightbors<true> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x1, y1) = self.point;
        let (x2, y2) = match self.current {
            0 => (x1 - 1, y1),
            1 => (x1 - 1, y1 + 1),
            2 => (x1, y1 + 1),
            3 => (x1 + 1, y1 + 1),
            4 => (x1 + 1, y1),
            5 => (x1 + 1, y1 - 1),
            6 => (x1, y1 - 1),
            7 => (x1 - 1, y1 - 1),
            _ => return None,
        };

        self.current += 1;

        if x2 < 0 || y2 < 0 || y2 >= self.height as i32 || x2 >= self.width as i32 {
            return self.next();
        }

        Some((x2 as usize, y2 as usize))
    }
}

pub struct MapPoints {
    width: usize,
    height: usize,
    point: (usize, usize),
}

impl MapPoints {
    pub fn new(width: usize, height: usize) -> Self {
        MapPoints {
            width,
            height,
            point: (0, 0),
        }
    }
}

impl Iterator for MapPoints {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.point;
        if y == self.height {
            return None;
        }

        if x == self.width - 1 {
            self.point.0 = 0;
            self.point.1 += 1;
        } else {
            self.point.0 += 1;
        }

        return Some((x, y));
    }
}
