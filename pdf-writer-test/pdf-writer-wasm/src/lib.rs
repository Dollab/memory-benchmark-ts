use wasm_bindgen::prelude::*;
use pdf_writer::types::{ActionType, AnnotationType, BorderType, LineCapStyle, LineJoinStyle};
use pdf_writer::*;
use image::{ColorType, GenericImageView, ImageFormat};
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, JsCast, Clamped};
use wasm_bindgen_futures::JsFuture;

// note 72pts per inch = 25.4 mm

const MM_PER_PT: f32 = 25.4/ 72.0;




#[wasm_bindgen]
pub struct WasmApp {
}

#[wasm_bindgen]
impl WasmApp {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self{
        WasmApp {
        }
    }

    async fn fetch_image_data(id: &str) -> Result<Vec<u8>, JsValue> {
        let window = web_sys::window().expect("Could not get a window"); // Browser window
        let document = window.document().expect("Could not get document");
        let image: web_sys::HtmlImageElement = document.get_element_by_id(id).expect("No element found w").dyn_into().unwrap();
        let promise = JsFuture::from(window.fetch_with_str(&image.src())); // File fetch promise
        let result = promise.await?; // Await fulfillment of fetch
        let response: web_sys::Response = result.dyn_into().unwrap(); // Type casting
        let image_data = JsFuture::from(response.array_buffer()?).await?; // Get text
        Ok(Uint8Array::new(&image_data).to_vec())
    }


    pub fn get_pdf(&mut self) -> Result<Vec<u8>, JsValue> {
        // Start writing.
        let mut writer = PdfWriter::new();

        // Define some indirect reference ids we'll use.
        let catalog_id = Ref::new(1);
        let page_tree_id = Ref::new(2);
        let page_id = Ref::new(3);
        let font_id = Ref::new(4);
        let content_id = Ref::new(5);
        let font_name = Name(b"F1");

        // Write the document catalog with a reference to the page tree.
        writer.catalog(catalog_id).pages(page_tree_id);

        // Write the page tree with a single child page.
        writer.pages(page_tree_id).kids([page_id]).count(1);

        // Write a page.
        let mut page = writer.page(page_id);

        // Set the size to A4 (measured in points) using `media_box` and set the
        // text object we'll write later as the page's contents.
        page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
        page.parent(page_tree_id);
        page.contents(content_id);

        // We also create the annotations list here that allows us to have things
        // like links or comments on the page.
        let mut annotations = page.annotations();
        let mut annotation = annotations.push();

        // Write the type, area, alt-text, and color for our link annotation.
        annotation.subtype(AnnotationType::Link);
        annotation.rect(Rect::new(215.0, 730.0, 251.0, 748.0));
        annotation.contents(TextStr("Link to the Rust project web page"));
        annotation.color_rgb(0.0, 0.0, 1.0);

        // Write an action for the annotation, telling it where to link to. Actions
        // can be associated with annotations, outline objects, and more and allow
        // creating interactive PDFs (open links, play sounds...).
        annotation
            .action()
            .action_type(ActionType::Uri)
            .uri(Str(b"https://www.rust-lang.org/"));

        // Set border and style for the link annotation.
        annotation.border_style().width(2.0).style(BorderType::Underline);

        // We have to finish all the writers that depend on the page here because
        // otherwise they would be mutably borrowed until the end of the block.
        // Finishing is handled through the writer's `Drop` implementations, so that
        // you cannot accidentally forget it. The `finish()` method from the `Finish`
        // trait is just a postfix-style version of dropping.
        annotation.finish();
        annotations.finish();

        // We also need to specify which resources the page needs, which in our case
        // is only a font that we name "F1" (the specific name doesn't matter).
        page.resources().fonts().pair(font_name, font_id);
        page.finish();

        // Specify the font we want to use. Because Helvetica is one of the 14 base
        // fonts shipped with every PDF reader, we don't have to embed any font
        // data.
        writer.type1_font(font_id).base_font(Name(b"Helvetica"));

        // Write a line of text, with the font specified in the resource list
        // before, at a font size of 14.0, starting at coordinates (108.0, 734.0)
        // measured from the bottom left of the page.
        //
        // Because we haven't specified any encoding when writing the Type 1 font,
        // the standard encoding is used which happens to work with most ASCII
        // characters.
        let mut content = Content::new();
        content.begin_text();
        content.set_font(font_name, 14.0);
        content.next_line(108.0, 734.0);
        content.show(Str(b"Hello World from Rust!"));
        content.end_text();

        // Write a cubic bezier curve
        content.save_state()
            .set_line_width(1.0)
            .set_line_join(LineJoinStyle::MiterJoin)
            .set_line_cap(LineCapStyle::ButtCap)
            .set_stroke_rgb(0.5, 0.0, 1.0)
            .move_to(100.0, 200.0)
            .line_to(200.0, 500.0)
            .line_to(10.0, 200.0)
            .cubic_to(40.0, 250.0, 100.0, 300.0, 150.0, 200.0)
            .line_to(0.0, 0.0)
            .stroke()
            .restore_state();

        writer.stream(content_id, &content.finish());

        // Finish writing (this automatically creates the cross-reference table and
        // file trailer) and retrieve the resulting byte buffer.
        let buf: Vec<u8> = writer.finish();
        Ok(buf)
    }


    pub async fn get_pdf_image(id: String) -> Result<Uint8Array, JsValue> {
        // Start writing.
        let mut writer = PdfWriter::new();

        // Define some indirect reference ids we'll use.
        let catalog_id = Ref::new(1);
        let page_tree_id = Ref::new(2);
        let page_id = Ref::new(3);
        let image_id = Ref::new(4);
        let s_mask_id = Ref::new(5);
        let content_id = Ref::new(6);
        let image_name = Name(b"Im1");

        // Set up the page tree. For more details see `hello.rs`.
        writer.catalog(catalog_id).pages(page_tree_id);
        writer.pages(page_tree_id).kids([page_id]).count(1);

        // Specify one A4 page and map the image name "Im1" to the id of the
        // embedded image stream.
        let mut page = writer.page(page_id);
        let a4 = Rect::new(0.0, 0.0, 595.0, 842.0);
        page.media_box(a4);
        page.parent(page_tree_id);
        page.contents(content_id);
        page.resources().x_objects().pair(image_name, image_id);
        page.finish();

        // Decode the image.
        let data = Self::fetch_image_data(&id).await?;
        let format = image::guess_format(&data).unwrap();
        let dynamic = image::load_from_memory(&data).unwrap();

        // Now, there are multiple considerations:
        // - Writing an XObject with just the raw samples would work, but lead to
        //   huge file sizes since the image would be embedded without any
        //   compression.
        // - We can encode the samples with a filter. However, which filter is best
        //   depends on the file format. For example, for JPEGs you should use
        //   DCT-Decode and for PNGs you should use Deflate.
        // - When the image has transparency, we need to provide that separately
        //   through an extra linked SMask image.
        let (filter, encoded, mask) = match format {
            // A JPEG is already valid DCT-encoded data.
            ImageFormat::Jpeg => {
                assert_eq!(dynamic.color(), ColorType::Rgb8);
                (Filter::DctDecode, data, None)
            }

            // While PNGs uses deflate internally, we need to re-encode to get just
            // the raw coded samples without metadata. Also, we need to encode the
            // RGB and alpha data separately.
            ImageFormat::Png => {
                let level = CompressionLevel::DefaultLevel as u8;
                let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

                // If there's an alpha channel, extract the pixel alpha values.
                let mask = dynamic.color().has_alpha().then(|| {
                    let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                    compress_to_vec_zlib(&alphas, level)
                });

                (Filter::FlateDecode, encoded, mask)
            }

            // You could handle other image formats similarly or just recode them to
            // JPEG or PNG, whatever best fits your use case.
            _ => panic!("unsupported image format"),
        };

        // Write the stream for the image we want to embed.
        let mut image = writer.image_xobject(image_id, &encoded);
        image.filter(filter);
        image.width(dynamic.width() as i32);
        image.height(dynamic.height() as i32);
        image.color_space().device_rgb();
        image.bits_per_component(8);
        if mask.is_some() {
            image.s_mask(s_mask_id);
        }
        image.finish();

        // Add SMask if the image has transparency.
        if let Some(encoded) = &mask {
            let mut s_mask = writer.image_xobject(s_mask_id, &encoded);
            s_mask.filter(filter);
            s_mask.width(dynamic.width() as i32);
            s_mask.height(dynamic.height() as i32);
            s_mask.color_space().device_gray();
            s_mask.bits_per_component(8);
        }

        // Size the image at 1pt per pixel.
        let w = 200.0 as f32;
        let h = dynamic.height() as f32 * 200.0 / dynamic.width() as f32;

        // Center the image on the page.
        let x = (a4.x2 - w) / 2.0;
        let y = (a4.y2 - h) / 2.0;

        // Place and size the image in a content stream.
        //
        // By default, PDF XObjects always have a size of 1x1 user units (and 1 user
        // unit is one 1pt if you don't change that). To position and size them, you
        // have to change the current transformation matrix, which is structured as
        // [scale_x, skew_x, skew_y, scale_y, translate_x, translate_y]. Also,
        // remember that the PDF coordinate system starts at the bottom left! When
        // you have other elements after the image, it's also important to save &
        // restore the state so that they are not affected by the transformation.
        let mut content = Content::new();
        content.save_state();
        content.transform([w, 0.0, 0.0, h, x, y]);
        content.x_object(image_name);
        content.restore_state();
        writer.stream(content_id, &content.finish());
        let data = writer.finish();
        let array = Uint8Array::new_with_length(data.len() as u32);
        array.copy_from(&data);
        Ok(array)
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pdf(){
        let mut app = WasmApp::new();
        let data = app.get_pdf().unwrap();
        std::fs::write("output/test.pdf", &data);
    }


}

