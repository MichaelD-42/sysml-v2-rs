//! `VariantUsageElement` admits a full usage, so a variant member may carry a
//! short name and a name after its kind keywords: `variant part <'PC-2'> radar;`.

use sysml_syntax::parse;

const ADMITTED: &[&str] = &[
    "variation part def S { variant part <'PC-002'> radar; }",
    "variation part def S { variant part <'PC-002'> radar : Radar; }",
    "variation part def S { variant action <'F-9'> a; }",
    "variation part def S { variant #Electrical part <'PC-3'> ecu; }",
    "variation part def S { variant abstract part <'PC-4'> x; }",
    "variation part def S { variant part radar; }",
    "variation part def S { variant x; }",
];

#[test]
fn a_variant_member_takes_an_id_after_its_kind() {
    for shape in ADMITTED {
        let text = format!("package P {{ metadata def Electrical; part def Radar; {shape} }}\n");
        let parsed = parse(&text);
        assert!(parsed.ok(), "{shape}\n  {:?}", parsed.errors());
    }
}
