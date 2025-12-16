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
    
    result.iter().for_each(|result| {
      for text in &result.text_regions {
        if let (Some(text), Some(confidence)) = (&text.text, &text.confidence) {
          println!("Confidence: {confidence} Text: {text}");
        }
      }
    });
    
    "anc".into()
  }
}



