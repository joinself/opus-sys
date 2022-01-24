extern crate bindgen;

use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut clang_flags = Vec::<String>::new();

    let opus_includes = Path::new("vendor/include/");
    let celt_includes = Path::new("vendor/celt/");
    let silk_includes = Path::new("vendor/silk/");
    let silk_fixed_includes = Path::new("vendor/silk/fixed/");

    if target == "wasm32-unknown-emscripten" {
        clang_flags.push(String::from("-fvisibility=default"));
    }

    cc::Build::new()
        .warnings(false)
        .include(opus_includes)
        .include(celt_includes)
        .include(silk_includes)
        .include(silk_fixed_includes)
        .define("USE_ALLOCA", "1")
        .define("FIXED_POINT", "1")
        .define("OPUS_BUILD", "1")
        .define("HAVE_LRINT", "1")
        .define("HAVE_LRINTF", "1")
        .define("LOCALE_NOT_USED", "1")
        .file("vendor/celt/bands.c")
        .file("vendor/celt/celt.c")
        .file("vendor/celt/celt_decoder.c")
        .file("vendor/celt/celt_encoder.c")
        .file("vendor/celt/celt_lpc.c")
        .file("vendor/celt/cwrs.c")
        .file("vendor/celt/entcode.c")
        .file("vendor/celt/entdec.c")
        .file("vendor/celt/entenc.c")
        .file("vendor/celt/kiss_fft.c")
        .file("vendor/celt/laplace.c")
        .file("vendor/celt/mathops.c")
        .file("vendor/celt/mdct.c")
        .file("vendor/celt/modes.c")
        .file("vendor/celt/pitch.c")
        .file("vendor/celt/quant_bands.c")
        .file("vendor/celt/rate.c")
        .file("vendor/celt/vq.c")
        .compile("celt");

    cc::Build::new()
        .warnings(false)
        .include(opus_includes)
        .include(celt_includes)
        .include(silk_includes)
        .include(silk_fixed_includes)
        .define("USE_ALLOCA", "1")
        .define("FIXED_POINT", "1")
        .define("OPUS_BUILD", "1")
        .define("HAVE_LRINT", "1")
        .define("HAVE_LRINTF", "1")
        .define("LOCALE_NOT_USED", "1")
        .file("vendor/silk/A2NLSF.c")
        .file("vendor/silk/ana_filt_bank_1.c")
        .file("vendor/silk/biquad_alt.c")
        .file("vendor/silk/bwexpander_32.c")
        .file("vendor/silk/bwexpander.c")
        .file("vendor/silk/check_control_input.c")
        .file("vendor/silk/CNG.c")
        .file("vendor/silk/code_signs.c")
        .file("vendor/silk/control_audio_bandwidth.c")
        .file("vendor/silk/control_codec.c")
        .file("vendor/silk/control_SNR.c")
        .file("vendor/silk/debug.c")
        .file("vendor/silk/dec_API.c")
        .file("vendor/silk/decode_core.c")
        .file("vendor/silk/decode_frame.c")
        .file("vendor/silk/decode_indices.c")
        .file("vendor/silk/decode_parameters.c")
        .file("vendor/silk/decode_pitch.c")
        .file("vendor/silk/decode_pulses.c")
        .file("vendor/silk/decoder_set_fs.c")
        .file("vendor/silk/enc_API.c")
        .file("vendor/silk/encode_indices.c")
        .file("vendor/silk/encode_pulses.c")
        .file("vendor/silk/gain_quant.c")
        .file("vendor/silk/HP_variable_cutoff.c")
        .file("vendor/silk/init_decoder.c")
        .file("vendor/silk/init_encoder.c")
        .file("vendor/silk/inner_prod_aligned.c")
        .file("vendor/silk/interpolate.c")
        .file("vendor/silk/lin2log.c")
        .file("vendor/silk/log2lin.c")
        .file("vendor/silk/LPC_analysis_filter.c")
        .file("vendor/silk/LPC_fit.c")
        .file("vendor/silk/LPC_inv_pred_gain.c")
        .file("vendor/silk/LP_variable_cutoff.c")
        .file("vendor/silk/NLSF2A.c")
        .file("vendor/silk/NLSF_decode.c")
        .file("vendor/silk/NLSF_del_dec_quant.c")
        .file("vendor/silk/NLSF_encode.c")
        .file("vendor/silk/NLSF_stabilize.c")
        .file("vendor/silk/NLSF_unpack.c")
        .file("vendor/silk/NLSF_VQ.c")
        .file("vendor/silk/NLSF_VQ_weights_laroia.c")
        .file("vendor/silk/NSQ.c")
        .file("vendor/silk/NSQ_del_dec.c")
        .file("vendor/silk/pitch_est_tables.c")
        .file("vendor/silk/PLC.c")
        .file("vendor/silk/process_NLSFs.c")
        .file("vendor/silk/quant_LTP_gains.c")
        .file("vendor/silk/resampler.c")
        .file("vendor/silk/resampler_down2_3.c")
        .file("vendor/silk/resampler_down2.c")
        .file("vendor/silk/resampler_private_AR2.c")
        .file("vendor/silk/resampler_private_down_FIR.c")
        .file("vendor/silk/resampler_private_IIR_FIR.c")
        .file("vendor/silk/resampler_private_up2_HQ.c")
        .file("vendor/silk/resampler_rom.c")
        .file("vendor/silk/shell_coder.c")
        .file("vendor/silk/sigm_Q15.c")
        .file("vendor/silk/sort.c")
        .file("vendor/silk/stereo_decode_pred.c")
        .file("vendor/silk/stereo_encode_pred.c")
        .file("vendor/silk/stereo_find_predictor.c")
        .file("vendor/silk/stereo_LR_to_MS.c")
        .file("vendor/silk/stereo_MS_to_LR.c")
        .file("vendor/silk/stereo_quant_pred.c")
        .file("vendor/silk/sum_sqr_shift.c")
        .file("vendor/silk/table_LSF_cos.c")
        .file("vendor/silk/tables_gain.c")
        .file("vendor/silk/tables_LTP.c")
        .file("vendor/silk/tables_NLSF_CB_NB_MB.c")
        .file("vendor/silk/tables_NLSF_CB_WB.c")
        .file("vendor/silk/tables_other.c")
        .file("vendor/silk/tables_pitch_lag.c")
        .file("vendor/silk/tables_pulses_per_block.c")
        .file("vendor/silk/VAD.c")
        .file("vendor/silk/VQ_WMat_EC.c")
        .compile("silk");

    cc::Build::new()
        .warnings(false)
        .include(opus_includes)
        .include(celt_includes)
        .include(silk_includes)
        .include(silk_fixed_includes)
        .define("USE_ALLOCA", "1")
        .define("FIXED_POINT", "1")
        .define("OPUS_BUILD", "1")
        .define("HAVE_LRINT", "1")
        .define("HAVE_LRINTF", "1")
        .define("LOCALE_NOT_USED", "1")
        .file("vendor/src/analysis.c")
        .file("vendor/src/mapping_matrix.c")
        .file("vendor/src/mlp.c")
        .file("vendor/src/mlp_data.c")
        .file("vendor/src/opus.c")
        .file("vendor/src/opus_decoder.c")
        .file("vendor/src/opus_encoder.c")
        .file("vendor/src/opus_multistream.c")
        .file("vendor/src/opus_multistream_decoder.c")
        .file("vendor/src/opus_multistream_encoder.c")
        .file("vendor/src/opus_projection_decoder.c")
        .file("vendor/src/opus_projection_encoder.c")
        .file("vendor/src/repacketizer.c")
        .compile("opus");

    // generate the bindings for opus headers
    let mut builder = bindgen::Builder::default();

    for value in &clang_flags {
        builder = builder.clang_arg(value);
    }

    let bindings = builder
        .clang_arg("-Ivendor/include/")
        .clang_arg("-Ivendor/celt/")
        .clang_arg("-Ivendor/silk/")
        .clang_arg("-Ivendor/silk/fixed/")
        .allowlist_type(r"opus.*")
        .allowlist_type(r"Opus.*")
        .allowlist_type(r"OPUS.*")
        .allowlist_function(r"opus.*")
        .allowlist_function(r"Opus.*")
        .allowlist_function(r"OPUS.*")
        .allowlist_var(r"opus.*")
        .allowlist_var(r"Opus.*")
        .allowlist_var(r"OPUS.*")
        .header("vendor/include/opus.h")
        .header("vendor/silk/fixed/main_FIX.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate opus bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("opus.rs"))
        .expect("Couldn't write opus bindings!");
}
