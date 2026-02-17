use std::io::{BufWriter, Write};

use anyhow::Context;

const FACTOR: usize = 60;
const WIDTH: usize = 16 * FACTOR;
const HEIGHT: usize = 9 * FACTOR;
const MAX_COLOR_VALUE: usize = 255;

fn main() -> anyhow::Result<()> {
    for i in 0..60 {
        let f = std::fs::File::create(format!("assets/output-{:02}.ppm", i))
            .context("create output.ppm")?;
        let mut writer = BufWriter::new(f);

        writeln!(writer, "P6").context("write ppm format")?;
        writeln!(writer, "{} {}", WIDTH, HEIGHT).context("write image's width & height")?;
        writeln!(writer, "{}", MAX_COLOR_VALUE).context("write max color value")?;

        for h in 0..HEIGHT {
            for w in 0..WIDTH {
                writer.write_all(&[0xff, 0, 0])?;
            }
        }
    }
    Ok(())
}
