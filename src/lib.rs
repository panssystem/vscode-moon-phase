use moon_calc::{Moon, Phase, PHASES};
use std::time::SystemTime;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct MoonPhase {
    pub phase: f64,
    phase_name: String,
    phase_emoji: String,
    pub illumination: f64,
}

#[wasm_bindgen]
impl MoonPhase {
    #[wasm_bindgen(getter)]
    pub fn phase_name(&self) -> String {
        self.phase_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_phase_name(&mut self, phase_name: String) {
        self.phase_name = phase_name;
    }

    #[wasm_bindgen(getter)]
    pub fn phase_emoji(&self) -> String {
        self.phase_emoji.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_phase_emoji(&mut self, phase_emoji: String) {
        self.phase_emoji = phase_emoji;
    }
}
#[wasm_bindgen]
pub fn get_moon_phase() -> MoonPhase {
    let moon = Moon::new(SystemTime::now());
    let mut our_phase: &Phase = &PHASES[0];
    for phase in PHASES.iter() {
        if moon.phase >= phase.start && moon.phase < phase.end {
            our_phase = phase
        }
    }
    MoonPhase {
        phase: moon.phase,
        phase_name: our_phase.name.to_string(),
        phase_emoji: our_phase.emoji.to_string(),
        illumination: moon.illumination,
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
