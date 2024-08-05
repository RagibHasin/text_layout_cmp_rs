mod cosmic_zeno;
mod parley_skia;
mod parley_zeno;

pub mod parley_common;

const TEXT: &str = r##" !"#$%&'()*+,-./
0123456789:;<=>
?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^
_`abcdefghijklmnopqrstuvwxyz{|}~
αβγδεζηθικλμνξοπρςστυφχψω
ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡ ΣΤΥΦΧΨΩ"##;

fn main() {
    cosmic_zeno::main(TEXT, false);
    cosmic_zeno::main(TEXT, true);

    let (mut font_cx, mut layout_cx, font) = parley_common::init();

    parley_skia::main(&mut font_cx, &mut layout_cx, &font, TEXT, false);
    parley_zeno::main(&mut font_cx, &mut layout_cx, &font, TEXT, false);

    parley_skia::main(&mut font_cx, &mut layout_cx, &font, TEXT, true);
    parley_zeno::main(&mut font_cx, &mut layout_cx, &font, TEXT, true);
}
