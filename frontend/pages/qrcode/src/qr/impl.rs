use super::*;

/// Implementation of `QrErrorLevel`.
impl QrErrorLevel {
    /// Returns the index of the level inside the tables of the encoder.
    ///
    /// # Returns
    ///
    /// - `usize` - The index of the level.
    pub fn index(&self) -> usize {
        match self {
            QrErrorLevel::Low => 0,
            QrErrorLevel::Medium => 1,
            QrErrorLevel::Quartile => 2,
            QrErrorLevel::High => 3,
        }
    }

    /// Returns the two bits of the level the format information carries.
    ///
    /// # Returns
    ///
    /// - `u32` - The bits of the level.
    pub fn format_bits(&self) -> u32 {
        match self {
            QrErrorLevel::Low => 1,
            QrErrorLevel::Medium => 0,
            QrErrorLevel::Quartile => 3,
            QrErrorLevel::High => 2,
        }
    }
}

impl QrBuilder {
    /// Creates the builder of one matrix.
    ///
    /// # Arguments
    ///
    /// - `usize` - The version of the code.
    /// - `QrErrorLevel` - The error correction level of the code.
    ///
    /// # Returns
    ///
    /// - `QrBuilder` - The builder of the matrix.
    pub(crate) fn new(version: usize, level: QrErrorLevel) -> QrBuilder {
        let size: usize = version * 4 + 17;
        QrBuilder {
            size,
            version,
            level,
            modules: vec![false; size * size],
            function: vec![false; size * size],
        }
    }

    /// Reads one module of the matrix.
    ///
    /// # Arguments
    ///
    /// - `usize` - The column of the module.
    /// - `usize` - The row of the module.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the module is dark.
    pub(crate) fn get(&self, x: usize, y: usize) -> bool {
        self.get_modules()[y * self.get_size() + x]
    }

    /// Writes one module of the matrix.
    ///
    /// # Arguments
    ///
    /// - `usize` - The column of the module.
    /// - `usize` - The row of the module.
    /// - `bool` - Whether the module is dark.
    ///
    /// # Returns
    ///
    /// - `()` - The module has been written.
    pub(crate) fn set(&mut self, x: usize, y: usize, dark: bool) {
        let index: usize = y * self.get_size() + x;
        self.get_mut_modules()[index] = dark;
    }

    /// Writes one module of the matrix that carries a function pattern.
    ///
    /// # Arguments
    ///
    /// - `usize` - The column of the module.
    /// - `usize` - The row of the module.
    /// - `bool` - Whether the module is dark.
    ///
    /// # Returns
    ///
    /// - `()` - The module has been written.
    pub(crate) fn mark_function(&mut self, x: usize, y: usize, dark: bool) {
        let index: usize = y * self.get_size() + x;
        self.get_mut_function()[index] = true;
        self.set(x, y, dark);
    }

    /// Reports whether one module of the matrix carries a function pattern.
    ///
    /// # Arguments
    ///
    /// - `usize` - The column of the module.
    /// - `usize` - The row of the module.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the module carries a function pattern.
    pub(crate) fn is_function(&self, x: usize, y: usize) -> bool {
        self.get_function()[y * self.get_size() + x]
    }

    /// Draws the function patterns of the code.
    ///
    /// # Returns
    ///
    /// - `()` - The patterns have been drawn.
    pub(crate) fn draw_function_patterns(&mut self) {
        for index in 0..self.get_size() {
            let dark: bool = index % 2 == 0;
            self.mark_function(6, index, dark);
            self.mark_function(index, 6, dark);
        }
        self.draw_finder(3, 3);
        self.draw_finder(self.get_size() as i32 - 4, 3);
        self.draw_finder(3, self.get_size() as i32 - 4);
        let positions: Vec<usize> = alignment_positions(self.get_version());
        let count: usize = positions.len();
        for row in 0..count {
            for column in 0..count {
                let corner: bool = (row == 0 && column == 0)
                    || (row == 0 && column == count - 1)
                    || (row == count - 1 && column == 0);
                if !corner {
                    self.draw_alignment(positions[row], positions[column]);
                }
            }
        }
        self.draw_format(0);
        self.draw_version();
    }

    /// Draws one finder pattern and its separator.
    ///
    /// # Arguments
    ///
    /// - `i32` - The column of the centre of the pattern.
    /// - `i32` - The row of the centre of the pattern.
    ///
    /// # Returns
    ///
    /// - `()` - The pattern has been drawn.
    pub(crate) fn draw_finder(&mut self, x: i32, y: i32) {
        for dy in -FINDER_RADIUS..=FINDER_RADIUS {
            for dx in -FINDER_RADIUS..=FINDER_RADIUS {
                let distance: i32 = dx.abs().max(dy.abs());
                let column: i32 = x + dx;
                let row: i32 = y + dy;
                if column >= 0
                    && column < self.get_size() as i32
                    && row >= 0
                    && row < self.get_size() as i32
                {
                    let dark: bool = distance != 2 && distance != 4;
                    self.mark_function(column as usize, row as usize, dark);
                }
            }
        }
    }

    /// Draws one alignment pattern.
    ///
    /// # Arguments
    ///
    /// - `usize` - The column of the centre of the pattern.
    /// - `usize` - The row of the centre of the pattern.
    ///
    /// # Returns
    ///
    /// - `()` - The pattern has been drawn.
    pub(crate) fn draw_alignment(&mut self, x: usize, y: usize) {
        for dy in -2i32..=2 {
            for dx in -2i32..=2 {
                let dark: bool = dx.abs().max(dy.abs()) != 1;
                self.mark_function(
                    (x as i32 + dx) as usize,
                    (y as i32 + dy) as usize,
                    dark,
                );
            }
        }
    }

    /// Draws the format information of the code.
    ///
    /// # Arguments
    ///
    /// - `usize` - The mask of the code.
    ///
    /// # Returns
    ///
    /// - `()` - The information has been drawn.
    pub(crate) fn draw_format(&mut self, mask: usize) {
        let data: u32 = self.get_level().format_bits() << 3 | mask as u32;
        let mut remainder: u32 = data;
        for _ in 0..10 {
            remainder = (remainder << 1) ^ ((remainder >> 9) * 0x537);
        }
        let bits: u32 = (data << 10 | remainder) ^ 0x5412;
        for index in 0..=5 {
            self.mark_function(8, index, get_bit(bits, index));
        }
        self.mark_function(8, 7, get_bit(bits, 6));
        self.mark_function(8, 8, get_bit(bits, 7));
        self.mark_function(7, 8, get_bit(bits, 8));
        for index in 9..15 {
            self.mark_function(14 - index, 8, get_bit(bits, index));
        }
        for index in 0..8 {
            self.mark_function(self.get_size() - 1 - index, 8, get_bit(bits, index));
        }
        for index in 8..15 {
            self.mark_function(8, self.get_size() - 15 + index, get_bit(bits, index));
        }
        self.mark_function(8, self.get_size() - 8, true);
    }

    /// Draws the version information of the code.
    ///
    /// # Returns
    ///
    /// - `()` - The information has been drawn.
    pub(crate) fn draw_version(&mut self) {
        if self.get_version() < 7 {
            return;
        }
        let mut remainder: u32 = self.get_version() as u32;
        for _ in 0..12 {
            remainder = (remainder << 1) ^ ((remainder >> 11) * 0x1F25);
        }
        let bits: u32 = (self.get_version() as u32) << 12 | remainder;
        for index in 0..18 {
            let dark: bool = get_bit(bits, index);
            let first: usize = self.get_size() - 11 + index % 3;
            let second: usize = index / 3;
            self.mark_function(first, second, dark);
            self.mark_function(second, first, dark);
        }
    }

    /// Writes the codewords of the code into the matrix.
    ///
    /// The codewords are written in the zigzag of the matrix, so the reader
    /// of the code finds them in the order they were written in.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The codewords of the code.
    ///
    /// # Returns
    ///
    /// - `()` - The codewords have been written.
    pub(crate) fn draw_codewords(&mut self, codewords: &[u8]) {
        let mut index: usize = 0;
        let mut right: i32 = self.get_size() as i32 - 1;
        while right >= 1 {
            if right == 6 {
                right = 5;
            }
            for vertical in 0..self.get_size() as i32 {
                for offset in 0..2 {
                    let column: i32 = right - offset;
                    let upward: bool = ((right + 1) & 2) == 0;
                    let row: i32 = if upward {
                        self.get_size() as i32 - 1 - vertical
                    } else {
                        vertical
                    };
                    let x: usize = column as usize;
                    let y: usize = row as usize;
                    if !self.is_function(x, y) && index < codewords.len() * 8 {
                        let dark: bool = get_bit(codewords[index >> 3] as u32, 7 - (index & 7));
                        self.set(x, y, dark);
                        index += 1;
                    }
                }
            }
            right -= 2;
        }
    }

    /// Applies one mask to the data modules of the matrix.
    ///
    /// # Arguments
    ///
    /// - `usize` - The mask of the code.
    ///
    /// # Returns
    ///
    /// - `()` - The mask has been applied.
    pub(crate) fn apply_mask(&mut self, mask: usize) {
        for y in 0..self.get_size() {
            for x in 0..self.get_size() {
                let invert: bool = match mask {
                    0 => (x + y) % 2 == 0,
                    1 => y % 2 == 0,
                    2 => x % 3 == 0,
                    3 => (x + y) % 3 == 0,
                    4 => (x / 3 + y / 2) % 2 == 0,
                    5 => (x * y) % 2 + (x * y) % 3 == 0,
                    6 => ((x * y) % 2 + (x * y) % 3) % 2 == 0,
                    _ => ((x + y) % 2 + (x * y) % 3) % 2 == 0,
                };
                if !self.is_function(x, y) && invert {
                    let dark: bool = !self.get(x, y);
                    self.set(x, y, dark);
                }
            }
        }
    }

    /// Scores the matrix of the code.
    ///
    /// The lower the score, the easier the code is to read.
    ///
    /// # Returns
    ///
    /// - `u32` - The score of the matrix.
    pub(crate) fn penalty(&self) -> u32 {
        let mut score: u32 = 0;
        for y in 0..self.get_size() {
            score += self.line_penalty(y, true);
        }
        for x in 0..self.get_size() {
            score += self.line_penalty(x, false);
        }
        for y in 0..self.get_size().saturating_sub(1) {
            for x in 0..self.get_size().saturating_sub(1) {
                let dark: bool = self.get(x, y);
                if dark == self.get(x + 1, y)
                    && dark == self.get(x, y + 1)
                    && dark == self.get(x + 1, y + 1)
                {
                    score += PENALTY_N2;
                }
            }
        }
        score += self.finder_penalty();
        let total: u32 = (self.get_size() * self.get_size()) as u32;
        let dark: u32 = self.get_modules().iter().filter(|module: &&bool| **module).count() as u32;
        let step: u32 = total / 20;
        let deviation: u32 = (2 * dark).abs_diff(total);
        let steps: u32 = if deviation <= step {
            0
        } else {
            (deviation - step + step - 1) / step
        };
        score += steps * PENALTY_N4;
        score
    }

    /// Scores one row or one column of the matrix.
    ///
    /// # Arguments
    ///
    /// - `usize` - The index of the row or of the column.
    /// - `bool` - Whether the line is a row.
    ///
    /// # Returns
    ///
    /// - `u32` - The score of the line.
    pub(crate) fn line_penalty(&self, index: usize, row: bool) -> u32 {
        let mut score: u32 = 0;
        let mut run: u32 = 0;
        let mut dark: bool = self.at(0, index, row);
        for step in 0..self.get_size() {
            let current: bool = self.at(step, index, row);
            if current == dark {
                run += 1;
                if run == 5 {
                    score += PENALTY_N1;
                } else if run > 5 {
                    score += 1;
                }
            } else {
                dark = current;
                run = 1;
            }
        }
        score
    }

    /// Scores the finder like patterns of the matrix.
    ///
    /// # Returns
    ///
    /// - `u32` - The score of the patterns.
    pub(crate) fn finder_penalty(&self) -> u32 {
        let mut score: u32 = 0;
        for y in 0..self.get_size() {
            for x in 0..self.get_size() {
                if x + 7 <= self.get_size() && self.is_finder_row(y, x) {
                    score += PENALTY_N3;
                }
                if y + 7 <= self.get_size() && self.is_finder_column(x, y) {
                    score += PENALTY_N3;
                }
            }
        }
        score
    }

    /// Reports whether one row holds a finder like pattern.
    ///
    /// # Arguments
    ///
    /// - `usize` - The row of the pattern.
    /// - `usize` - The column the pattern starts at.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the row holds the pattern.
    pub(crate) fn is_finder_row(&self, y: usize, x: usize) -> bool {
        let pattern: [bool; 7] = [true, false, true, true, true, false, true];
        for (offset, expected) in pattern.iter().enumerate() {
            if self.get(x + offset, y) != *expected {
                return false;
            }
        }
        let before: bool = x >= 4 && (0..4).all(|offset: usize| !self.get(x - 4 + offset, y));
        let after: bool = x + 11 <= self.get_size()
            && (0..4).all(|offset: usize| !self.get(x + 7 + offset, y));
        before || after
    }

    /// Reports whether one column holds a finder like pattern.
    ///
    /// # Arguments
    ///
    /// - `usize` - The column of the pattern.
    /// - `usize` - The row the pattern starts at.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the column holds the pattern.
    pub(crate) fn is_finder_column(&self, x: usize, y: usize) -> bool {
        let pattern: [bool; 7] = [true, false, true, true, true, false, true];
        for (offset, expected) in pattern.iter().enumerate() {
            if self.get(x, y + offset) != *expected {
                return false;
            }
        }
        let before: bool = y >= 4 && (0..4).all(|offset: usize| !self.get(x, y - 4 + offset));
        let after: bool = y + 11 <= self.get_size()
            && (0..4).all(|offset: usize| !self.get(x, y + 7 + offset));
        before || after
    }

    /// Reads one module of a row or of a column of the matrix.
    ///
    /// # Arguments
    ///
    /// - `usize` - The position inside the line.
    /// - `usize` - The index of the row or of the column.
    /// - `bool` - Whether the line is a row.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the module is dark.
    pub(crate) fn at(&self, position: usize, index: usize, row: bool) -> bool {
        if row {
            return self.get(position, index);
        }
        self.get(index, position)
    }

    /// Turns the builder into the code it holds.
    ///
    /// # Returns
    ///
    /// - `QrCode` - The code.
    pub(crate) fn into_code(self) -> QrCode {
        QrCode {
            size: self.get_size(),
            version: self.get_version(),
            modules: self.get_modules().clone(),
        }
    }
}
