use crate::meme::*;

#[derive(Debug)]
pub struct ShadowTrace {
    pub pos_on_track: usize,
    pub meme: Meme,
}

impl ShadowTrace {
    /// Casting shadow onto a specific position. If shadow position already has something then
    /// return it and not bother looking further
    pub fn trace<'a, T>(
        on_track: &mut T,
        shadow_position: usize,
        look_up_to_postion: usize,
    ) -> Option<ShadowTrace>
    where
        T: DoubleEndedIterator<Item = &'a Meme> + ExactSizeIterator<Item = &'a Meme>,
    {
        if shadow_position >= on_track.len() || look_up_to_postion >= on_track.len() {
            panic!("Wrong input");
        } else if shadow_position <= look_up_to_postion {
            on_track
                .enumerate()
                .skip(shadow_position)
                .take(look_up_to_postion - shadow_position + 1)
                //.inspect(|&(pos, &meme)| println!("=> tracing step: pos={}, meme={:?}", pos, meme))
                .find(|&(_pos, &meme)| meme != NO_MEME)
        } else {
            on_track
                .enumerate()
                .skip(look_up_to_postion)
                .rev()
                .skip_while(|&(pos, &_meme)| pos > shadow_position)
                //.inspect(|&(pos, &meme)| println!("=> tracing step: pos={}, meme={:?}", pos, meme))
                .find(|&(_pos, &meme)| meme != NO_MEME)
        }
        .map(|(pos_on_track, &meme)| ShadowTrace { pos_on_track, meme })
    }

    pub fn trace_multi<'a, T, U>(
        tracks: &mut T,
        shadow_position: usize,
        look_up_to_postion: usize,
    ) -> Vec<Option<ShadowTrace>>
    where
        T: Iterator<Item = U>,
        U: DoubleEndedIterator<Item = &'a Meme> + ExactSizeIterator<Item = &'a Meme>,
    {
        tracks
            .map(|mut track| ShadowTrace::trace(&mut track, shadow_position, look_up_to_postion))
            .collect::<_>()
    }
}
