fn get_n_diverged(x0: f64, y0: f64, max_iter: usize) -> u8 {
    let mut xn = 0.0;
    let mut yn = 0.0;
    for i in 1..max_iter {
        let x_next = xn * xn - yn * yn + x0;
        let y_next = 2.0 * xn * yn + y0;
        xn = x_next;
        yn = y_next;
        if yn * yn + xn * xn > 4.0 {
            return i as u8;
        }
    }
    max_iter as u8
}

pub fn generate_mandelbrot_set(
    canvas_w: usize,
    canvas_h: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iter: usize,
) -> Vec<u8> {
    let canvas_w_f64 = canvas_w as f64;
    let canvas_h_f64 = canvas_h as f64;

    let mut data = vec![];
    for i in 0..canvas_h {
        let y = y_min + (y_max - y_min) * (i as f64) / canvas_h_f64;
        for j in 0..canvas_w {
            let x = x_min + (x_max - x_min) * (j as f64) / canvas_w_f64;
            let iter_index = get_n_diverged(x, y, max_iter);
            let intensity = (iter_index as f64 / max_iter as f64 * 255.0) as u8;
            data.push(intensity);
            data.push(intensity);
            data.push(intensity);
            data.push(255);
        }
    }
    data
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_n_diverged() {
        let max_iter = 10;
        assert_eq!(get_n_diverged(1.0, 0.0, max_iter), 3);
        assert_eq!(get_n_diverged(0.0, 0.0, max_iter), max_iter as u8);
        assert_eq!(get_n_diverged(0.0, 1.0, max_iter), max_iter as u8);
    }

    #[test]
    fn test_generate_manedelbrot_set() {
        let canvas_w = 2;
        let canvas_h = 2;
        let x_min = -1.0;
        let x_max = 1.0;
        let y_min = -1.0;
        let y_max = 1.0;
        let max_iter = 8;
        let data = generate_mandelbrot_set(
            canvas_w,
            canvas_h,
            x_min,
            x_max,
            y_min,
            y_max,
            max_iter,
        );
        assert_eq!(data.len(), 4 * canvas_w * canvas_h);
        assert_eq!(data[0], 0);
        assert_eq!(data[1], 0);
        assert_eq!(data[2], 0);
        assert_eq!(data[3], 255);
        assert_eq!(data[4], 0);
        assert_eq!(data[5], 0);
        assert_eq!(data[6], 0);
        assert_eq!(data[7], 255);
        assert_eq!(data[8], 0);
        assert_eq!(data[9], 0);
        assert_eq!(data[10], 0);
        assert_eq!(data[11], 255);
        assert_eq!(data[12], 0);
        assert_eq!(data[13], 0);
        assert_eq!(data[14], 0);
        assert_eq!(data[15], 255);
    }
}
