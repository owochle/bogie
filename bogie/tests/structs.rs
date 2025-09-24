use std::fmt::Formatter;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use bogie_derive::Debogue;

fn b64_fmt(value: &[u8], fmt: &mut Formatter<'_>) -> core::fmt::Result {
    write!(fmt, "{:?}", BASE64_STANDARD.encode(value))
}

#[derive(Debogue)]
#[bogie(Hex, pub_only)]
struct TestStruct {
    pub a: usize,

    #[bogie(skip)]
    pub b: usize,

    #[bogie(bin)]
    pub c: usize,

    #[bogie(hex)]
    pub d: usize,

    #[bogie(fn = "b64_fmt")]
    pub e: [u8; 16],

    #[bogie(empty)]
    pub f: usize,

    g: usize,

    #[bogie(dbg)]
    pub h: usize
}

#[derive(Debogue)]
enum TestEnum {
    UnitVariant,
    TupleVariant(usize, usize),
    NamedVariant {
        #[bogie(hex)]
        a: usize,
        b: usize
    }
}

#[derive(Debogue)]
struct TestEmpty {

}

#[test]
pub fn format_test() {
    let mut out = [0; 16];
    let _ = BASE64_STANDARD.decode_slice("1OZkZxkY05EVJwimPAM1AA==", &mut out);

    let structure = TestStruct{ a: 31, b: 31, c: 2, d: 31, e: out, f: 45, g: 1, h: 241 };

    assert_eq!("NamedVariant { a: 1f, b: 2 }", format!("{:?}", TestEnum::NamedVariant {
        a: 31, b: 2
    }));
    assert_eq!("TestEmpty", format!("{:?}", TestEmpty{}));
    assert_eq!(r#"TestStruct { a: 1F, c: 10, d: 1f, e: "1OZkZxkY05EVJwimPAM1AA==", f: (), h: 241 }"#, format!("{:?}", structure));
}