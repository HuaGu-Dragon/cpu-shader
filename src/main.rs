use anyhow::Context;
use cpu_shader::vec::{
    vec2::{Vec2, sin_vec2},
    vec4::{Vec4, cos_vec4},
};
use std::io::{BufWriter, Write};

const FACTOR: usize = 60;
const WIDTH: usize = 16 * FACTOR;
const HEIGHT: usize = 9 * FACTOR;
const MAX_COLOR_VALUE: usize = 255;

fn main() -> anyhow::Result<()> {
    for frame in 0..60 {
        let f = std::fs::File::create(format!("assets/output-{:02}.ppm", frame))
            .context("create output.ppm")?;
        let mut writer = BufWriter::new(f);

        writeln!(writer, "P6").context("write ppm format")?;
        writeln!(writer, "{} {}", WIDTH, HEIGHT).context("write image's width & height")?;
        writeln!(writer, "{}", MAX_COLOR_VALUE).context("write max color value")?;

        let time = frame as f64 / 60.0;

        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                writer.write_all(&[0xff, 0, 0])?;
            }
        }
    }
    Ok(())
}
