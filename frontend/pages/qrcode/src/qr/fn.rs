use super::*;

/// Encodes a text into a QR code.
///
/// The text is encoded in the byte mode, so every character of the text
/// takes the number of bytes its UTF-8 form needs.
///
/// # Arguments
///
/// - `&str` - The text to encode.
/// - `QrErrorLevel` - The error correction level of the code.
///
/// # Returns
///
/// - `Option<QrCode>` - The code, absent when the text is too long.
pub fn encode(text: &str, level: QrErrorLevel) -> Option<QrCode> {
    encode_bytes(text.as_bytes(), level)
}

/// Encodes bytes into a QR code.
///
/// # Arguments
///
/// - `&[u8]` - The bytes to encode.
/// - `QrErrorLevel` - The error correction level of the code.
///
/// # Returns
///
/// - `Option<QrCode>` - The code, absent when the bytes are too many.
pub fn encode_bytes(data: &[u8], level: QrErrorLevel) -> Option<QrCode> {
    let version: usize = (1..=MAX_VERSION).find(|candidate: &usize| {
        let capacity: usize = data_codeword_count(*candidate, level) * 8;
        MODE_BITS + count_bits(*candidate) + data.len() * 8 <= capacity
    })?;
    let codewords: Vec<u8> = build_codewords(data, version, level);
    let mut best: Option<(u32, usize)> = None;
    for mask in 0..8 {
        let builder: QrBuilder = build(version, level, &codewords, mask);
        let score: u32 = builder.penalty();
        match best {
            Some((known, _)) if known <= score => {}
            _ => best = Some((score, mask)),
        }
    }
    let (_, mask): (u32, usize) = best?;
    let builder: QrBuilder = build(version, level, &codewords, mask);
    Some(builder.into_code())
}

/// Builds the matrix of one QR code.
///
/// # Arguments
///
/// - `usize` - The version of the code.
/// - `QrErrorLevel` - The error correction level of the code.
/// - `&[u8]` - The codewords of the code.
/// - `usize` - The mask of the code.
///
/// # Returns
///
/// - `QrBuilder` - The builder of the matrix.
fn build(version: usize, level: QrErrorLevel, codewords: &[u8], mask: usize) -> QrBuilder {
    let mut builder: QrBuilder = QrBuilder::new(version, level);
    builder.draw_function_patterns();
    builder.draw_codewords(codewords);
    builder.apply_mask(mask);
    builder.draw_format(mask);
    builder
}

/// Builds the codewords of one QR code.
///
/// The codewords hold the text, the terminator and the padding of the
/// version, followed by the error correction codewords of every block.
///
/// # Arguments
///
/// - `&[u8]` - The bytes to encode.
/// - `usize` - The version of the code.
/// - `QrErrorLevel` - The error correction level of the code.
///
/// # Returns
///
/// - `Vec<u8>` - The codewords of the code.
fn build_codewords(data: &[u8], version: usize, level: QrErrorLevel) -> Vec<u8> {
    let data_count: usize = data_codeword_count(version, level);
    let capacity: usize = data_count * 8;
    let mut bits: Vec<bool> = Vec::with_capacity(capacity);
    push_number(&mut bits, MODE_BYTE, MODE_BITS);
    push_number(&mut bits, data.len() as u32, count_bits(version));
    for byte in data {
        push_number(&mut bits, *byte as u32, 8);
    }
    let terminator: usize = min(MODE_BITS, capacity.saturating_sub(bits.len()));
    push_number(&mut bits, 0, terminator);
    while bits.len() % 8 != 0 {
        bits.push(false);
    }
    let mut padding: bool = true;
    while bits.len() < capacity {
        let byte: u8 = if padding { PAD_FIRST } else { PAD_SECOND };
        push_number(&mut bits, byte as u32, 8);
        padding = !padding;
    }
    let mut data_bytes: Vec<u8> = Vec::with_capacity(data_count);
    for chunk in bits.chunks(8) {
        let mut byte: u8 = 0;
        for bit in chunk {
            byte = (byte << 1) | u8::from(*bit);
        }
        data_bytes.push(byte);
    }
    let raw_codewords: usize = num_raw_data_modules(version) / 8;
    let block_ecc_len: usize = ECC_CODEWORDS_PER_BLOCK[level.index()][version] as usize;
    let num_blocks: usize = NUM_ERROR_CORRECTION_BLOCKS[level.index()][version] as usize;
    let short_block_len: usize = raw_codewords / num_blocks;
    let num_short_blocks: usize = num_blocks - raw_codewords % num_blocks;
    let divisor: Vec<u8> = reed_solomon_divisor(block_ecc_len);
    let mut blocks: Vec<Vec<u8>> = Vec::with_capacity(num_blocks);
    let mut offset: usize = 0;
    for index in 0..num_blocks {
        let data_len: usize =
            short_block_len - block_ecc_len + usize::from(index >= num_short_blocks);
        let block_data: &[u8] = &data_bytes[offset..offset + data_len];
        offset += data_len;
        let ecc: Vec<u8> = reed_solomon_remainder(block_data, &divisor);
        let mut block: Vec<u8> = block_data.to_vec();
        if index < num_short_blocks {
            block.push(0);
        }
        block.extend_from_slice(&ecc);
        blocks.push(block);
    }
    let mut result: Vec<u8> = Vec::with_capacity(raw_codewords);
    for index in 0..short_block_len + 1 {
        for (block_index, block) in blocks.iter().enumerate() {
            if index != short_block_len - block_ecc_len || block_index >= num_short_blocks {
                result.push(block[index]);
            }
        }
    }
    result
}

/// Returns the number of modules that hold data or error correction code.
///
/// # Arguments
///
/// - `usize` - The version of the code.
///
/// # Returns
///
/// - `usize` - The number of the modules.
fn num_raw_data_modules(version: usize) -> usize {
    let mut result: usize = (16 * version + 128) * version + 64;
    if version >= 2 {
        let num_align: usize = version / 7 + 2;
        result -= (25 * num_align - 10) * num_align - 55;
        if version >= 7 {
            result -= 36;
        }
    }
    result
}

/// Returns the number of codewords that hold the data of a code.
///
/// # Arguments
///
/// - `usize` - The version of the code.
/// - `QrErrorLevel` - The error correction level of the code.
///
/// # Returns
///
/// - `usize` - The number of the codewords.
fn data_codeword_count(version: usize, level: QrErrorLevel) -> usize {
    let raw: usize = num_raw_data_modules(version) / 8;
    let ecc: usize = ECC_CODEWORDS_PER_BLOCK[level.index()][version] as usize;
    let blocks: usize = NUM_ERROR_CORRECTION_BLOCKS[level.index()][version] as usize;
    raw - ecc * blocks
}

/// Returns the number of bits the character count of a version needs.
///
/// # Arguments
///
/// - `usize` - The version of the code.
///
/// # Returns
///
/// - `usize` - The number of the bits.
fn count_bits(version: usize) -> usize {
    if version <= 9 {
        return COUNT_BITS_SHORT;
    }
    COUNT_BITS_LONG
}

/// Returns the centres of the alignment patterns of a version.
///
/// # Arguments
///
/// - `usize` - The version of the code.
///
/// # Returns
///
/// - `Vec<usize>` - The centres of the alignment patterns.
pub(crate) fn alignment_positions(version: usize) -> Vec<usize> {
    if version == 1 {
        return Vec::new();
    }
    let num_align: usize = version / 7 + 2;
    let step: usize = if version == 32 {
        26
    } else {
        (version * 4 + num_align * 2 + 1) / (num_align * 2 - 2) * 2
    };
    let mut result: Vec<usize> = vec![0; num_align];
    result[0] = 6;
    let mut position: i64 = version as i64 * 4 + 10;
    for index in (1..num_align).rev() {
        result[index] = position as usize;
        position -= step as i64;
    }
    result
}

/// Builds the generator polynomial of one error correction block.
///
/// # Arguments
///
/// - `usize` - The number of the error correction codewords.
///
/// # Returns
///
/// - `Vec<u8>` - The generator polynomial.
fn reed_solomon_divisor(degree: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; degree];
    result[degree - 1] = 1;
    let mut root: u8 = 1;
    for _ in 0..degree {
        for index in 0..degree {
            result[index] = gf_multiply(result[index], root);
            if index + 1 < degree {
                result[index] ^= result[index + 1];
            }
        }
        root = gf_multiply(root, 2);
    }
    result
}

/// Builds the error correction codewords of one block.
///
/// # Arguments
///
/// - `&[u8]` - The data codewords of the block.
/// - `&[u8]` - The generator polynomial of the block.
///
/// # Returns
///
/// - `Vec<u8>` - The error correction codewords of the block.
fn reed_solomon_remainder(data: &[u8], divisor: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; divisor.len()];
    for byte in data {
        let factor: u8 = byte ^ result[0];
        result.rotate_left(1);
        result[divisor.len() - 1] = 0;
        for index in 0..divisor.len() {
            result[index] ^= gf_multiply(divisor[index], factor);
        }
    }
    result
}

/// Multiplies two numbers of the Galois field of the code.
///
/// # Arguments
///
/// - `u8` - The first number.
/// - `u8` - The second number.
///
/// # Returns
///
/// - `u8` - The product of the numbers.
fn gf_multiply(x: u8, y: u8) -> u8 {
    let mut product: u8 = 0;
    for index in (0..8).rev() {
        let reduction: u8 = if product & 0x80 == 0 { 0 } else { 0x1D };
        product = (product << 1) ^ reduction;
        if (y >> index) & 1 == 1 {
            product ^= x;
        }
    }
    product
}

/// Writes a number into a stream of bits.
///
/// # Arguments
///
/// - `&mut Vec<bool>` - The stream of the bits.
/// - `u32` - The number to write.
/// - `usize` - The number of the bits of the number.
///
/// # Returns
///
/// - `()` - The bits have been written.
fn push_number(bits: &mut Vec<bool>, value: u32, count: usize) {
    for index in (0..count).rev() {
        bits.push((value >> index) & 1 == 1);
    }
}

/// Reads one bit of a number.
///
/// # Arguments
///
/// - `u32` - The number.
/// - `usize` - The index of the bit.
///
/// # Returns
///
/// - `bool` - Whether the bit is set.
pub(crate) fn get_bit(value: u32, index: usize) -> bool {
    (value >> index) & 1 == 1
}

