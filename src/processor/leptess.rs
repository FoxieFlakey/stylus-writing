use image::{ImageBuffer, Rgb, codecs::png::PngEncoder};
use leptess::LepTess;

use crate::processor::Processor;

pub struct LepTessProcessor {
  api: LepTess,
  data_buffer: Vec<u8>
}

impl LepTessProcessor {
  #[expect(unused)]
  pub fn new() -> Self {
    let mut result = Self {
      api: LepTess::new(Some("./tessdata"), "eng").unwrap(),
      data_buffer: Vec::new()
    };
    
    result.api.set_variable(leptess::Variable::TesseditOcrEngineMode, "lstm").unwrap();
    result.api.set_variable(leptess::Variable::SuperscriptScaledownRatio, "3.0").unwrap();
    result.api.set_variable(leptess::Variable::SubscriptMaxYTop, "3.0").unwrap();
    result.api.set_variable(leptess::Variable::SuperscriptMinYBottom, "3.0").unwrap();
    result.api.set_variable(leptess::Variable::TesseditZeroRejection, "true").unwrap();
    result.api.set_variable(leptess::Variable::TesseditZeroKelvinRejection, "true").unwrap();
    result.api.set_variable(leptess::Variable::TesseditUnrejAnyWd, "true").unwrap();
    result.api.set_variable(leptess::Variable::TesseditPreserveMinWdLen, "0").unwrap();
    result.api.set_variable(leptess::Variable::BlandUnrej, "true").unwrap();
    result.api.set_variable(leptess::Variable::SuspectLevel, "80").unwrap();
    result.api.set_variable(leptess::Variable::TesseditParallelize, "true").unwrap();
    
    result
  }
}

impl Processor for LepTessProcessor {
  fn detect(&mut self, img: &ImageBuffer<Rgb<u8>, &[u8]>) -> String {
    self.data_buffer.clear();
    img.write_with_encoder(PngEncoder::new(&mut self.data_buffer)).unwrap();
    
    self.api.set_image_from_mem(&self.data_buffer).unwrap();
    
    self.api.get_utf8_text().unwrap()
  }
}


