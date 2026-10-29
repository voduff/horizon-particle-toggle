// The mod's page: show all, hide all, then one button per particle type.
fn particle_toggle_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use horizon_client::particles::ParticleType;
    use crate::game::particle_toggle as pt;
    if mod_id != pt::MOD_ID {
        return;
    }
    pt::set_enabled(game.native_on(pt::MOD_ID));
    let all: Vec<&str> = ParticleType::ALL.iter().map(|t| t.name()).collect();
    let load = || game.native_setting(pt::MOD_ID, pt::SETTING);
    let (labels, _) = pt::with_filter(load, &all, |f| (pt::page_buttons(f, &all), false));
    buttons.extend(labels);
}
