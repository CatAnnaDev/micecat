
pub fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn grad(hash: u8, x: f32, y: f32) -> f32 {
    match hash & 0b11 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        _ => -x - y,
    }
}

pub const PERM: [u8; 512] = {
    let p = [
        151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,
        140,36,103,30,69,142,8,99,37,240,21,10,23,190, 6,148,
        247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,
        57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,
        74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,
        60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,
        65,25,63,161, 1,216,80,73,209,76,132,187,208,89,18,169,
        200,196,135,130,116,188,159,86,164,100,109,198,173,186, 3,64,
        52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,
        207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,
        119,248,152, 2,44,154,163,70,221,153,101,155,167, 43,172,9,
        129,22,39,253, 19,98,108,110,79,113,224,232,178,185,112,104,
        218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,
        81,51,145,235,249,14,239,107,49,192,214, 31,181,199,106,157,
        184, 84,204,176,115,121,50,45,127, 4,150,254,138,236,205,93,
        222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180
    ];
    let mut table = [0u8; 512];
    let mut i = 0;
    while i < 256 {
        table[i] = p[i];
        table[i + 256] = p[i];
        i += 1;
    }
    table
};

pub fn perlin(x: f32, y: f32) -> f32 {
    let xi = x.floor() as usize & 255;
    let yi = y.floor() as usize & 255;

    let xf = x - x.floor();
    let yf = y - y.floor();

    let u = fade(xf);
    let v = fade(yf);

    let aa = PERM[PERM[xi] as usize + yi];
    let ab = PERM[PERM[xi] as usize + yi + 1];
    let ba = PERM[PERM[xi + 1] as usize + yi];
    let bb = PERM[PERM[xi + 1] as usize + yi + 1];

    let x1 = lerp(
        grad(aa, xf, yf),
        grad(ba, xf - 1.0, yf),
        u,
    );
    let x2 = lerp(
        grad(ab, xf, yf - 1.0),
        grad(bb, xf - 1.0, yf - 1.0),
        u,
    );

    // Résultat entre -1 et 1
    (lerp(x1, x2, v) + 1.0) / 2.0
}

pub fn perlin2d(x: f32, y: f32) -> f32 {
    use std::f32;
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let xf = x - x0 as f32;
    let yf = y - y0 as f32;

    let grad = |ix: i32, iy: i32| {
        let hash = (ix * 374761393 + iy * 668265263) as u32;
        let h = hash % 4;
        match h {
            0 => (1.0, 1.0),
            1 => (-1.0, 1.0),
            2 => (1.0, -1.0),
            _ => (-1.0, -1.0),
        }
    };

    let fade = |t: f32| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    let lerp = |a: f32, b: f32, t: f32| a + t * (b - a);

    let (g00x, g00y) = grad(x0, y0);
    let (g10x, g10y) = grad(x0 + 1, y0);
    let (g01x, g01y) = grad(x0, y0 + 1);
    let (g11x, g11y) = grad(x0 + 1, y0 + 1);

    let dot00 = g00x * xf + g00y * yf;
    let dot10 = g10x * (xf - 1.0) + g10y * yf;
    let dot01 = g01x * xf + g01y * (yf - 1.0);
    let dot11 = g11x * (xf - 1.0) + g11y * (yf - 1.0);

    let u = fade(xf);
    let v = fade(yf);

    let nx0 = lerp(dot00, dot10, u);
    let nx1 = lerp(dot01, dot11, u);
    let nxy = lerp(nx0, nx1, v);

    nxy
}


/// Perlin bruit fractal (octaves)
pub fn perlin_octaves(x: f32, y: f32, octaves: u32, persistence: f32, scale: f32) -> f32 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        total += perlin(x * frequency / scale, y * frequency / scale) * amplitude;
        max_value += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }

    total / max_value
}
