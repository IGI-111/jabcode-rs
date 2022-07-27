use super::option::ColorNumber;
use super::WriteError;
use crate::jabcode;

pub struct EncodeHandle<'a> {
    enc: &'a mut jabcode::jab_encode,
}

impl<'a> EncodeHandle<'a> {
    pub fn new(color_number: ColorNumber, symbol_number: usize) -> Result<Self, WriteError> {
        let enc = if let Some(enc) =
            unsafe { jabcode::createEncode(color_number.into(), symbol_number as i32).as_mut() }
        {
            enc
        } else {
            return Err(WriteError::Encode);
        };

        Ok(Self { enc })
    }

    pub fn enc_mut(&mut self) -> &mut jabcode::jab_encode {
        self.enc
    }
    #[allow(dead_code)]
    pub fn enc(&self) -> &jabcode::jab_encode {
        self.enc
    }
}

impl<'a> Drop for EncodeHandle<'a> {
    fn drop(&mut self) {
        unsafe {
            jabcode::destroyEncode(self.enc);
        }
    }
}
