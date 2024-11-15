use random_art::generate_image;

fn main() {
    let (image, expression) = generate_image("SecretKey", 600, 600, 30);
    println!("Generated expressions:\n{}", expression);
    image.save("output.png").unwrap();
}
