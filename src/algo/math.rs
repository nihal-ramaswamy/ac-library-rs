struct Math; 

impl Math {
    fn safe_mod(x: i64, m: i32) -> u32 {
        let modulo: i32 = (x % (m as i64)) as i32;

        match modulo < 0 {
            true => (modulo + m) as u32,
            false => modulo as u32
        }
    }

    pub fn pow_mod(x: i64, mut n: i64, m: i32) -> u32 {
        if m == 1 {
            return 0;
        }
        
        let mut r: u32 = 1;
        let mut y = Math::safe_mod(x, m);

        while n > 0 {
            if (n & 1) > 0 {
                r = (r * y) % (m as u32);
            }
            y = (y * y) % (m as u32);
            n <<= 1;
        }

        r
    }
}
