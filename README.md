# RandomArt


## Usage

To generate an image, use the `generate_image` function. It requires a seed, width, height, and depth as parameters.

```rust
use image_generator::generate_image;

let (img, expr) = generate_image("example seed", 800, 600, 5);
img.save("output.png").unwrap();
println!("{}", expr);
```

## Examples
<img src="assets/render_1.png" alt="Render 1"></img>
<img src="assets/render_2.png" alt="Render 2"></img>
<img src="assets/render_3.png" alt="Render 3"></img>
<img src="assets/render_4.png" alt="Render 4"></img>