use std::error::Error;
use base64::Engine;
use crate::tink::load_and_encrypt;

mod tink;
mod raw;

fn main() -> anyhow::Result<()> {
    println!("start");
    let key_path = "src/config/keyset_siv.json";
    let test = load_and_encrypt("src/config/keyset_siv.json", "PRJA391P8");
    let test_ids =
        vec![
        ("PRJA391P8", "AQ3Lg6xuE_N_vXSVuOgmHGFpTrM0f_AOeIHn-t3P"),
        ("P4K606AGC", "AQ3Lg6yxY9rAHayoaUFLUndWGoksrGntEX63_the"),
        ("PUHZMV07U", "AQ3Lg6yw0dHBs72XOp43-WmOSLs3uLN8KfO1utFo"),
        ("PH2KRL2JM", "AQ3Lg6wbvU7yyuyheLQJJlAOOHfllpuRvWpLN-wi"),
        ("P12CU5D2C", "AQ3Lg6zeCmPfOOXiYaaYKFgtmdrSeWd4hQzn7gVG"),
        ("P4K1CKKPZ", "AQ3Lg6wASPxGCpXw3GpiiDDWhvBgt2qYFZzEgQ5U"),
        ("PVK0666T8", "AQ3Lg6xfkHDbSudQ7AvZVPQWwEbTxtbIW12ckxO4"),
        ("PDKYDRM4Y", "AQ3Lg6zh_6EpTG-M70-JtVT-cjptj5YA_Vm13DO-"),
        ("PDHTUKX47", "AQ3Lg6yRpnbn7-4tPNA7dPLL8BjUKd3mUL3Mqfmb"),
        ("P0KRKM80V", "AQ3Lg6xJZBorIRG_2OQRA0qpQFy7l53_9_tIjZoL"),
        ];
    println!("{:?}", test_ids[0].0);
    println!("{:?}", test);
    println!("{:?}", test_ids[0].1);

    for (localid, expected_b64) in &test_ids {
        let ct = load_and_encrypt(key_path, localid).unwrap();
        let expected = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(expected_b64).unwrap();
        assert_eq!(ct, expected, "FAILED for {}", localid);
        println!("OK {}", localid);
    }

    Ok(())
}
