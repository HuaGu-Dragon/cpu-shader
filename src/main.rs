use std::io::Write;

use anyhow::Context;

const FACTOR: usize = 60;
const WIDTH: usize = 16 * FACTOR;
const HEIGHT: usize = 9 * FACTOR;
const MAX_COLOR_VALUE: usize = 255;

fn main() -> anyhow::Result<()> {
    let mut f = std::fs::File::create("output.ppm").context("create output.ppm")?;

    writeln!(f, "P6").context("write ppm format")?;
    writeln!(f, "{} {}", WIDTH, HEIGHT).context("write image's width & height")?;
    writeln!(f, "{}", MAX_COLOR_VALUE).context("write max color value")?;

    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            f.write_all(&[0xff, 0, 0])?;
        }
    }

    Ok(())
}
