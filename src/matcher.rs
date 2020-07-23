use crate::{meme::*, shadow::ShadowTrace};

pub struct Matcher {}

impl Matcher {
    /// Match with condition, for some plot twist in game play
    pub fn match_condition<'a, T, F>(shadow_wall: &'a mut T, filter: F) -> Vec<(usize, usize)>
    where
        T: ExactSizeIterator<Item = &'a Option<ShadowTrace>>,
        F: Fn(&[(usize, &ShadowTrace)]) -> Option<(usize, usize)>,
    {
        let ungap = shadow_wall
            .enumerate()
            .filter_map(|(idx, possible_shadow)| {
                if let Some(shadow) = possible_shadow {
                    Some((idx, shadow))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, &ShadowTrace)>>();
        ungap.windows(2).filter_map(filter).collect()
    }

    /// After casting shadows on a wall, if 2 shadows with the same type have
    /// no blocking shadow(s) in between, then there is a match
    pub fn match_same<'a, T>(shadow_wall: &'a mut T) -> Vec<(usize, usize)>
    where
        T: ExactSizeIterator<Item = &'a Option<ShadowTrace>>,
    {
        Matcher::match_condition(shadow_wall, |w: &[(usize, &ShadowTrace)]| {
            if w[0].1.meme == w[1].1.meme {
                Some((w[0].0, w[1].0))
            } else {
                None
            }
        })
    }

    /// Match single type of block, for matching user chosen couple of blocks
    pub fn match_only_meme<'a, T>(shadow_wall: &'a mut T, meme: Meme) -> Vec<(usize, usize)>
    where
        T: ExactSizeIterator<Item = &'a Option<ShadowTrace>>,
    {
        Matcher::match_condition(shadow_wall, |w: &[(usize, &ShadowTrace)]| {
            if w[0].1.meme == meme && w[0].1.meme == w[1].1.meme {
                Some((w[0].0, w[1].0))
            } else {
                None
            }
        })
    }
}
