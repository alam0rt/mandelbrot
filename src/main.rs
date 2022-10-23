use num::complex::Complex;

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);

    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;

            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row); // meant to be all_rows?
    }
    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };

    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
	    let val = colour('█', from_val(column));
            line.push_str(val.as_str());
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot(2000, -2.0, 1.0, -1.0, 1.0, 110, 32);
    render_mandelbrot(mandelbrot);
}


fn colour(rune: char, colour: Colour) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", colour.r, colour.g, colour.b, rune)
}


struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

fn from_val(val: usize) -> Colour {
    match val {
	x if (0..2).contains(&x) => return Colour{r: 0, g: 0, b: 150},
	x if (3..5).contains(&x) => return Colour{r: 0, g: 0, b: 200},
	x if (6..10).contains(&x) => return Colour{r: 255, g: 255, b: 0},
	x if (11..30).contains(&x) => return Colour{r: 0, g: 255, b: 255},
	x if (31..100).contains(&x) => return Colour{r: 0, g: 255, b: 0},
	x if (101..200).contains(&x) => return Colour{r: 255, g: 255, b: 255},
	x if (201..400).contains(&x) => return Colour{r: 255, g: 255, b: 255},
	x if (401..700).contains(&x) => return Colour{r: 255, g: 255, b: 255},
	_ => return Colour{r: 0, g: 0, b: 50},
    };
}
