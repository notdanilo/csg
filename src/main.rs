

fn main() {
    let display = gpu::ContextDisplay::Window("Constructive Solid Geometry".to_string(), 800, 600);
    let mut context = gpu::ContextBuilder::new().with_display(display).build();
    context.make_current();
    while context.run() {
    }
}
