use moon_calc::{Moon, Phase, PHASES};
use std::time::SystemTime;

#[wasm_bindgen]
pub struct MoonPhase {
    pub phase: Phase,
    pub moon: Moon,
}
#[wasm_bindgen]
pub fn get_moon_phase() -> MoonPhase {
    let moon = Moon::new(SystemTime::now());
    let mut phase: Phase;
        for phase in PHASES.iter() {
            if moon.phase >= phase.start && moon.phase < phase.end {
                phase = phase
            }
        }
    MoonPhase {
        moon,
        phase,
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
