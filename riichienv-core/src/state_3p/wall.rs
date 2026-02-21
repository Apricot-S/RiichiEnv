use rand::prelude::*;
use rand::rngs::StdRng;
use sha2::{Digest, Sha256};

use crate::types::is_sanma_excluded_tile;

/// Wall state for 3-player mahjong (108 tiles, sanma hardcoded).
#[derive(Debug, Clone)]
pub struct WallState3P {
    pub tiles: Vec<u8>,
    pub dora_indicators: Vec<u8>,
    pub rinshan_draw_count: u8,
    pub pending_kan_dora_count: u8,
    pub wall_digest: String,
    pub salt: String,
    pub seed: Option<u64>,
    pub hand_index: u64,
}

impl WallState3P {
    pub fn new(seed: Option<u64>) -> Self {
        Self {
            tiles: Vec::new(),
            dora_indicators: Vec::new(),
            rinshan_draw_count: 0,
            pending_kan_dora_count: 0,
            wall_digest: String::new(),
            salt: String::new(),
            seed,
            hand_index: 0,
        }
    }

    pub fn shuffle(&mut self) {
        // 3P: 108 tiles (no 2m-8m)
        let mut w: Vec<u8> = (0..136u8).filter(|&t| !is_sanma_excluded_tile(t)).collect();

        let mut rng = if let Some(episode_seed) = self.seed {
            let hand_seed = splitmix64(episode_seed.wrapping_add(self.hand_index));
            self.hand_index = self.hand_index.wrapping_add(1);
            StdRng::seed_from_u64(hand_seed)
        } else {
            self.hand_index = self.hand_index.wrapping_add(1);
            StdRng::from_entropy()
        };

        w.shuffle(&mut rng);
        self.salt = format!("{:016x}", rng.next_u64());

        // Calculate digest
        let mut hasher = Sha256::new();
        hasher.update(self.salt.as_bytes());
        for &t in &w {
            hasher.update([t]);
        }
        self.wall_digest = format!("{:x}", hasher.finalize());

        w.reverse();
        self.tiles = w;

        self.dora_indicators.clear();
        if self.tiles.len() > 5 {
            self.dora_indicators.push(self.tiles[4]);
        }
        self.rinshan_draw_count = 0;
        self.pending_kan_dora_count = 0;
    }

    pub fn load_wall(&mut self, tiles: Vec<u8>) {
        let mut t = tiles;

        // MjSoul 3P dead wall wraps around a table corner.
        // Paishan positions 94-103 encode dora indicator stacks as:
        //   [94,95]=D3/U3  [96,97]=D2/U2  [98,99]=D1/U1  [100,101]=D4/U4  [102,103]=D5/U5
        // Rearrange to standard descending order (D5, D4, D3, D2, D1 from left
        // to right) so that after reversal D1 lands at index 4, matching 4P.
        if t.len() == 108 {
            let orig: [u8; 10] = t[94..104].try_into().unwrap();
            t[94] = orig[8];
            t[95] = orig[9]; // D5 (was at 102,103)
            t[96] = orig[6];
            t[97] = orig[7]; // D4 (was at 100,101)
            t[98] = orig[0];
            t[99] = orig[1]; // D3 (was at 94,95)
            t[100] = orig[2];
            t[101] = orig[3]; // D2 (was at 96,97)
            t[102] = orig[4];
            t[103] = orig[5]; // D1 (was at 98,99)
        }

        t.reverse();
        self.tiles = t;
        self.dora_indicators.clear();
        if self.tiles.len() > 5 {
            self.dora_indicators.push(self.tiles[4]);
        }
        self.rinshan_draw_count = 0;
        self.pending_kan_dora_count = 0;
    }
}

fn splitmix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
