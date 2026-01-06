use image::RgbImage;
use oar_ocr::prelude::{OAROCR, OAROCRBuilder};

use crate::processor::Processor;

pub struct PaddleOcrProcessor {
  oar: OAROCR
}

impl PaddleOcrProcessor {
  pub fn new() -> Self {
    Self {
      oar: OAROCRBuilder::new(
          "./paddle-paddle/det.onnx".into(),
          "./paddle-paddle/rec.onnx".into(),
          "./paddle-paddle/dict.txt".into()
        )
        .with_high_performance()
        .textline_orientation_classify_model_name("./paddle-paddle/textline.onnx".into())
        .doc_orientation_classify_model_name("./paddle-paddle/orientation.onnx".into())
        .doc_unwarping_model_name("./paddle-paddle/unwarping.onnx".into())
        .build()
        .unwrap()
    }
  }
}

impl Processor for PaddleOcrProcessor {
  fn detect(&mut self, image: &image::ImageBuffer<image::Rgb<u8>, &[u8]>) -> String {
    let mut rgb = Vec::new();
    rgb.extend_from_slice(image.as_raw());
    let rgb = RgbImage::from_raw(image.width(), image.height(), rgb).unwrap();
    
    let result = self.oar.predict(&[rgb]).unwrap();
    
    let mut string = String::new();
    result.iter().for_each(|result| {
      for text in &result.text_regions {
        if let (Some(text), Some(confidence)) = (&text.text, &text.confidence) {
          if *confidence < 0.70 {
            println!("AI not confidence enough not adding '{text}' (confidence: {confidence} < 70.0)");
          } else {
            if !string.is_empty() {
              string.push(' ');
            }
            string.push_str(text.trim());
          }
        }
      }
    });
    
    string
  }
}



