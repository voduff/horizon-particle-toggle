// `/particles ...` in chat.
fn particle_toggle_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use horizon_client::particles::ParticleType;
    use crate::game::particle_toggle as pt;
    let (name, args) = command.split_once(' ').unwrap_or((command, ""));
    if *ran || !name.eq_ignore_ascii_case("particles") {
        return;
    }
    *ran = true;
    let all: Vec<&str> = ParticleType::ALL.iter().map(|t| t.name()).collect();
    let load = || game.native_setting(pt::MOD_ID, pt::SETTING);
    let (lines, saved) = pt::with_filter(load, &all, |f| pt::command(f, args, &all));
    if let Some(saved) = saved {
        game.native_set_setting(pt::MOD_ID, pt::SETTING, &saved);
    }
    for line in lines {
        game.native_chat(&format!("[Particle Toggle] {line}"));
    }
}
