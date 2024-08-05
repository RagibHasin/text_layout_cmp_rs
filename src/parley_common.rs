use parley::style::{FontStack, StyleProperty};
use parley::{
    layout::{Alignment, Layout},
    style::FontFamily,
};
use parley::{FontContext, LayoutContext};
use peniko::Color;

pub fn parley_layout(
    font_cx: &mut FontContext,
    layout_cx: &mut LayoutContext<Color>,
    text: &str,
    font: &str,
    text_color: Color,
) -> Layout<Color> {
    // The text we are going to style and lay out
    // let text = String::from(
    //     "Some text here. Let's make it a bit longer so that line wrapping kicks in 😊. And also some اللغة العربية arabic text.",
    // );

    // The display scale for HiDPI rendering
    let display_scale = 1.0;

    // The width for line wrapping
    // let max_advance = Some(320.0 * display_scale);
    let max_advance = None;

    // Create a RangedBuilder
    let mut builder = layout_cx.ranged_builder(font_cx, text, display_scale);

    // Set default text colour styles (set foreground text color)
    let brush_style = StyleProperty::Brush(text_color);
    builder.push_default(&brush_style);

    // Set default font family
    let font_stack = FontStack::Single(FontFamily::Named(font));
    let font_stack_style = StyleProperty::FontStack(font_stack);
    builder.push_default(&font_stack_style);
    builder.push_default(&StyleProperty::LineHeight(1.3));
    builder.push_default(&StyleProperty::FontSize(24.0));

    // Build the builder into a Layout
    let mut layout: Layout<Color> = builder.build();

    // Perform layout (including bidi resolution and shaping) with start alignment
    layout.break_all_lines(max_advance);
    layout.align(max_advance, Alignment::Start);

    layout
}

pub fn init() -> (FontContext, LayoutContext<Color>, String) {
    // Create a FontContext, LayoutContext and ScaleContext
    //
    // These are all intended to be constructed rarely (perhaps even once per app (or once per thread))
    // and provide caches and scratch space to avoid allocations
    let mut font_cx = FontContext::default();
    let layout_cx = LayoutContext::new();

    let (font_id, _) = font_cx
        .collection
        .register_fonts(std::fs::read(r"assets\iosevka-ahad-regular.ttf").unwrap())
        .pop()
        .unwrap();
    let family_name = font_cx.collection.family_name(font_id).unwrap().to_string();

    (font_cx, layout_cx, family_name)
}
