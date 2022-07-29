#![no_main]

use img_hash::image::{ImageBuffer, Luma};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (u8, u8, u8, &[u8])| {
    let (width, height, options, img) = data;
    let _ = fuzz(width, height, options, img);
});

fn fuzz(width: u8, height: u8, options: u8, img: &[u8]) -> Option<()> {
    let filter = match options & 0b111 {
        0 => img_hash::FilterType::Nearest,
        1 => img_hash::FilterType::Triangle,
        2 => img_hash::FilterType::CatmullRom,
        3 => img_hash::FilterType::Gaussian,
        _ => img_hash::FilterType::Lanczos3,
    };

    let hash = match (options >> 3) & 0b111 {
        0 => img_hash::HashAlg::Mean,
        1 => img_hash::HashAlg::Gradient,
        2 => img_hash::HashAlg::VertGradient,
        3 => img_hash::HashAlg::DoubleGradient,
        _ => img_hash::HashAlg::Blockhash,
    };

    let preproc_dct = (options >> 6) == 1;
    let preproc_gauss = (options >> 7) == 1;

    let width = width as u32 + 1;
    let height = height as u32 + 1;

    let image: ImageBuffer<Luma<u8>, _> = ImageBuffer::from_vec(width, height, img.to_vec())?;

    let mut builder = img_hash::HasherConfig::new()
        .resize_filter(filter)
        .hash_alg(hash);

    if preproc_dct {
        builder = builder.preproc_dct();
    }

    if preproc_gauss {
        builder = builder.preproc_diff_gauss();
    }

    let hasher = builder.to_hasher();

    #[cfg(feature = "debug")]
    println!(
        "{width}x{height} = {}, actual: {}",
        width * height,
        img.len()
    );

    let _ = hasher.hash_image(&image);

    Some(())
}
