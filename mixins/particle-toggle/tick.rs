// Each tick in the window: follow the mod's switch in Options > Mods, and
// load the saved filter once.
fn particle_toggle_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::particles::ParticleType;
    use crate::game::particle_toggle as pt;
    if !game.native_window() {
        return;
    }
    pt::set_enabled(game.native_on(pt::MOD_ID));
    if !pt::loaded() {
        let all: Vec<&str> = ParticleType::ALL.iter().map(|t| t.name()).collect();
        pt::with_filter(|| game.native_setting(pt::MOD_ID, pt::SETTING), &all, |_| ((), false));
    }
}
