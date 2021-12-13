#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/opus.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        let mut error: i32 = 0;

        unsafe {
            let encoder = opus_encoder_create(48000, 1, OPUS_APPLICATION_AUDIO as i32, &mut error);
            assert_eq!(error, 0);
            
            opus_encoder_ctl(encoder, 16000);
        }
    }
}
