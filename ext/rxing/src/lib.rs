use magnus::{function, method, prelude::*, Error, Ruby, RString, RArray};
use rxing::{self, ResultPoint};
use image;

#[magnus::wrap(class = "Rxing::BarcodeResult")]
struct BarcodeResult {
    text: String,
    format: String,
    points: Vec<(i32, i32)>,
}

impl BarcodeResult {
    fn new(text: String, format: String, points: Vec<(i32, i32)>) -> Self {
        Self { text, format, points }
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn format(&self) -> &str {
        &self.format
    }

    fn points(&self) -> Result<RArray, Error> {
        let array = RArray::new();
        for (x, y) in &self.points {
            let point = RArray::new();
            point.push(*x)?;
            point.push(*y)?;
            array.push(point)?;
        }
        Ok(array)
    }
}

#[magnus::wrap(class = "Rxing::BarcodeReader")]
struct BarcodeReader;

impl BarcodeReader {
    fn new() -> Self {
        BarcodeReader
    }

    fn process_results(results: Vec<rxing::RXingResult>) -> Result<RArray, Error> {
        let array = RArray::new();
        for result in results {
            let points = result.getPoints();
            if points.len() >= 2 {
                let points: Vec<_> = points.iter()
                    .map(|p| (p.getX() as i32, p.getY() as i32))
                    .collect();
                let barcode = BarcodeResult::new(
                    result.getText().to_string(),
                    format!("{:?}", result.getBarcodeFormat()),
                    points,
                );
                array.push(barcode)?;
            }
        }
        Ok(array)
    }

    fn read_from_file(&self, path: String) -> Result<RArray, Error> {
        let results = rxing::helpers::detect_multiple_in_file(&path)
            .map_err(|e| Error::new(magnus::exception::runtime_error(), format!("Failed to read file: {}", e)))?;
        Self::process_results(results)
    }

    fn read_from_bytes(&self, data: RString) -> Result<RArray, Error> {
        let bytes = unsafe {
          // SAFETY: We are only reading byte data and will not modify it
          // During this function call, the Ruby string will not be modified
            data.as_slice()
        };
        let img = image::load_from_memory(bytes)
            .map_err(|e| Error::new(magnus::exception::runtime_error(), format!("Failed to load image: {}", e)))?;
        let luma = img.to_luma8();
        let width = luma.width();
        let height = luma.height();
        let results = rxing::helpers::detect_multiple_in_luma(luma.into_raw(), width, height)
            .map_err(|e| Error::new(magnus::exception::runtime_error(), format!("Failed to detect barcodes: {}", e)))?;
        Self::process_results(results)
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Rxing")?;

    let reader_class = module.define_class("BarcodeReader", ruby.class_object())?;
    reader_class.define_singleton_method("new", function!(BarcodeReader::new, 0))?;
    reader_class.define_method("read_from_file", method!(BarcodeReader::read_from_file, 1))?;
    reader_class.define_method("read_from_bytes", method!(BarcodeReader::read_from_bytes, 1))?;

    let result_class = module.define_class("BarcodeResult", ruby.class_object())?;
    result_class.define_method("text", method!(BarcodeResult::text, 0))?;
    result_class.define_method("format", method!(BarcodeResult::format, 0))?;
    result_class.define_method("points", method!(BarcodeResult::points, 0))?;

    Ok(())
}
