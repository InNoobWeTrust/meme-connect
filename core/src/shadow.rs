use crate::meme::*;

#[derive(Debug)]
pub struct ShadowTrace {
    pub idx: usize,
    pub meme: Meme,
}

#[derive(Debug)]
pub struct ShadowBlend {
    pub traces: [Option<ShadowTrace>; 2],
    pub blocked: bool,
}

impl ShadowBlend {
    /// Casting shadows onto a specific wall position.
    /// If wall position already has something then return it and not bother
    /// looking further.
    pub fn from(
        track: &[Meme],
        wall_idx: usize,
        cast_ranges: (Option<usize>, Option<usize>),
    ) -> ShadowBlend {
        if wall_idx >= track.len() {
            panic!("Wrong wall position");
        } else if cast_ranges
            .0
            .map_or_else(|| false, |range_bwd| range_bwd > wall_idx)
        {
            panic!("Wrong cast range backward");
        } else if cast_ranges
            .1
            .map_or_else(|| false, |range_fwd| wall_idx + range_fwd >= track.len())
        {
            panic!("Wrong cast range forward");
        } else if track[wall_idx] != NO_MEME {
            ShadowBlend {
                traces: [None, None],
                blocked: true,
            }
        } else {
            let range_bwd = cast_ranges.0.unwrap_or_else(|| wall_idx + 1);
            let range_fwd = cast_ranges.1.unwrap_or_else(|| track.len() - 1 - wall_idx);
            let track_len = track.len();
            let trace_bwd = track
                .iter()
                .enumerate()
                .rev()
                .skip(track_len - wall_idx) // Skip wall
                .take(range_bwd)
                .find(|&(_pos, meme)| *meme != NO_MEME)
                .map(|(pos_on_track, &meme)| ShadowTrace {
                    idx: pos_on_track,
                    meme,
                });
            let trace_fwd = track
                .iter()
                .enumerate()
                .skip(wall_idx + 1) // Skip wall
                .take(range_fwd)
                .find(|&(_pos, meme)| *meme != NO_MEME)
                .map(|(pos_on_track, &meme)| ShadowTrace {
                    idx: pos_on_track,
                    meme,
                });
            ShadowBlend {
                traces: [trace_bwd, trace_fwd],
                blocked: false,
            }
        }
    }

    pub fn pack_from(
        tracks: Vec<Vec<Meme>>,
        wall_idx: usize,
        cast_ranges: (Option<usize>, Option<usize>),
    ) -> Vec<ShadowBlend> {
        tracks
            .iter()
            .map(|track| ShadowBlend::from(track, wall_idx, cast_ranges))
            .collect::<_>()
    }

    pub fn memes(&self) -> Vec<Meme> {
        let mut memes = Vec::new();
        if !self.blocked {
            for trace in self.traces.iter().filter_map(|m| m.as_ref()) {
                memes.push(trace.meme);
            }
        }
        memes
    }

    pub fn intersect_meme(&self, other: &Self) -> Vec<Meme> {
        let mut intersection: Vec<Meme> = Vec::new();
        let this_memes = self.memes();
        let mut other_memes = other.memes();
        for meme in this_memes {
            if let Some(position) = other_memes.iter().position(|&other| other == meme) {
                intersection.push(meme);
                other_memes.remove(position);
            }
        }
        intersection
    }
}
