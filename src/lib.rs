mod window;

use window::window_builder;

#[test]
fn test_window()    {
    let mut window = window_builder::Window::new(
        "test",
        window_builder::Size    {
            height: 100,
            width: 100,
        }
    );

    window.show();
}
