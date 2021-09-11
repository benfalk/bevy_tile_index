#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

pub struct SurroundingCoords {
    min_x: usize,
    max_x: usize,
    max_y: usize,
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn surrounding_coords(&self, distance: usize, ceiling: usize) -> SurroundingCoords {
        let min_x = if self.x < distance { 0 } else { self.x - distance };
        let min_y = if self.y < distance { 0 } else { self.y - distance };
        let max_x = (self.x + distance).clamp(min_x, ceiling);
        let max_y = (self.y + distance).clamp(min_y, ceiling) + 1;

        SurroundingCoords {
            x: min_x,
            y: min_y,
            min_x,
            max_x,
            max_y,
        }
    }
}

impl Iterator for SurroundingCoords {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.max_y { return None; }

        let coord = Coord { x: self.x, y: self.y };

        if self.x < self.max_x {
            self.x += 1;
            Some(coord)
        }
        else{
            self.x = self.min_x;
            self.y += 1;
            Some(coord)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn top_left_corner() {
        let coord = Coord::new(0, 0);
        let items: Vec<Coord> = coord.surrounding_coords(1, 20).collect();

        assert_eq!(
            items,
            vec![
                Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 },
                Coord { x: 0, y: 1 }, Coord { x: 1, y: 1 },
            ]
        );
    }

    #[test]
    fn top_right_corner() {
        let coord = Coord::new(19, 0);
        let items: Vec<Coord> = coord.surrounding_coords(1, 19).collect();

        assert_eq!(
            items,
            vec![
                Coord { x: 18, y: 0 }, Coord { x: 19, y: 0 },
                Coord { x: 18, y: 1 }, Coord { x: 19, y: 1 },
            ]
        );
    }

    #[test]
    fn bottom_left_corner() {
        let coord = Coord::new(0, 19);
        let items: Vec<Coord> = coord.surrounding_coords(1, 19).collect();

        assert_eq!(
            items,
            vec![
                Coord { x: 0, y: 18 }, Coord { x: 1, y: 18 },
                Coord { x: 0, y: 19 }, Coord { x: 1, y: 19 },
            ]
        );
    }

    #[test]
    fn bottom_right_corner() {
        let coord = Coord::new(19, 19);
        let items: Vec<Coord> = coord.surrounding_coords(1, 19).collect();

        assert_eq!(
            items,
            vec![
                Coord { x: 18, y: 18 }, Coord { x: 19, y: 18 },
                Coord { x: 18, y: 19 }, Coord { x: 19, y: 19 },
            ]
        );
    }
    
    #[test]
    fn not_near_an_edge() {
        let coord = Coord::new(4,4);
        let items: Vec<Coord> = coord.surrounding_coords(1, 19).collect();

        assert_eq!(
            items,
            vec![
                Coord { x: 3, y: 3 }, Coord { x: 4, y: 3 }, Coord { x: 5, y: 3 },
                Coord { x: 3, y: 4 }, Coord { x: 4, y: 4 }, Coord { x: 5, y: 4 },
                Coord { x: 3, y: 5 }, Coord { x: 4, y: 5 }, Coord { x: 5, y: 5 },
            ]
        );
    }
}
