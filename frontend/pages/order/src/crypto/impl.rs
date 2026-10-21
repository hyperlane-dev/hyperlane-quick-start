use super::*;

/// Implementation of `BigUint`.
impl BigUint {
    /// Builds the number from its big endian bytes.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The big endian bytes of the number.
    ///
    /// # Returns
    ///
    /// - `BigUint` - The number the bytes hold.
    pub fn from_bytes_be(bytes: &[u8]) -> BigUint {
        let mut limbs: Vec<u32> = Vec::new();
        let mut end: usize = bytes.len();
        while end > 0 {
            let start: usize = end.saturating_sub(LIMB_BYTES);
            let mut limb: u32 = 0;
            for byte in &bytes[start..end] {
                limb = (limb << BITS_BYTE) | u32::from(*byte);
            }
            limbs.push(limb);
            end = start;
        }
        let mut value: BigUint = BigUint { limbs };
        value.trim();
        value
    }

    /// Builds the number zero.
    ///
    /// # Returns
    ///
    /// - `BigUint` - The number zero.
    pub fn zero() -> BigUint {
        BigUint { limbs: Vec::new() }
    }

    /// Builds the number one.
    ///
    /// # Returns
    ///
    /// - `BigUint` - The number one.
    pub fn one() -> BigUint {
        BigUint {
            limbs: vec![1],
        }
    }

    /// Reports whether the number holds zero.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the number holds zero.
    pub fn is_zero(&self) -> bool {
        self.get_limbs().is_empty()
    }

    /// Returns the number of bits the number holds.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of bits of the number.
    pub fn bit_len(&self) -> usize {
        match self.get_limbs().last() {
            None => 0,
            Some(top) => {
                (self.get_limbs().len() - 1) * LIMB_BITS as usize
                    + (LIMB_BITS - top.leading_zeros()) as usize
            }
        }
    }

    /// Reports whether the bit at the index is set.
    ///
    /// # Arguments
    ///
    /// - `usize` - The index of the bit.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the bit is set.
    pub fn bit(&self, index: usize) -> bool {
        match self.get_limbs().get(index / LIMB_BITS as usize) {
            None => false,
            Some(limb) => (limb >> (index % LIMB_BITS as usize)) & 1 == 1,
        }
    }

    /// Sets the bit at the index.
    ///
    /// # Arguments
    ///
    /// - `usize` - The index of the bit.
    pub fn set_bit(&mut self, index: usize) {
        let limb: usize = index / LIMB_BITS as usize;
        while self.get_limbs().len() <= limb {
            self.get_mut_limbs().push(0);
        }
        self.get_mut_limbs()[limb] |= 1 << (index % LIMB_BITS as usize);
    }

    /// Compares the number with another one.
    ///
    /// # Arguments
    ///
    /// - `&BigUint` - The number to compare with.
    ///
    /// # Returns
    ///
    /// - `Ordering` - The order of the two numbers.
    pub fn compare(&self, other: &BigUint) -> Ordering {
        if self.get_limbs().len() != other.get_limbs().len() {
            return self.get_limbs().len().cmp(&other.get_limbs().len());
        }
        for index in (0..self.get_limbs().len()).rev() {
            if self.get_limbs()[index] != other.get_limbs()[index] {
                return self.get_limbs()[index].cmp(&other.get_limbs()[index]);
            }
        }
        Ordering::Equal
    }

    /// Subtracts another number from the number.
    ///
    /// The caller has to make sure the number is not smaller than the other
    /// one, the subtraction of a larger number is not defined.
    ///
    /// # Arguments
    ///
    /// - `&BigUint` - The number to subtract.
    ///
    /// # Returns
    ///
    /// - `BigUint` - The difference of the two numbers.
    pub fn subtract(&self, other: &BigUint) -> BigUint {
        let mut limbs: Vec<u32> = self.get_limbs().clone();
        let mut borrow: u64 = 0;
        for index in 0..limbs.len() {
            let right: u64 = other.get_limbs().get(index).copied().unwrap_or(0) as u64 + borrow;
            let left: u64 = limbs[index] as u64;
            if left >= right {
                limbs[index] = (left - right) as u32;
                borrow = 0;
            } else {
                limbs[index] = (left + (1u64 << LIMB_BITS) - right) as u32;
                borrow = 1;
            }
        }
        let mut value: BigUint = BigUint { limbs };
        value.trim();
        value
    }

    /// Multiplies the number with another one.
    ///
    /// # Arguments
    ///
    /// - `&BigUint` - The number to multiply with.
    ///
    /// # Returns
    ///
    /// - `BigUint` - The product of the two numbers.
    pub fn mul(&self, other: &BigUint) -> BigUint {
        if self.is_zero() || other.is_zero() {
            return BigUint::zero();
        }
        let mut limbs: Vec<u32> = vec![0u32; self.get_limbs().len() + other.get_limbs().len()];
        for (left_index, left) in self.get_limbs().iter().enumerate() {
            let mut carry: u64 = 0;
            for (right_index, right) in other.get_limbs().iter().enumerate() {
                let current: u64 =
                    limbs[left_index + right_index] as u64 + (*left as u64) * (*right as u64) + carry;
                limbs[left_index + right_index] = current as u32;
                carry = current >> LIMB_BITS;
            }
            let mut index: usize = left_index + other.get_limbs().len();
            while carry > 0 {
                let current: u64 = limbs[index] as u64 + carry;
                limbs[index] = current as u32;
                carry = current >> LIMB_BITS;
                index += 1;
            }
        }
        let mut value: BigUint = BigUint { limbs };
        value.trim();
        value
    }

    /// Divides the number by another one.
    ///
    /// # Arguments
    ///
    /// - `&BigUint` - The number to divide by.
    ///
    /// # Returns
    ///
    /// - `(BigUint, BigUint)` - The quotient and the remainder.
    pub fn divmod(&self, other: &BigUint) -> (BigUint, BigUint) {
        if other.is_zero() {
            return (BigUint::zero(), BigUint::zero());
        }
        if self.compare(other) == Ordering::Less {
            return (BigUint::zero(), self.clone());
        }
        let mut quotient: BigUint = BigUint::zero();
        let mut remainder: BigUint = BigUint::zero();
        for index in (0..self.bit_len()).rev() {
            remainder = remainder.shift_left();
            if self.bit(index) {
                remainder.set_lowest_bit();
            }
            if remainder.compare(other) != Ordering::Less {
                remainder = remainder.subtract(other);
                quotient.set_bit(index);
            }
        }
        (quotient, remainder)
    }

    /// Returns the remainder of the division by another number.
    ///
    /// # Arguments
    ///
    /// - `&BigUint` - The number to divide by.
    ///
    /// # Returns
    ///
    /// - `BigUint` - The remainder of the division.
    pub fn remainder(&self, other: &BigUint) -> BigUint {
        self.divmod(other).1
    }

    /// Raises the number to the power of an exponent.
    ///
    /// # Arguments
    ///
    /// - `&BigUint` - The exponent of the power.
    /// - `&BigUint` - The modulus the power is reduced by.
    ///
    /// # Returns
    ///
    /// - `BigUint` - The reduced power.
    pub fn modpow(&self, exponent: &BigUint, modulus: &BigUint) -> BigUint {
        let mut result: BigUint = BigUint::one();
        let mut base: BigUint = self.remainder(modulus);
        for index in 0..exponent.bit_len() {
            if exponent.bit(index) {
                result = result.mul(&base).remainder(modulus);
            }
            base = base.mul(&base).remainder(modulus);
        }
        result
    }

    /// Returns the big endian bytes of the number.
    ///
    /// # Arguments
    ///
    /// - `usize` - The number of bytes the result holds.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - The big endian bytes of the number.
    pub fn to_bytes_be(&self, size: usize) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![0u8; size];
        for index in 0..size {
            let limb: u32 = self.get_limbs().get(index / LIMB_BYTES).copied().unwrap_or(0);
            let shift: u32 = BITS_BYTE * (index % LIMB_BYTES) as u32;
            bytes[size - 1 - index] = ((limb >> shift) & 0xff) as u8;
        }
        bytes
    }

    /// Shifts the number one bit to the left.
    ///
    /// # Returns
    ///
    /// - `BigUint` - The shifted number.
    fn shift_left(&self) -> BigUint {
        let mut limbs: Vec<u32> = self.get_limbs().clone();
        let mut carry: u32 = 0;
        for limb in limbs.iter_mut() {
            let next: u32 = *limb >> (LIMB_BITS - 1);
            *limb = (*limb << 1) | carry;
            carry = next;
        }
        if carry > 0 {
            limbs.push(carry);
        }
        let mut value: BigUint = BigUint { limbs };
        value.trim();
        value
    }

    /// Sets the lowest bit of the number.
    fn set_lowest_bit(&mut self) {
        if self.get_limbs().is_empty() {
            self.get_mut_limbs().push(1);
            return;
        }
        self.get_mut_limbs()[0] |= 1;
    }

    /// Removes the limbs that hold no value.
    fn trim(&mut self) {
        while let Some(limb) = self.get_limbs().last() {
            if *limb != 0 {
                break;
            }
            self.get_mut_limbs().pop();
        }
    }
}

/// Implementation of `PublicKey`.
impl PublicKey {
    /// Builds the public key from the base64 texts of the server.
    ///
    /// # Arguments
    ///
    /// - `&str` - The base64 encoded modulus of the key.
    /// - `&str` - The base64 encoded exponent of the key.
    ///
    /// # Returns
    ///
    /// - `Result<PublicKey, String>` - The key, or the failure that stopped it.
    pub fn from_base64(modulus: &str, exponent: &str) -> Result<PublicKey, String> {
        let modulus_bytes: Vec<u8> = base64_decode(modulus)?;
        let exponent_bytes: Vec<u8> = base64_decode(exponent)?;
        if modulus_bytes.is_empty() || exponent_bytes.is_empty() {
            return Err(ERROR_INVALID_KEY.to_string());
        }
        Ok(PublicKey {
            modulus: BigUint::from_bytes_be(&modulus_bytes),
            exponent: BigUint::from_bytes_be(&exponent_bytes),
            size: modulus_bytes.len(),
        })
    }

    /// Returns the modulus of the key.
    ///
    /// # Returns
    ///
    /// - `&BigUint` - The modulus of the key.
    pub fn modulus(&self) -> &BigUint {
        &self.get_modulus()
    }

    /// Returns the public exponent of the key.
    ///
    /// # Returns
    ///
    /// - `&BigUint` - The public exponent of the key.
    pub fn exponent(&self) -> &BigUint {
        &self.get_exponent()
    }

    /// Returns the number of bytes one encrypted block holds.
    ///
    /// # Returns
    ///
    /// - `usize` - The number of bytes of one block.
    pub fn block_size(&self) -> usize {
        self.get_size()
    }
}
