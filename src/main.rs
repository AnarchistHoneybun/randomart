use random_art::generate_image;

fn main() {
    let (image, _expression) = generate_image("SecretKey", 600, 600, 18);
    // println!("Generated expressions:\n{}", expression);
    image.save("output.png").unwrap();
}
