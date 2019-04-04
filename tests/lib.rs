use pixelate::{Color, Image};

#[test]
fn renders_pixelate_logo() {
    let palette = &[
        Color::Rgba(255, 255, 255, 0),
        Color::Rgb(0, 0, 0),
        Color::Rgba(0, 0, 0, 80),
    ];

    const __: u8 = 0;
    const XX: u8 = 1;

    let ss = 2;

    let pixels = &[
        XX,XX,XX,XX,XX,__,XX,XX,XX,__,XX,__,__,__,XX,__,XX,XX,XX,XX,XX,__,XX,__,__,__,__,__,XX,XX,XX,XX,XX,__,XX,XX,XX,XX,XX,__,XX,XX,XX,XX,XX,
        XX,ss,ss,ss,XX,__,ss,XX,ss,__,XX,__,__,__,XX,__,XX,ss,ss,ss,ss,__,XX,__,__,__,__,__,XX,ss,ss,ss,XX,__,ss,ss,XX,ss,ss,__,XX,ss,ss,ss,ss,
        XX,__,__,__,XX,__,__,XX,__,__,ss,XX,__,XX,ss,__,XX,__,__,__,__,__,XX,__,__,__,__,__,XX,__,__,__,XX,__,__,__,XX,__,__,__,XX,__,__,__,__,
        XX,XX,XX,XX,XX,__,__,XX,__,__,__,ss,XX,ss,__,__,XX,XX,XX,XX,XX,__,XX,__,__,__,__,__,XX,XX,XX,XX,XX,__,__,__,XX,__,__,__,XX,XX,XX,XX,XX,
        XX,ss,ss,ss,ss,__,__,XX,__,__,__,XX,ss,XX,__,__,XX,ss,ss,ss,ss,__,XX,__,__,__,__,__,XX,ss,ss,ss,XX,__,__,__,XX,__,__,__,XX,ss,ss,ss,ss,
        XX,__,__,__,__,__,__,XX,__,__,XX,ss,__,ss,XX,__,XX,__,__,__,__,__,XX,__,__,__,__,__,XX,__,__,__,XX,__,__,__,XX,__,__,__,XX,__,__,__,__,
        XX,__,__,__,__,__,XX,XX,XX,__,XX,__,__,__,XX,__,XX,XX,XX,XX,XX,__,XX,XX,XX,XX,XX,__,XX,__,__,__,XX,__,__,__,XX,__,__,__,XX,XX,XX,XX,XX,
        ss,__,__,__,__,__,ss,ss,ss,__,ss,__,__,__,ss,__,ss,ss,ss,ss,ss,__,ss,ss,ss,ss,ss,__,ss,__,__,__,ss,__,__,__,ss,__,__,__,ss,ss,ss,ss,ss,
    ];

    let image = Image {
        palette,
        pixels,
        width: 45,
        scale: 16,
    };

    let expected = include_bytes!("../pixelate.png");

    let mut actual = Vec::new();

    image.render(&mut actual).unwrap();

    assert_eq!(&expected[..], &actual[..]);
}
