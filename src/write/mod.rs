use crate::jabcode;
use encode::EncodeHandle;

mod encode;
mod error;
mod option;

pub use error::WriteError;
pub use option::WriteOptions;

pub fn write_jabcode(data: &[u8], options: &WriteOptions) -> Result<image::RgbaImage, WriteError> {
    let mut handle = EncodeHandle::new(options.color_number, options.symbol_number())?;
    let enc = handle.enc_mut();

    if options.module_size > 0 {
        enc.module_size = options.module_size;
    }
    if options.master_symbol_width > 0 {
        enc.master_symbol_width = options.master_symbol_width;
    }
    if options.master_symbol_height > 0 {
        enc.master_symbol_height = options.master_symbol_height;
    }

    // setup master symbol
    unsafe {
        *enc.symbol_ecc_levels = options.master.ecc_level.into();
        *enc.symbol_versions = options.master.version.into();
        *enc.symbol_positions = options.master.position;
    }

    // setup slave symbols
    for (i, slave) in options.slaves.iter().enumerate() {
        unsafe {
            let off = 1 + i as isize;
            *enc.symbol_ecc_levels.offset(off) = slave.ecc_level.into();
            *enc.symbol_versions.offset(off) = slave.version.into();
            *enc.symbol_positions.offset(off) = slave.position;
        }
    }

    let layout = std::alloc::Layout::from_size_align(
        std::mem::size_of::<jabcode::jab_int32>()
            + data.len() * std::mem::size_of::<jabcode::jab_char>(),
        std::cmp::max(
            std::mem::align_of::<jabcode::jab_int32>(),
            std::mem::align_of::<jabcode::jab_char>(),
        ),
    )
    .unwrap();
    let buf = unsafe { std::alloc::alloc(layout) as *mut jabcode::jab_data };
    unsafe {
        (*buf).length = data.len() as i32;
    }
    for (i, c) in data.iter().enumerate() {
        unsafe {
            *(*buf).data.as_mut_ptr().offset(i as isize) = std::mem::transmute_copy(c);
        }
    }

    let r = unsafe { jabcode::generateJABCode(enc, buf) };

    unsafe {
        std::alloc::dealloc(buf as *mut u8, layout);
    }

    if r != 0 {
        return Err(WriteError::Jab(r));
    }
    let output = unsafe {
        let jabcode::jab_bitmap {
            width,
            height,
            bits_per_pixel,
            channel_count,
            pixel,
            ..
        } = &*enc.bitmap;
        assert_eq!(*channel_count, 4);

        let byte_size = (width * height * bits_per_pixel) as usize / 8;
        let mut buf = vec![0; byte_size];
        buf.copy_from_slice(pixel.as_slice(byte_size));

        image::RgbaImage::from_raw(
            (*width).try_into().expect("Invalid width"),
            (*height).try_into().expect("Invalid height"),
            buf,
        )
        .unwrap()
    };

    Ok(output)
}
