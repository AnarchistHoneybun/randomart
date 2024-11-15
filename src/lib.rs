use image::{ImageBuffer, Rgb};
use rand::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::hash::Hasher;

#[derive(Debug, Clone)]
enum Node {
    X,
    Y,
    T,
    Number(f32),
    Add(Box<Node>, Box<Node>),
    Mult(Box<Node>, Box<Node>),
    Mod(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Sqrt(Box<Node>),
    Sin(Box<Node>),
    Cos(Box<Node>),
    Average(Box<Node>, Box<Node>),
    Mix(Box<Node>, Box<Node>, Box<Node>, Box<Node>),
}

impl Node {
    fn eval(&self, x: f32, y: f32, t: f32) -> f32 {
        match self {
            Node::X => x,
            Node::Y => y,
            Node::T => t,
            Node::Number(n) => *n,
            Node::Add(a, b) => (a.eval(x, y, t) + b.eval(x, y, t)) / 2.0,
            Node::Mult(a, b) => a.eval(x, y, t) * b.eval(x, y, t),
            Node::Mod(a, b) => {
                let b_val = b.eval(x, y, t);
                if b_val == 0.0 {
                    0.0
                } else {
                    a.eval(x, y, t) % b_val
                }
            }
            Node::Div(a, b) => {
                let b_val = b.eval(x, y, t);
                if b_val.abs() < 1e-6 {
                    0.0
                } else {
                    a.eval(x, y, t) / b_val
                }
            }
            Node::Sqrt(a) => {
                let val = a.eval(x, y, t);
                if val < 0.0 {
                    0.0
                } else {
                    val.sqrt()
                }
            }
            Node::Sin(a) => a.eval(x, y, t).sin(),
            Node::Cos(a) => a.eval(x, y, t).cos(),
            Node::Average(a, b) => (a.eval(x, y, t) + b.eval(x, y, t)) / 2.0,
            Node::Mix(a, b, c, d) => {
                let a_val = a.eval(x, y, t);
                let b_val = b.eval(x, y, t);
                let c_val = c.eval(x, y, t);
                let d_val = d.eval(x, y, t);
                (a_val * c_val + b_val * d_val) / (a_val + b_val + 1e-6)
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Node::X => "x".to_string(),
            Node::Y => "y".to_string(),
            Node::T => "t".to_string(),
            Node::Number(n) => format!("{}", n),
            Node::Add(a, b) => format!("add({}, {})", a.to_string(), b.to_string()),
            Node::Mult(a, b) => format!("mult({}, {})", a.to_string(), b.to_string()),
            Node::Mod(a, b) => format!("mod({}, {})", a.to_string(), b.to_string()),
            Node::Div(a, b) => format!("div({}, {})", a.to_string(), b.to_string()),
            Node::Sqrt(a) => format!("sqrt({})", a.to_string()),
            Node::Sin(a) => format!("sin({})", a.to_string()),
            Node::Cos(a) => format!("cos({})", a.to_string()),
            Node::Average(a, b) => format!("avg({}, {})", a.to_string(), b.to_string()),
            Node::Mix(a, b, c, d) => format!(
                "mix({}, {}, {}, {})",
                a.to_string(),
                b.to_string(),
                c.to_string(),
                d.to_string()
            ),
        }
    }
}

struct ExpressionGenerator {
    rng: ChaCha8Rng,
}

impl ExpressionGenerator {
    fn new(seed: &str) -> Self {
        let seed_bytes = seed.as_bytes();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed_bytes, &mut hasher);
        let seed_val = hasher.finish();

        ExpressionGenerator {
            rng: ChaCha8Rng::seed_from_u64(seed_val),
        }
    }

    fn gen_expr(&mut self, depth: i32) -> Node {
        if depth <= 0 || self.rng.gen::<f32>() < 0.1 {
            match self.rng.gen_range(0..=3) {
                0 => Node::X,
                1 => Node::Y,
                2 => Node::T,
                _ => Node::Number(self.rng.gen_range(-1.0..=1.0)),
            }
        } else {
            match self.rng.gen_range(0..=9) {
                0 => Node::Add(
                    Box::new(self.gen_expr(depth - 1)),
                    Box::new(self.gen_expr(depth - 1)),
                ),
                1 => Node::Mult(
                    Box::new(self.gen_expr(depth - 1)),
                    Box::new(self.gen_expr(depth - 1)),
                ),
                2 => Node::Mod(
                    Box::new(self.gen_expr(depth - 1)),
                    Box::new(self.gen_expr(depth - 1)),
                ),
                3 => Node::Div(
                    Box::new(self.gen_expr(depth - 1)),
                    Box::new(self.gen_expr(depth - 1)),
                ),
                4 => Node::Sqrt(Box::new(self.gen_expr(depth - 1))),
                5 => Node::Sin(Box::new(self.gen_expr(depth - 1))),
                6 => Node::Cos(Box::new(self.gen_expr(depth - 1))),
                7 => Node::Average(
                    Box::new(self.gen_expr(depth - 1)),
                    Box::new(self.gen_expr(depth - 1)),
                ),
                8 => Node::Mix(
                    Box::new(self.gen_expr(depth - 1)),
                    Box::new(self.gen_expr(depth - 1)),
                    Box::new(self.gen_expr(depth - 1)),
                    Box::new(self.gen_expr(depth - 1)),
                ),
                _ => Node::X,
            }
        }
    }
}

pub fn generate_image(
    seed: &str,
    width: u32,
    height: u32,
    depth: i32,
) -> (ImageBuffer<Rgb<u8>, Vec<u8>>, String) {
    let mut gen = ExpressionGenerator::new(seed);

    // Generate three expressions for R, G, B channels
    let expr_r = gen.gen_expr(depth);
    let expr_g = gen.gen_expr(depth);
    let expr_b = gen.gen_expr(depth);

    // Create expression string
    let expr_string = format!(
        "R: {}\nG: {}\nB: {}",
        expr_r.to_string(),
        expr_g.to_string(),
        expr_b.to_string()
    );

    // Create image buffer
    let mut imgbuf = ImageBuffer::new(width, height);

    // Generate pixel values
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let nx = (x as f32 / width as f32) * 2.0 - 1.0;
        let ny = (y as f32 / height as f32) * 2.0 - 1.0;

        // Evaluate each channel
        let r = expr_r.eval(nx, ny, 0.0);
        let g = expr_g.eval(nx, ny, 0.0);
        let b = expr_b.eval(nx, ny, 0.0);

        // Convert to 0-255 range and clamp
        let to_pixel = |v: f32| -> u8 {
            let scaled = ((v + 1.0) / 2.0 * 255.0).round();
            scaled.clamp(0.0, 255.0) as u8
        };

        *pixel = Rgb([to_pixel(r), to_pixel(g), to_pixel(b)]);
    }

    (imgbuf, expr_string)
}

// Required for tests and benchmarks
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_generation() {
        let (img, expr) = generate_image("test seed", 100, 100, 5);
        assert_eq!(img.dimensions(), (100, 100));
        assert!(expr.contains("R:"));
        assert!(expr.contains("G:"));
        assert!(expr.contains("B:"));
    }

    #[test]
    fn test_deterministic() {
        let (img1, expr1) = generate_image("same seed", 10, 10, 5);
        let (img2, expr2) = generate_image("same seed", 10, 10, 5);
        assert_eq!(expr1, expr2);

        // Compare all pixels
        for (p1, p2) in img1.pixels().zip(img2.pixels()) {
            assert_eq!(p1, p2);
        }
    }
}
