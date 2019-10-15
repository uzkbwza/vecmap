#[cfg(test)]
mod tests {
    use crate::VecMap;

    #[test]
    fn retrieve_len() {
        let map = VecMap::filled_with(0, 2, 2);
        assert_eq!(map.retrieve(1, 1), Ok(0));
    }
}

#[derive(Debug)]
pub struct VecMap<T> {
    pub items: Vec<T>,
    pub width: i32,
    pub height: i32,
    default: T
}

impl<T> VecMap<T> where T: Clone + Copy {
    pub fn filled_with(item: T, width: i32, height: i32) -> Self {
        if width * height == 0 {
            panic!("vecmap must have length > 0")
        }

        let items = vec![item; (width * height) as usize];
        let default = item;
        VecMap {
            items,
            width,
            height,
            default
        }
    }

    pub fn retrieve(&self, x: i32, y: i32) -> Result<T, String> {
        let id = self.xy_idx(x, y);
        if self.is_in_bounds(x, y) {
            return Ok(self.items[id])
        }
        Err(format!("No item at {}, {}; out of bounds", x, y))
    }

    pub fn set_point(&mut self, x: i32, y: i32, item: T) -> Result<(), String> {
        let id = self.xy_idx(x, y);
        if self.is_in_bounds(x, y) {
            self.items[id] = item;
            return Ok(());
        } else {
            Err(format!("Couldn't insert at {} {}; out of bounds", x, y))
        }
    }

    pub fn reset_point(&mut self, x: i32, y: i32) -> Result<(), String> {
        self.set_point(x, y, self.default)
    }

    pub fn reset_map(&mut self) {
        for i in 0..self.items.len() {
            let (x, y) = self.idx_xy(i);
            if self.is_in_bounds(x, y) {
                self.reset_point(x, y);
            }
        }
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x < self.width && y < self.height && x >= 0 && y >= 0
    }

    // stealing this from thebracket

    // We're storing all the tiles in one big array, so we need a way to map an X,Y coordinate to
    // a tile. Each row is stored sequentially (so 0..80, 81..160, etc.). This takes an x/y and returns
    // the array index.
    pub fn xy_idx(&self, x: i32, y: i32) -> usize{
        (y as usize * self.width as usize) + x as usize;
    }

    // It's a great idea to have a reverse mapping for these coordinates. This is as simple as
    // index % MAP_WIDTH (mod MAP_WIDTH), and index / MAP_WIDTH
    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.width, idx as i32 / self.width)
    }
}
