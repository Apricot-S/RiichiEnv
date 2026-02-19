mod agari;
pub mod errors;
pub mod hand_evaluator;
pub mod score;
mod tests;
pub mod types;
mod yaku;

pub mod action;
pub mod observation;
pub mod parser;
pub mod replay;
pub mod rule;
mod shanten;
pub mod state;
pub mod win_projection;
mod yaku_checker;

pub fn check_riichi_candidates(tiles_136: Vec<u8>) -> Vec<u32> {
    let mut candidates = Vec::new();
    // Convert to 34-tile hand
    let mut tiles_34 = Vec::with_capacity(tiles_136.len());
    for t in &tiles_136 {
        tiles_34.push(t / 4);
    }

    for (i, &t_discard) in tiles_136.iter().enumerate() {
        let mut hand = types::Hand::default();
        for (j, &t) in tiles_34.iter().enumerate() {
            if i != j {
                hand.add(t);
            }
        }

        if agari::is_tenpai(&mut hand) {
            candidates.push(t_discard as u32);
        }
    }
    candidates
}
