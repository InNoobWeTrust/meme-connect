#[derive(Debug)]
struct ShadowTrace {
    shadow_of_rune: usize,
    rune_position: usize,
}

fn shadow_tracing<'a, DI>(
    on_track: &mut DI,
    shadow_position: usize,
    look_up_to_postion: usize,
) -> Option<ShadowTrace>
where
    DI: DoubleEndedIterator<Item = &'a usize> + ExactSizeIterator<Item = &'a usize>,
{
    if let Some((rune_position, &shadow_of_rune)) = if shadow_position <= look_up_to_postion {
        on_track
            .enumerate()
            .skip(shadow_position)
            .take(look_up_to_postion - shadow_position)
            .find(|&(_pos, &rune)| rune != 0)
    } else {
        on_track
            .enumerate()
            .skip(look_up_to_postion)
            .rev()
            .skip_while(|&(pos, &_rune)| pos > shadow_position)
            .take(shadow_position - look_up_to_postion)
            .find(|&(_pos, &rune)| rune != 0)
    } {
        return Some(ShadowTrace {
            shadow_of_rune,
            rune_position,
        });
    }
    None
}

fn main() {
    println!("Hello, world!");
    let track = vec![1, 2, 0, 4, 0, 0, 0, 0, 0, 3, 0, 0, 4];
    let shadow = shadow_tracing(&mut track.iter(), 6, 12).unwrap();
    println!("Found shadow: {:#?}", shadow);
}
