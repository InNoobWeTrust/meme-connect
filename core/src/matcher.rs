use crate::{meme::*, shadow::ShadowBlend};

pub struct Matcher {}

impl Matcher {
    /// Match with condition, for some plot twist in game play
    pub fn match_condition<F>(shadow_wall: &[ShadowBlend], filter: F) -> Vec<(usize, usize)>
    where
        F: Fn([&(usize, &ShadowBlend); 2]) -> Option<(usize, usize)>,
    {
        let mut couples: Vec<(usize, usize)> = Vec::new();
        let mut g: Vec<(usize, &ShadowBlend)> = Vec::new();
        for (idx, blend) in shadow_wall.iter().enumerate() {
            if blend.blocked {
                if !g.is_empty() {
                    g.clear();
                }
            } else {
                g.push((idx, blend));
                if g.len() > 1 {
                    for i in (0..g.len() - 1).rev() {
                        if let Some((idx1, idx2)) = filter([g.get(i).unwrap(), g.last().unwrap()]) {
                            dbg!(&[idx1, idx2]);
                            couples.push((idx1, idx2));
                        }
                    }
                }
            }
        }
        couples
    }

    /// After casting shadows on a wall, if 2 shadows with the same type have
    /// no blocking shadow(s) in between, then there is a match
    pub fn match_same(shadow_wall: &[ShadowBlend]) -> Vec<(usize, usize)> {
        Self::match_condition(shadow_wall, |w| {
            if !(w[0].1.intersect_meme(w[1].1).is_empty()) {
                Some((w[0].0, w[1].0))
            } else {
                None
            }
        })
    }

    /// Match single type of block, for matching user chosen couple of blocks
    pub fn match_only_meme(shadow_wall: &[ShadowBlend], meme: Meme) -> Vec<(usize, usize)> {
        Self::match_condition(shadow_wall, |w| {
            if w[0].1.memes().contains(&meme) && w[1].1.memes().contains(&meme) {
                Some((w[0].0, w[1].0))
            } else {
                None
            }
        })
    }
}
