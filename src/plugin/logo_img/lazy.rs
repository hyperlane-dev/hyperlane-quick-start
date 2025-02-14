use crate::{once_cell::sync::Lazy, *};

pub static LOGO_IMG: Lazy<Vec<u8>> = Lazy::new(|| {
    let data: Vec<u8> = read_from_file("./static/img/logo.png").unwrap();
    data
});
