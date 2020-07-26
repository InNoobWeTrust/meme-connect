mod block;
mod game_map;
mod matcher;
mod meme;
pub mod prelude;
mod shadow;

#[cfg(test)]
mod test_tracing_shadow {
    use crate::{block::Block, game_map::GameMap, matcher::Matcher, meme::*, shadow::ShadowBlend};

    #[test]
    fn test_shadow_connections() -> Result<(), String> {
        let mut game_map = GameMap::new(6, 6).unwrap();
        println!("Game map:\n{}", game_map._fmt());
        game_map.set_meme(1, &Block::new(1, 1))?;
        game_map.set_meme(1, &Block::new(4, 4))?;
        println!("Game map after filling some couples:\n{}", game_map._fmt());
        let horizontal_shadows = game_map.cast_horizontal_shadows(0, (None, None));
        println!("Horizontal shadows: {:?}", horizontal_shadows);
        let horizontal_couples = Matcher::match_same(&horizontal_shadows);
        println!("Horizontal couples: {:?}", horizontal_couples);
        assert_eq!(horizontal_couples, &[(1, 4)]);
        let vertical_shadows = game_map.cast_vertical_shadows(2, (None, None));
        println!("Vertical shadows: {:?}", vertical_shadows);
        let vertical_couples = Matcher::match_same(&vertical_shadows);
        println!("Vertical couples: {:?}", vertical_couples);
        assert_eq!(vertical_couples, &[(1, 4)]);
        Ok(())
    }

    #[test]
    fn test_wall_subject() {
        const TRACK: &[Meme] = &[1, 0, 0, 0, 0];
        println!("Track: {:?}", TRACK);
        let wall = TRACK.len() - 1;
        println!("Wall position: {}", wall);
        let traces = ShadowBlend::from(TRACK.to_vec(), wall, (None, None));
        assert_eq!(traces.traces[0].as_ref().unwrap().idx, 0);
        assert_eq!(traces.traces[0].as_ref().unwrap().meme, 1);
        assert!(traces.traces[1].is_none(), true);
    }

    #[test]
    fn test_limited_vision() {
        const TRACK: &[Meme] = &[1, 0, 0, 0, 0];
        println!("Track: {:?}", TRACK);
        let wall = TRACK.len() - 1;
        let vision = 1;
        println!("Wall position: {}, vision: {}", wall, vision);
        let possible_traces = ShadowBlend::from(TRACK.to_vec(), wall, (Some(vision), None));
        assert!(possible_traces.traces[0].is_none());
        assert!(possible_traces.traces[1].is_none());
    }

    #[test]
    fn test_wall_blocked() {
        const TRACK: &[Meme] = &[1, 0, 2, 0, 3, 0, 4];
        println!("Track: {:?}", TRACK);
        let wall = 3;
        let vision = 3;
        println!("Wall position: {}, vision: {}", wall, vision);
        let traces = ShadowBlend::from(TRACK.to_vec(), wall, (Some(vision), Some(vision)));
        assert_eq!(traces.traces[0].as_ref().unwrap().idx, 2);
        assert_eq!(traces.traces[0].as_ref().unwrap().meme, 2);
        assert_eq!(traces.traces[1].as_ref().unwrap().idx, 4);
        assert_eq!(traces.traces[1].as_ref().unwrap().meme, 3);
    }
}
