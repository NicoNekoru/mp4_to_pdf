// imports the `image` library with the exact version that we are using
use printpdf::*;

use std::convert::From;
use std::fs::File;

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // currently, the only reliable file formats are bmp/jpeg/png
    // this is an issue of the image library, not a fault of printpdf
    let mut image_file = File::open("assets/img/BMP_test.bmp").unwrap();
    let image = Image::try_from(image_crate::codecs::bmp::BmpDecoder::new(&mut image_file).unwrap()).unwrap();

    // translate x, translate y, rotate, scale x, scale y
    // by default, an image is optimized to 300 DPI (if scale is None)
    // rotations and translations are always in relation to the lower left corner
    image.add_to_layer(current_layer.clone(), ImageTransform::default());

    // you can also construct images manually from your data:
    let mut image_file_2 = ImageXObject {
        width: Px(200),
        height: Px(200),
        color_space: ColorSpace::Greyscale,
        bits_per_component: ColorBits::Bit8,
        interpolate: true,
        /* put your bytes here. Make sure the total number of bytes =
           width * height * (bytes per component * number of components)
           (e.g. 2 (bytes) x 3 (colors) for RGB 16bit) */
        image_data: Vec::new(),
        image_filter: None, /* does not work yet */
        clipping_bbox: None, /* doesn't work either, untested */
    };

    let image2 = Image::from(image_file_2);
}
