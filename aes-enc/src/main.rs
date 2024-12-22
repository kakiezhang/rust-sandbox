use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyInit};
type Aes256CbcEnc = ecb::Encryptor<aes::Aes256>;
type Aes256CbcDec = ecb::Decryptor<aes::Aes256>;

// const KEY: &[u8; 16] = b"abcdedghijklmnop"; // 模拟密钥，请勿在实际程序中使用
const key: &[u8; 32] = b"ec3d170007120634a792c6c7b694e336";

/// 加密
pub fn encrypt(plain: &[u8]) -> Vec<u8> {
    // 随机值
    let mut buf = [0u8; 48];
    let pt_len = plain.len();
    buf[..pt_len].copy_from_slice(plain);
    let ct = Aes256CbcEnc::new(key.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(plain, &mut buf)
        .unwrap();

    ct.to_vec()
}

/// 解密
pub fn decrypt(cipher: &[u8]) -> Vec<u8> {
    let cipher_len = cipher.len();
    let mut buf = [0u8; 48];
    buf[..cipher_len].copy_from_slice(cipher);

    let pt = Aes256CbcDec::new(key.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(cipher, &mut buf)
        .unwrap();

    pt.to_vec()
}

fn main() {
    // 账号密码应为单向加密，参考：https://github.com/RustCrypto/password-hashes
    // 这里的示例代码应用来加密如手机号、身份证号、银行卡号等涉及用户隐私的数据

    let plain = b"admin";
    let ct = encrypt(plain);
    // let ct2 = ct.clone();
    println!("加密结果{:?}", ct);
    //加密结果Err(FromUtf8Error { bytes: [225, 51, 115, 113, 36, 252, 95, 73, 227, 166, 134, 32, 50, 140, 248, 43], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } })

    // let text = "4TNzcST8X0njpoYgMoz4Kw==".as_bytes().to_vec();
    let pt = decrypt(&ct);
    println!("解密结果：{:?}", String::from_utf8(pt));
    //解密结果：Ok("admin")
}
