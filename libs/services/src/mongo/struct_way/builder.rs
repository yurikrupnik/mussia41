struct House {
    windows: u8,
    doors: u8,
    garage: bool,
}

struct HouseBuilder {
    windows: u8,
    doors: u8,
    garage: bool,
}

impl HouseBuilder {
    fn new() -> Self {
        HouseBuilder {
            windows: 0,
            doors: 0,
            garage: false,
        }
    }

    fn windows(mut self, count: u8) -> Self {
        self.windows = count;
        self
    }

    fn doors(mut self, count: u8) -> Self {
        self.doors = count;
        self
    }

    fn garage(mut self, has_garage: bool) -> Self {
        self.garage = has_garage;
        self
    }

    fn build(self) -> House {
        House {
            windows: self.windows,
            doors: self.doors,
            garage: self.garage,
        }
    }
}
