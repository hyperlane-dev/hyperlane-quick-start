use crate::{once_cell::sync::Lazy, *};

static LOGO_IMG: Lazy<Vec<u8>> = Lazy::new(|| {
    let data: Vec<u8> = read_from_file("./static/img/logo.png").unwrap();
    data
});

pub fn get_logo_img() -> Vec<u8> {
    LOGO_IMG.clone()
}
