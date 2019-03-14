#[macro_use]
extern crate conrod;
extern crate conrod_derive;

extern crate audioengine;

mod event_loop;
mod types;
mod ui;

#[allow(unused_imports)]
use audioengine::types::KeyAction;

#[allow(unused_imports)]
use ui::Ui;

#[allow(unused_imports)]
use std::f64::consts::PI;

#[allow(unused_variables)]
fn main() -> Result<(), Error> {
    let audioengine = audioengine::EngineController::start();

    let sample_rate = audioengine.sample_rate;
    let time_per_sample = 1.0 / sample_rate;

    let mut time = 0.0;

    let mut current_key = None;
    let synth = move |action: Option<i32>| {
        time += time_per_sample;
        if action != current_key {
            current_key = action;

            println!("{:?}", action);
        }

        let frequency: f64;
        let get_note = move |note: i32| {
            440_f64 * 1.059463094359_f64.powi(note)
        };

        match action {
            Some(i) => frequency = get_note(i),
            None => frequency = 0.0
        }
        
        let amplitude: f64 = 1.0;
        let phase: f64 = 1.0;
        let pi = PI;

        let sine = f64::from(2) * PI * frequency * time + phase;
        let sine_wave = amplitude * sine.sin();

        let clamp = move |min: f64, max: f64, num: f64| {
            let ret: f64;
            
            if num < min {
                ret = min;
            } else if num > max {
                ret = max;
            } else {
                ret = num
            }

            ret
        };

        clamp(0.01_f64, 0.99_f64, sine_wave)
    };

    audioengine.set_processor_function(Box::new(synth));

    let mut window = Ui::new(
        "Synthesizer",
        [1280.0, 800.0],
        audioengine,
        None,
        None,
        None,
    );

    window.show();

    Ok(())
}

#[derive(Debug)]
enum Error {}
