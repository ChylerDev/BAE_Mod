#[cfg(test)]
mod tests {
    use bae_gen::*;
    use bae_mod::*;
    use bae_types::*;
    use bae_utils::*;

    use std::fs::File;
    use std::time::Duration;

    const SAMPLE_RATE: usize = 48_000;

    const FILE_PREFIX: &'static str = ".junk/modifiers/";

    #[test]
    fn test_adsr() -> Result<(), ()> {
        let mut a = ADSR::new(
            Duration::from_secs_f64(0.03125),
            Duration::from_secs_f64(0.125),
            linear_to_db(0.5),
            Duration::from_secs_f64(0.5),
            SAMPLE_RATE as Math,
        );

        let mut g = Sine::new(440.0, SAMPLE_RATE as Math);
        let mut t = bae_rs::SamplerackT::new();

        for i in 0..SAMPLE_RATE {
            if i == SAMPLE_RATE / 2 {
                a.trigger_release();
            }

            if i == 1199 {
                println!("{} - {}", i, t.last().unwrap());
            }

            t.push(a.process(g.process()));
        }

        let f = ".junk/modifiers/adsr.wav";
        WaveWriteOptions::new()
            .bps(24)?
            .r(SAMPLE_RATE as Math)
            .clip(true)
            .write(vec![t], &mut File::create(f).unwrap())
            .unwrap();

        Ok(())
    }

    #[test]
    fn test_bandpass() {
        let mut bp1 = BandPass::from_corners((100.0, 200.0), SAMPLE_RATE as Math);
        let f1 = "bandpass_100_200.wav";
        let mut bp2 = BandPass::from_corners((200.0, 225.0), SAMPLE_RATE as Math);
        let f2 = "bandpass_200_225.wav";
        let mut bp3 = BandPass::from_corners((20.0, 20000.0), SAMPLE_RATE as Math);
        let f3 = "bandpass_20_20k.wav";

        run_modifier(&mut bp1, f1).unwrap();
        run_modifier(&mut bp2, f2).unwrap();
        run_modifier(&mut bp3, f3).unwrap();
    }

    #[test]
    fn test_delay() {
        let mut d = Delay::new(std::time::Duration::from_secs_f64(0.5), SAMPLE_RATE as Math);

        run_modifier(&mut d, "delay.wav").unwrap();
    }

    #[test]
    fn test_echo() -> Result<(), ()> {
        let mut e = Echo::new(
            std::time::Duration::from_secs_f64(0.25),
            0.5,
            SAMPLE_RATE as Math,
        );

        let mut g = Sine::new(440.0, SAMPLE_RATE as Math);
        let mut t = bae_rs::SamplerackT::new();

        for _ in 0..SAMPLE_RATE {
            t.push(e.process(g.process() * 0.5));
        }

        let f = ".junk/modifiers/echo.wav";
        WaveWriteOptions::new()
            .bps(24)?
            .r(SAMPLE_RATE as Math)
            .clip(true)
            .write(vec![t], &mut File::create(f).unwrap())
            .unwrap();

        Ok(())
    }

    #[test]
    fn test_envelope() -> Result<(), ()> {
        let mut e = Envelope::new(20.0, 20_000.0, SAMPLE_RATE as Math);

        let mut a = ADSR::new(
            Duration::from_secs_f64(0.03125),
            Duration::from_secs_f64(0.125),
            linear_to_db(0.5),
            Duration::from_secs_f64(0.5),
            SAMPLE_RATE as Math,
        );
        let mut g = Sine::new(440.0, SAMPLE_RATE as Math);
        let mut t = bae_rs::SamplerackT::new();

        for i in 0..SAMPLE_RATE {
            if i == SAMPLE_RATE / 2 {
                a.trigger_release();
            }

            t.push(e.process(a.process(g.process())));
        }

        let f = ".junk/modifiers/envelope.wav";
        WaveWriteOptions::new()
            .bps(24)?
            .r(SAMPLE_RATE as Math)
            .clip(true)
            .write(vec![t], &mut File::create(f).unwrap())
            .unwrap();

        Ok(())
    }

    #[test]
    fn test_gain() {
        let mut g = Gain::new(0.125);

        run_modifier(&mut g, "gain.wav").unwrap();
    }

    #[test]
    fn test_generic() -> Result<(), ()> {
        let mut g = Generic::new(
            {
                let mut v = Zeros::new();

                v.push_back((0, 0.69));
                v.push_back((1, 0.32));
                v.push_back((2, 0.13));
                v.push_back((6, 0.4892));

                v
            },
            {
                let mut v = Poles::new();

                v.push_back((1, 0.378_946_24));
                v.push_back((5, 0.125_012_84));
                v.push_back((300, 0.104_536_59));

                v
            },
        );

        let mut s = Sine::new(440.0, SAMPLE_RATE as Math);
        let mut t = bae_rs::SamplerackT::new();

        for _ in 0..SAMPLE_RATE {
            t.push(g.process(s.process() * 0.25));
        }

        let f = ".junk/modifiers/generic.wav";
        WaveWriteOptions::new()
            .bps(24)?
            .r(SAMPLE_RATE as Math)
            .clip(true)
            .write(vec![t], &mut File::create(f).unwrap())
            .unwrap();

        Ok(())
    }

    #[test]
    fn test_highpass() {
        let mut hp = HighPass::new(440.0, 1.0, SAMPLE_RATE as Math);

        run_modifier(&mut hp, "highpass.wav").unwrap();
    }

    #[test]
    fn test_lowpass() -> Result<(), ()> {
        let mut lp = LowPass::new(440.0, 0.0, SAMPLE_RATE as Math);
        let mut n = Noise::new();
        let mut t = bae_rs::SamplerackT::new();

        for i in 0..8 * SAMPLE_RATE {
            lp.set_resonance(i as bae_rs::Math / (8 * SAMPLE_RATE) as bae_rs::Math);

            t.push(lp.process(n.process()));
        }

        let f = ".junk/modifiers/lowpass.wav";
        WaveWriteOptions::new()
            .bps(24)?
            .r(SAMPLE_RATE as Math)
            .clip(true)
            .write(vec![t], &mut File::create(f).unwrap())
            .unwrap();

        Ok(())
    }

    fn run_modifier(m: &mut dyn bae_rs::modifiers::Modifier, file: &str) -> Result<(), ()> {
        let mut g = Noise::new();
        let mut t = bae_rs::SamplerackT::new();

        let before = std::time::Instant::now();

        for _ in 0..SAMPLE_RATE {
            t.push(m.process(g.process()));
        }

        let after = std::time::Instant::now();
        let duration = after - before;
        println!(
            "Test generated 1s of audio in {} seconds",
            duration.as_secs_f32()
        );

        let mut f = String::from(FILE_PREFIX);
        f.push_str(file);

        WaveWriteOptions::new()
            .bps(24)?
            .r(SAMPLE_RATE as Math)
            .clip(true)
            .write(vec![t], &mut File::create(f.as_str()).unwrap())
            .unwrap();

        Ok(())
    }
}
