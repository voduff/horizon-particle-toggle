// Each frame, after the ticks and before drawing: hidden types leave the
// live particles and the queue, so they never draw.
fn particle_toggle_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::particle_toggle as pt;
    if !game.native_window() {
        return;
    }
    let engine = &mut game.particles;
    pt::retain_shown(engine.__horizon_mixins_particle_toggle_live(), |p| p.ty as usize);
    pt::retain_shown(engine.__horizon_mixins_particle_toggle_queued(), |p| p.ty as usize);
}
