// http://mshang.ca/2014/10/20/starfield.html

use std::f64::consts;

use fnv::FnvHasher;
use std::hash::Hasher;

fn hash(value: &str) -> u64 {
    let mut hash = FnvHasher::default();
    hash.write(value.as_bytes());

    hash.finish()
}

fn jitter(a: f64, b: f64, k: f64, m: f64, n: f64 ) -> f64 {

    let s = format!("{}:{}:{}", k, m, n);

    let frac: f64 = hash(&s) as f64 / (::std::u64::MAX >> 1) as f64;

    a + frac
}


pub fn get_stars(x: f64, y: f64, w: f64, h: f64, jit: bool) -> Vec<(f64, f64, f64)> {
    let mut stars = Vec::new();

    let k_cont = -(w * h).ln() / 2.0;
    let k_min = k_cont.floor();

    cfor!{let mut k = k_min; k < k_min + 5.0; k += 1.0; {
        let period = (-k).exp();

        cfor!{let mut m = (x / period).floor(); m <= ((x + w) / period).ceil(); m += 1.0; {

            cfor!{let mut n = (y / period).floor(); n <= ((y + h) / period).ceil(); n += 1.0; {

                let brightness: f64 = (10.0 * (k_cont - k).exp()).atan() * 2.0 / consts::PI;
                let s = match jit {
                    true => ( jitter(m, x, k, m, n) * period,
                              jitter(n, y, k, m, n) * period,
                              brightness
                            ),
                    false => ( m * period,
                               n * period,
                               brightness
                             ),
                };
                stars.push(s);
            }}

        }}
    }}

    stars
}
