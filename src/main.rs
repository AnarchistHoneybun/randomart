use random_art::generate_image;

fn main() {
    let (image, expression) = generate_image("spiderman", 600, 600, 12);
    println!("Generated expressions:\n{}", expression);
    image.save("output.png").unwrap();
}
