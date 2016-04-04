// http://mshang.ca/2014/10/20/starfield.html

use std::f64::consts;

pub fn get_stars(x: f64, y: f64, w: f64, h: f64) -> Vec<(f64, f64, f64)> {
    let mut stars = Vec::new();

    let k_cont = -(w * h).ln() / 2.0;
    let k_min = k_cont.floor();

    cfor!{let mut k = k_min; k < k_min + 5.0; k += 1.0; {
        let period = (-k).exp();

        cfor!{let mut m = (x / period).floor(); m <= ((x + w) / period).ceil(); m += 1.0; {

            cfor!{let mut n = (y / period).floor(); n <= ((y + h) / period).ceil(); n += 1.0; {

                let brightness: f64 = (10.0 * (k_cont - k).exp()).atan() * 2.0 / consts::PI;
                stars.push((m * period, n * period, brightness));
            }}

        }}
    }}

    stars
/*
for (var k = k_min; k < k_min + 5; k++) {
    var period = Math.exp(-k);
    for (var m = Math.floor(offsetX / period); m <= Math.ceil((offsetX + width) / period); m++)
      for (var n = Math.floor(offsetY / period); n <= Math.ceil((offsetY + height) / period); n++)
        stars.push(new Star(
          m * period,
          n * period,
          Math.atan(10 * Math.exp(k_cont - k)) * 2 / Math.PI));
}
*/

}
