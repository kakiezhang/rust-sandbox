use base64;
use base64::{
    alphabet,
    engine::{self, general_purpose, DecodePaddingMode},
    Engine as _,
};

fn main() {
    // encode0();
    decode0();
}

fn decode0() {
    // let mut s = String::from("jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C");
    let s = "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C=";
    // let s = "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2A=";

    // let padding_length = s.len() % 4;
    // let s = if padding_length > 0 {
    //     &s[..s.len() - padding_length]
    // } else {
    //     s
    // };

    // s.push('=');
    // println!("trimed: {:?}", s);

    // let bytes = general_purpose::STANDARD.decode(s).unwrap();

    let bytes = engine::GeneralPurpose::new(
        &alphabet::STANDARD,
        general_purpose::PAD.with_decode_allow_trailing_bits(true),
    )
    .decode(s)
    .unwrap();

    println!("{:?}", bytes);
}

fn encode0() {
    let bs: Vec<u8> = vec![
        141, 105, 152, 155, 186, 171, 230, 115, 40, 1, 76, 25, 70, 49, 173, 7, 25, 179, 220, 160,
        53, 182, 64, 35, 223, 41, 36, 71, 170, 182, 7, 96,
    ];

    let b64 = general_purpose::STANDARD.encode(&bs);
    println!("{}", b64);

    const CUSTOM_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(
        &alphabet::STANDARD,
        general_purpose::PAD.with_decode_allow_trailing_bits(true),
    );
    let b64 = CUSTOM_ENGINE.encode(&bs);
    println!("{}", b64);
}
