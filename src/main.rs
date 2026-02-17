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
                let frag_coord = Vec2::new(w as f64, (HEIGHT - h - 1) as f64);
                let color = shader_main(frag_coord, time);

                let r = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
                let g = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
                let b = (color.z.clamp(0.0, 1.0) * 255.0) as u8;

                writer.write_all(&[r, g, b])?;
            }
        }
    }
    Ok(())
}

fn shader_main(frag_coord: Vec2, i_time: f64) -> Vec4 {
    let i_resolution = Vec2::new(WIDTH as f64, HEIGHT as f64);

    let i = frag_coord;
    let mut o = Vec4::splat(0.0);

    let v_res = i_resolution;

    let p = (i + i - v_res) / v_res.y / 0.3;

    let mut layer = 0.0;

    while layer < 9.0 {
        layer += 1.0;

        let mut v = p;
        let mut f = 0.0;
        while f < 9.0 {
            f += 1.0;
            let yx_scaled = v.yx() * f;
            let with_offset = yx_scaled + layer + i_time;
            v += sin_vec2(with_offset) / f;
        }

        o += (cos_vec4(Vec4::new(layer, layer + 1.0, layer + 2.0, layer + 3.0)) + Vec4::splat(1.0))
            / 6.0
            / v.length();
    }

    o = (o * o).tanh();

    o
}
