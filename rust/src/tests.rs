
#[test]
fn bk_zip() {
    let in_vec : [u8; 20] = [0; 20];
    let out_vec = super::bk::zip(&in_vec);
    assert_eq!(out_vec, [0x11, 0x72, 0x00, 0x00, 0x00, 0x14, 0x63, 0x60, 0xC0, 0x04, 0x00, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
}

#[test]
fn bk_unzip() {
    let in_vec : Vec<u8> = vec![0x11, 0x72, 0x00, 0x00, 0x00, 0x14, 0x63, 0x60, 0xC0, 0x04, 0x00, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    let out_vec = super::bk::unzip(&in_vec);
    assert_eq!(out_vec, [0;20]);
}

#[test]
fn deflate() {
    let in_vec : [u8; 20] = [0; 20];
    let out_vec = super::deflate(&in_vec);
    assert_eq!(out_vec, [0x63, 0x60, 0xC0, 0x04, 0x00]);
}

#[test]
fn inflate() {
    let in_vec : [u8; 5] = [0x63, 0x60, 0xC0, 0x04, 0x00];
    let out_vec = super::inflate(&in_vec);
    assert_eq!(out_vec, [0;20]);
}

#[test]
fn zip() {
    let in_vec : [u8; 20] = [0; 20];
    let out_vec = super::zip(&in_vec);
    assert_eq!(out_vec, [0x1F, 0x8B, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x63, 0x60, 0xC0, 0x04, 0x00, 0x8D, 0x9B, 0xD5, 0xF, 0x14, 0x00, 0x00, 0x00]);
}

#[test]
fn unzip() {
    let in_vec : Vec<u8> = vec![0x1F, 0x8B, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x63, 0x60, 0xC0, 0x04, 0x00, 0x8D, 0x9B, 0xD5, 0xF, 0x14, 0x00, 0x00, 0x00];
    let out_vec = super::unzip(&in_vec);
    assert_eq!(out_vec, [0; 20]);
}
