use crate::noise::{fade, lerp, PERM};

fn grad3(hash: u8, x: f32, y: f32, z: f32) -> f32 {
    match hash & 0b111 {
        0 =>  x + y,
        1 => -x + y,
        2 =>  x - y,
        3 => -x - y,
        4 =>  x + z,
        5 => -x + z,
        6 =>  x - z,
        _ => -x - z,
    }
}

pub fn perlin3d(x: f32, y: f32, z: f32) -> f32 {
    let xi = x.floor() as usize & 255;
    let yi = y.floor() as usize & 255;
    let zi = z.floor() as usize & 255;

    let xf = x - x.floor();
    let yf = y - y.floor();
    let zf = z - z.floor();

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let aaa = PERM[PERM[PERM[xi] as usize  + yi] as usize  + zi];
    let aba = PERM[PERM[PERM[xi] as usize  + yi + 1] as usize  + zi];
    let aab = PERM[PERM[PERM[xi] as usize  + yi] as usize  + zi + 1];
    let abb = PERM[PERM[PERM[xi] as usize  + yi + 1] as usize  + zi + 1];
    let baa = PERM[PERM[PERM[xi + 1] as usize  + yi] as usize + zi];
    let bba = PERM[PERM[PERM[xi + 1] as usize  + yi + 1] as usize  + zi];
    let bab = PERM[PERM[PERM[xi + 1] as usize  + yi] as usize  + zi + 1];
    let bbb = PERM[PERM[PERM[xi + 1] as usize  + yi + 1] as usize  + zi + 1];

    let x1 = lerp(
        grad3(aaa, xf, yf, zf),
        grad3(baa, xf - 1.0, yf, zf),
        u,
    );
    let x2 = lerp(
        grad3(aba, xf, yf - 1.0, zf),
        grad3(bba, xf - 1.0, yf - 1.0, zf),
        u,
    );
    let y1 = lerp(x1, x2, v);

    let x3 = lerp(
        grad3(aab, xf, yf, zf - 1.0),
        grad3(bab, xf - 1.0, yf, zf - 1.0),
        u,
    );
    let x4 = lerp(
        grad3(abb, xf, yf - 1.0, zf - 1.0),
        grad3(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
        u,
    );
    let y2 = lerp(x3, x4, v);

    (lerp(y1, y2, w) + 1.0) / 2.0
}

pub fn perlin3d_octaves(x: f32, y: f32, z: f32, octaves: u32, persistence: f32, scale: f32) -> f32 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max = 0.0;

    for _ in 0..octaves {
        total += perlin3d(x * frequency / scale, y * frequency / scale, z * frequency / scale) * amplitude;
        max += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }

    total / max
}
/*
let density = perlin3d(x as f32 * 0.1, y as f32 * 0.1, z as f32 * 0.1);
if density > 0.5 {

}
 */