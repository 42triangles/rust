use rustc_span::hygiene::Transparency;

use super::prelude::*;

pub(crate) struct RustcMacroTransparencyParser;

impl SingleAttributeParser for RustcMacroTransparencyParser {
    const PATH: &[Symbol] = &[sym::rustc_macro_transparency];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::Custom(|cx, used, unused| {
        cx.dcx().span_err(vec![used, unused], "multiple macro transparency attributes");
    });
    const ALLOWED_TARGETS: AllowedTargets = AllowedTargets::AllowList(&[Allow(Target::MacroDef)]);
    const TEMPLATE: AttributeTemplate =
        template!(NameValueStr: ["transparent", "semiopaque", "opaque"]);

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        let nv = cx.expect_name_value(args, cx.attr_span, None)?;
        cx.expect_mapped_symbol_strings(
            nv.value_as_str(),
            nv.value_span,
            [
                (sym::transparent, Transparency::Transparent),
                (sym::semiopaque, Transparency::SemiOpaque),
                (sym::opaque, Transparency::Opaque),
            ],
        )
        .map(AttributeKind::RustcMacroTransparency)
    }
}
