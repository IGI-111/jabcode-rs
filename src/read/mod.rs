use crate::jabcode;

mod error;

pub use error::ReadError;

const RGBA_CHANNELS: i32 = 4;

pub fn read_jabcode(image: &image::RgbaImage) -> Result<Vec<u8>, ReadError> {
    let image_buf = image.as_raw();
    let bits_per_pixel = 8 * image_buf.len() as i32 / (image.width() * image.height()) as i32;

    let layout = std::alloc::Layout::from_size_align(
        5 * std::mem::size_of::<jabcode::jab_int32>()
            + image_buf.len() * std::mem::size_of::<jabcode::jab_byte>(),
        std::cmp::max(
            std::mem::align_of::<jabcode::jab_int32>(),
            std::mem::align_of::<jabcode::jab_byte>(),
        ),
    )
    .unwrap();
    let bitmap = unsafe { std::alloc::alloc(layout) as *mut jabcode::jab_bitmap };
    unsafe {
        (*bitmap).width = image.width() as i32;
        (*bitmap).height = image.height() as i32;
        (*bitmap).bits_per_pixel = bits_per_pixel;
        (*bitmap).bits_per_channel = bits_per_pixel / RGBA_CHANNELS;
        (*bitmap).channel_count = RGBA_CHANNELS;
    }
    for (i, b) in image_buf.iter().enumerate() {
        unsafe {
            *(*bitmap).pixel.as_mut_ptr().offset(i as isize) = std::mem::transmute_copy(b);
        }
    }

    let mut decode_status: i32 = 0;

    let decoded = unsafe {
        jabcode::decodeJABCode(bitmap, jabcode::NORMAL_DECODE as i32, &mut decode_status)
    };

    unsafe {
        std::alloc::dealloc(bitmap as *mut u8, layout);
    }

    if decoded.is_null() {
        return Err(ReadError::Jab);
    };

    if decode_status == 2 {
        eprintln!("The code is only partly decoded. Some slave symbols have not been decoded and are ignored.");
    }

    let buf = unsafe {
        let data_bytes = std::mem::transmute::<
            &jabcode::__IncompleteArrayField<i8>,
            &jabcode::__IncompleteArrayField<u8>,
        >(&(*decoded).data);
        data_bytes.as_slice((*decoded).length as usize)
    };
    let mut res = vec![0; buf.len()];
    res.copy_from_slice(buf);

    unsafe {
        libc::free(decoded as *mut libc::c_void);
    }

    Ok(res)
}
