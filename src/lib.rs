pub use self::{block::*, matcher::*, meme::*, shadow::*};

mod block;
mod game_map;
mod matcher;
mod meme;
pub mod prelude;
mod shadow;

#[cfg(test)]
mod test_tracing_shadow {
    use crate::{block::Block, game_map::GameMap, matcher::Matcher, meme::*, shadow::ShadowTrace};

    #[test]
    fn test_shadow_connections() -> Result<(), String> {
        let mut game_map = GameMap::new(6, 3).unwrap();
        println!("Game map:\n{}", game_map._fmt());
        game_map.set_meme(1, &Block::new(1, 1))?;
        game_map.set_meme(1, &Block::new(4, 1))?;
        println!("Game map after filling some couples:\n{}", game_map._fmt());
        let shadows = game_map.cast_horizontal_shadows(0, game_map.height - 1);
        println!("Shadows: {:?}", shadows);
        let couples = Matcher::match_same(&mut shadows.iter());
        println!("couples: {:?}", couples);
        assert_eq!(couples, &[(1, 4)]);
        Ok(())
    }

    #[test]
    fn test_wall_subject() {
        const TRACK: &[Meme] = &[1, 0, 0, 0, 0];
        println!("Track: {:?}", TRACK);
        let wall = TRACK.len() - 1;
        let look_up_to = 0;
        println!("Wall position: {}, look up position: {}", wall, look_up_to);
        let trace = ShadowTrace::trace(&mut TRACK.iter(), wall, look_up_to).unwrap();
        assert_eq!(trace.pos_on_track, 0);
        assert_eq!(trace.meme, 1);
    }

    #[test]
    fn test_limited_vision() {
        const TRACK: &[Meme] = &[1, 0, 0, 0, 0];
        println!("Track: {:?}", TRACK);
        let wall = TRACK.len() - 1;
        let look_up_to = 1;
        println!("Wall position: {}, look up position: {}", wall, look_up_to);
        let possible_trace = ShadowTrace::trace(&mut TRACK.iter(), wall, look_up_to);
        assert_eq!(possible_trace.is_none(), true);
    }

    #[test]
    fn test_wall_blocked() {
        const TRACK: &[Meme] = &[1, 0, 2, 0, 0];
        println!("Track: {:?}", TRACK);
        println!("Wall position: 1, look up position: 1");
        let trace = ShadowTrace::trace(&mut TRACK.iter(), 2, 2).unwrap();
        assert_eq!(trace.pos_on_track, 2);
        assert_eq!(trace.meme, 2);
    }
}
