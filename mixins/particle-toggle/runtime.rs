// Particle Toggle's logic. This file is both the standalone Cargo library and
// the injected `crate::game::particle_toggle` module of the game binary, so it uses
// only std types: particle types are their registry names (`flame`).

use std::collections::BTreeSet;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

pub const MOD_ID: &str = "particle-toggle";
pub const TITLE: &str = "Particle Toggle 0.1.0";
pub const DESCRIPTION: &str = "Show or hide each particle type. Also /particles.";
/// The setting holding the hidden types, one name a line.
pub const SETTING: &str = "hidden";

/// The hidden particle types.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Filter {
    hidden: BTreeSet<String>,
}

impl Filter {
    pub fn is_hidden(&self, name: &str) -> bool {
        self.hidden.contains(name)
    }
    pub fn is_empty(&self) -> bool {
        self.hidden.is_empty()
    }
    pub fn hidden(&self) -> impl Iterator<Item = &str> {
        self.hidden.iter().map(String::as_str)
    }
    /// Hides or shows `name`; whether that changed anything.
    pub fn set(&mut self, name: &str, hidden: bool) -> bool {
        if hidden {
            self.hidden.insert(name.to_owned())
        } else {
            self.hidden.remove(name)
        }
    }
    pub fn toggle(&mut self, name: &str) {
        let hidden = !self.is_hidden(name);
        self.set(name, hidden);
    }
    /// The saved form: one name a line.
    pub fn save(&self) -> String {
        self.hidden.iter().map(|n| format!("{n}\n")).collect()
    }
    /// Reads `save`'s form, keeping only names in `all`.
    pub fn load(text: &str, all: &[&str]) -> Self {
        let hidden = text
            .lines()
            .map(normalize)
            .filter(|n| all.contains(&n.as_str()))
            .collect();
        Self { hidden }
    }
}

/// `Minecraft:Soul_Fire_Flame ` → `soul_fire_flame`.
pub fn normalize(name: &str) -> String {
    let name = name.trim().to_ascii_lowercase();
    match name.strip_prefix("minecraft:") {
        Some(rest) => rest.to_owned(),
        None => name,
    }
}

/// `soul_fire_flame` → `Soul Fire Flame`.
pub fn pretty(name: &str) -> String {
    name.split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut chars = w.chars();
            chars.next().map_or_else(String::new, |c| {
                c.to_ascii_uppercase().to_string() + chars.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// `*` matches any run of characters; anything else matches itself.
pub fn glob(pattern: &str, name: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 1 {
        return pattern == name;
    }
    let (first, last) = (parts[0], parts[parts.len() - 1]);
    if !name.starts_with(first) || name.len() < first.len() + last.len() {
        return false;
    }
    let mut rest = &name[first.len()..name.len() - last.len()];
    for part in &parts[1..parts.len() - 1] {
        match rest.find(part) {
            Some(i) => rest = &rest[i + part.len()..],
            None => return false,
        }
    }
    name.ends_with(last)
}

/// The types `pattern` names: `all`, a glob, or one name.
fn matching<'a>(pattern: &str, all: &[&'a str]) -> Vec<&'a str> {
    let pattern = normalize(pattern);
    if pattern == "all" {
        return all.to_vec();
    }
    all.iter().copied().filter(|n| glob(&pattern, n)).collect()
}

/// The settings page, in click order: two bulk buttons, then each type by
/// its name in alphabetical order.
pub fn page_types<'a>(all: &[&'a str]) -> Vec<&'a str> {
    let mut types = all.to_vec();
    types.sort_unstable();
    types
}

pub fn page_buttons(filter: &Filter, all: &[&str]) -> Vec<String> {
    let mut buttons = vec![
        "Show all particles".to_owned(),
        "Hide all particles".to_owned(),
    ];
    for name in page_types(all) {
        let state = if filter.is_hidden(name) {
            "Hidden"
        } else {
            "Shown"
        };
        buttons.push(format!("{}: {state}", pretty(name)));
    }
    buttons
}

/// A click on the page's button `button`; whether the filter changed.
pub fn page_click(filter: &mut Filter, button: usize, all: &[&str]) -> bool {
    let before = filter.clone();
    match button {
        0 => *filter = Filter::default(),
        1 => all.iter().for_each(|n| {
            filter.set(n, true);
        }),
        i => {
            if let Some(name) = page_types(all).get(i - 2) {
                filter.toggle(name);
            }
        }
    }
    *filter != before
}

const USAGE: &str = "/particles hide|show|toggle <type|pattern*|all>..., /particles list, /particles types [pattern], /particles reset";

/// `/particles ...` (`args` is what follows the name): the chat lines to show,
/// and whether the filter changed.
pub fn command(filter: &mut Filter, args: &str, all: &[&str]) -> (Vec<String>, bool) {
    let mut words = args.split_whitespace();
    let action = words.next().unwrap_or("help").to_ascii_lowercase();
    let names: Vec<&str> = words.collect();
    match action.as_str() {
        "list" => {
            let hidden: Vec<&str> = filter.hidden().collect();
            let line = if hidden.is_empty() {
                "No particle types are hidden.".to_owned()
            } else {
                format!("Hidden ({}): {}", hidden.len(), hidden.join(", "))
            };
            (vec![line], false)
        }
        "types" => {
            let pattern = names
                .first()
                .map_or_else(|| "*".to_owned(), |p| normalize(p));
            let pattern = if pattern.contains('*') {
                pattern
            } else {
                format!("*{pattern}*")
            };
            let found: Vec<&str> = page_types(all)
                .into_iter()
                .filter(|n| glob(&pattern, n))
                .collect();
            (
                vec![format!(
                    "Particle types ({}): {}",
                    found.len(),
                    found.join(", ")
                )],
                false,
            )
        }
        "reset" => {
            let changed = !filter.is_empty();
            *filter = Filter::default();
            (vec!["Every particle type is shown.".to_owned()], changed)
        }
        "hide" | "off" | "show" | "on" | "toggle" => {
            if names.is_empty() {
                return (vec![format!("Name a particle type. {USAGE}")], false);
            }
            let before = filter.clone();
            let mut lines = Vec::new();
            let mut done = Vec::new();
            for pattern in names {
                let found = matching(pattern, all);
                if found.is_empty() {
                    lines.push(format!(
                        "No particle type matches \"{pattern}\". See /particles types."
                    ));
                }
                for name in found {
                    match action.as_str() {
                        "hide" | "off" => {
                            filter.set(name, true);
                        }
                        "show" | "on" => {
                            filter.set(name, false);
                        }
                        _ => filter.toggle(name),
                    }
                    done.push(name);
                }
            }
            if !done.is_empty() {
                let state = |n: &str| {
                    if filter.is_hidden(n) {
                        "hidden"
                    } else {
                        "shown"
                    }
                };
                let summary = if done.len() > 6 {
                    format!("{} particle types updated.", done.len())
                } else {
                    done.iter()
                        .map(|n| format!("{n}: {}", state(n)))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                lines.push(summary);
            }
            (lines, *filter != before)
        }
        _ => (vec![USAGE.to_owned()], false),
    }
}

// The game's filter, and the same as one flag per type in `all`'s order
// (`ParticleType::ALL`, so a particle's type is its index). `ACTIVE` lets
// the frame hook skip the lock while nothing is hidden or the player turned
// the mod off.
struct State {
    filter: Option<Filter>,
    mask: Vec<bool>,
}
static STATE: Mutex<State> = Mutex::new(State {
    filter: None,
    mask: Vec::new(),
});
static ENABLED: AtomicBool = AtomicBool::new(true);
static ACTIVE: AtomicBool = AtomicBool::new(false);

fn state() -> std::sync::MutexGuard<'static, State> {
    STATE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn refresh(filter: &Filter) {
    let active = ENABLED.load(Ordering::Relaxed) && !filter.is_empty();
    ACTIVE.store(active, Ordering::Relaxed);
}

/// Runs `f` on the game's filter, loading it with `load` the first time.
/// Returns `f`'s result and the saved form when `f` reports a change.
pub fn with_filter<R>(
    load: impl FnOnce() -> Option<String>,
    all: &[&str],
    f: impl FnOnce(&mut Filter) -> (R, bool),
) -> (R, Option<String>) {
    let mut guard = state();
    let state = &mut *guard;
    let filter = state
        .filter
        .get_or_insert_with(|| Filter::load(&load().unwrap_or_default(), all));
    let (result, changed) = f(filter);
    state.mask = all.iter().map(|n| filter.is_hidden(n)).collect();
    refresh(filter);
    (result, changed.then(|| filter.save()))
}

/// Whether `with_filter` has loaded the saved filter yet.
pub fn loaded() -> bool {
    state().filter.is_some()
}

/// Whether the player left the mod on in Options > Mods.
pub fn set_enabled(on: bool) {
    if ENABLED.swap(on, Ordering::Relaxed) != on
        && let Some(filter) = state().filter.as_ref()
    {
        refresh(filter);
    }
}

/// Removes the hidden types from `list`; `index` reads a particle's type as
/// its index in `all`.
pub fn retain_shown<T>(list: &mut Vec<T>, index: impl Fn(&T) -> usize) {
    if !ACTIVE.load(Ordering::Relaxed) || list.is_empty() {
        return;
    }
    let guard = state();
    list.retain(|item| !guard.mask.get(index(item)).copied().unwrap_or(false));
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: &[&str] = &[
        "flame",
        "soul_fire_flame",
        "smoke",
        "dripping_water",
        "dripping_lava",
    ];

    #[test]
    fn names_are_normalized_and_prettified() {
        assert_eq!(normalize(" Minecraft:Soul_Fire_Flame "), "soul_fire_flame");
        assert_eq!(pretty("soul_fire_flame"), "Soul Fire Flame");
        assert_eq!(pretty("smoke"), "Smoke");
    }

    #[test]
    fn globs_match_prefixes_suffixes_and_middles() {
        assert!(glob("dripping_*", "dripping_lava"));
        assert!(glob("*flame", "soul_fire_flame"));
        assert!(glob("*fire*", "soul_fire_flame"));
        assert!(glob("*", "smoke"));
        assert!(!glob("dripping_*", "flame"));
        assert!(!glob("a*a", "a"));
        assert!(glob("flame", "flame"));
        assert!(!glob("flame", "soul_fire_flame"));
    }

    #[test]
    fn save_and_load_keep_only_known_types() {
        let mut filter = Filter::default();
        filter.set("flame", true);
        filter.set("smoke", true);
        let text = filter.save() + "not_a_particle\nminecraft:dripping_lava\n";
        let loaded = Filter::load(&text, ALL);
        assert_eq!(
            loaded.hidden().collect::<Vec<_>>(),
            ["dripping_lava", "flame", "smoke"]
        );
    }

    #[test]
    fn commands_hide_show_toggle_and_reset() {
        let mut f = Filter::default();
        let (_, changed) = command(&mut f, "hide flame dripping_*", ALL);
        assert!(changed);
        assert!(
            f.is_hidden("flame") && f.is_hidden("dripping_water") && f.is_hidden("dripping_lava")
        );
        let (_, changed) = command(&mut f, "hide flame", ALL);
        assert!(!changed);
        command(&mut f, "show minecraft:flame", ALL);
        assert!(!f.is_hidden("flame"));
        command(&mut f, "toggle smoke dripping_lava", ALL);
        assert!(f.is_hidden("smoke") && !f.is_hidden("dripping_lava"));
        let (lines, changed) = command(&mut f, "off nope", ALL);
        assert!(!changed && lines[0].contains("nope"));
        let (_, changed) = command(&mut f, "reset", ALL);
        assert!(changed && f.is_empty());
        let (lines, _) = command(&mut f, "types drip", ALL);
        assert_eq!(
            lines[0],
            "Particle types (2): dripping_lava, dripping_water"
        );
    }

    #[test]
    fn page_lists_bulk_buttons_then_sorted_types() {
        let mut f = Filter::default();
        let buttons = page_buttons(&f, ALL);
        assert_eq!(buttons[0], "Show all particles");
        assert_eq!(buttons[2], "Dripping Lava: Shown");
        assert!(page_click(&mut f, 2, ALL));
        assert!(f.is_hidden("dripping_lava"));
        assert_eq!(page_buttons(&f, ALL)[2], "Dripping Lava: Hidden");
        assert!(page_click(&mut f, 1, ALL));
        assert_eq!(f.hidden().count(), ALL.len());
        assert!(page_click(&mut f, 0, ALL));
        assert!(f.is_empty());
        assert!(!page_click(&mut f, 99, ALL));
    }
}
