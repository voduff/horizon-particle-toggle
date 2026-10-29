// Options > Mods lists the mod.
fn particle_toggle_list(mods: &mut Vec<NativeMod>) {
    use crate::game::particle_toggle as pt;
    mods.push(NativeMod {
        id: pt::MOD_ID.to_owned(),
        title: pt::TITLE.to_owned(),
        description: pt::DESCRIPTION.to_owned(),
        keys: Vec::new(),
    });
}
