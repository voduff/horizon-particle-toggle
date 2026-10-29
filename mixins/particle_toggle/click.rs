// A click on the mod's page toggles a type (or all of them) and saves.
fn particle_toggle_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use horizon_client::particles::ParticleType;
    use crate::game::particle_toggle as pt;
    if mod_id != pt::MOD_ID {
        return;
    }
    let all: Vec<&str> = ParticleType::ALL.iter().map(|t| t.name()).collect();
    let load = || game.native_setting(pt::MOD_ID, pt::SETTING);
    let ((), saved) = pt::with_filter(load, &all, |f| ((), pt::page_click(f, button, &all)));
    if let Some(saved) = saved {
        game.native_set_setting(pt::MOD_ID, pt::SETTING, &saved);
    }
}
