use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new() -> Self {
        let seed = Self::generate_seed();
        Self { state: seed }
    }

    fn generate_seed() -> u64 {
        let mut buf = [0u8; 8];
        if let Ok(mut file) = File::open("/dev/urandom") {
            if file.read_exact(&mut buf).is_ok() {
                let val = u64::from_le_bytes(buf);
                if val != 0 {
                    return val;
                }
            }
        }

        // Fallback using high-resolution time and process IDs
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let pid = std::process::id() as u128;
        let mixed = now ^ (pid << 48);
        (mixed as u64) ^ ((mixed >> 64) as u64) ^ 0x9e3779b97f4a7c15
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    pub fn choose_index(&mut self, bound: usize) -> usize {
        if bound <= 1 {
            return 0;
        }
        (self.next_u64() % (bound as u64)) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_generates_within_bounds() {
        let mut rng = Rng::new();
        for bound in [2, 5, 10, 100] {
            for _ in 0..100 {
                let idx = rng.choose_index(bound);
                assert!(idx < bound);
            }
        }
    }

    #[test]
    fn test_rng_next_varies() {
        let mut rng = Rng::new();
        let a = rng.next_u64();
        let b = rng.next_u64();
        assert_ne!(a, b);
    }
}
