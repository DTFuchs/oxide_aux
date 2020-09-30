mod jpeg2000;
mod image;

#[cfg(test)]
mod tests {
    use crate::image::{RawImageData, IsImageDecoder, Loadable};
    use std::path::Path;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn image_decoding_j2k() {
        let raw_image= RawImageData::load(String::from("/home/dylancannisi/Downloads/relax.jp2")).expect("J2K not found");
        let decoded = jp2k::ImageBuffer::decode(&raw_image).expect("Unable to decode J2K");

        image::save_buffer(&Path::new("image.png"), &*decoded.buffer, decoded.width, decoded.height, image::ColorType::Rgb8);
        // for y in 1..decoded.height {
        //     for x in 1..decoded.width {
        //         if decoded.buffer[(y*decoded.width+(x*decoded.num_bands as u32)) as usize] < 128 {
        //             print!("X");
        //         } else {
        //             print!("O");
        //         }
        //     }
        //     print!("\n")
        // }


        assert!(true);
    }
}
