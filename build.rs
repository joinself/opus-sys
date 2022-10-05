extern crate bindgen;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut defines = HashMap::<&str, &str>::new();
    let mut clang_flags = Vec::<String>::new();

    let mut celt_sources = Vec::<&str>::new();
    let mut silk_sources = Vec::<&str>::new();

    let includes = Path::new("vendor/");
    let opus_includes = Path::new("vendor/include/");
    let celt_includes = Path::new("vendor/celt/");
    let silk_includes = Path::new("vendor/silk/");
    let silk_fixed_includes = Path::new("vendor/silk/fixed/");
    let silk_float_includes = Path::new("vendor/silk/float/");

    defines.insert("USE_ALLOCA", "1");
    defines.insert("OPUS_BUILD", "1");
    defines.insert("HAVE_LRINT", "1");
    defines.insert("HAVE_LRINTF", "1");
    defines.insert("LOCALE_NOT_USED", "1");

    if target == "i686-linux-android" {
    } else if target == "x86_64-apple-darwin" {
    } else if target == "x86_64-apple-ios" {
    } else if target == "x86_64-linux-android" {
    } else if target == "x86_64-unknown-linux-gnu" {
    } else if target == "armv7-linux-androideabi" {
        defines.insert("OPUS_ARM_ASM", "1");
        defines.insert("OPUS_ARM_INLINE_ASM", "1");
        defines.insert("OPUS_ARM_INLINE_EDSP", "1");
        defines.insert("OPUS_ARM_MAY_HAVE_EDSP", "1");
        defines.insert("OPUS_ARM_MAY_HAVE_MEDIA", "1");
        defines.insert("OPUS_ARM_MAY_HAVE_NEON", "1");
        defines.insert("OPUS_HAVE_RTCD", "1");

        // celt_sources.push("vendor/celt/arm/celt_pitch_xcorr_arm_gnu.S");
        celt_sources.push("vendor/celt/arm/armcpu.c");
        celt_sources.push("vendor/celt/arm/arm_celt_map.c");
        celt_sources.push("vendor/celt/arm/celt_neon_intr.c");
        celt_sources.push("vendor/celt/arm/pitch_neon_intr.c");
        silk_sources.push("vendor/silk/arm/arm_silk_map.c");
        silk_sources.push("vendor/silk/arm/biquad_alt_neon_intr.c");
        silk_sources.push("vendor/silk/arm/LPC_inv_pred_gain_neon_intr.c");
        silk_sources.push("vendor/silk/arm/NSQ_del_dec_neon_intr.c");
        silk_sources.push("vendor/silk/arm/NSQ_neon.c");
        clang_flags.push(String::from("-ffast-math"));
        clang_flags.push(String::from("-funroll-loops"));
    } else if target == "aarch64-apple-darwin" {
    } else if target == "aarch64-apple-ios" {
    } else if target == "aarch64-apple-ios-sim" {
    } else if target == "aarch64-linux-android" {
        defines.insert("OPUS_ARM_ASM", "1");
        defines.insert("OPUS_ARM_MAY_HAVE_NEON", "1");
        defines.insert("OPUS_ARM_MAY_HAVE_NEON_INTR", "1");
        defines.insert("OPUS_ARM_PRESUME_NEON_INTR", "1");
        defines.insert("OPUS_ARM_PRESUME_AARCH64_NEON_INTR", "1");
        defines.insert("OPUS_X86_PRESUME_AARCH64_NEON_INTR", "1");
        defines.insert("OPUS_HAVE_RTCD", "1");

        // celt_sources.push("vendor/celt/arm/celt_pitch_xcorr_arm_gnu.S");
        celt_sources.push("vendor/celt/arm/armcpu.c");
        celt_sources.push("vendor/celt/arm/arm_celt_map.c");
        celt_sources.push("vendor/celt/arm/celt_neon_intr.c");
        celt_sources.push("vendor/celt/arm/pitch_neon_intr.c");
        silk_sources.push("vendor/silk/arm/arm_silk_map.c");
        silk_sources.push("vendor/silk/arm/biquad_alt_neon_intr.c");
        silk_sources.push("vendor/silk/arm/LPC_inv_pred_gain_neon_intr.c");
        silk_sources.push("vendor/silk/arm/NSQ_del_dec_neon_intr.c");
        silk_sources.push("vendor/silk/arm/NSQ_neon.c");
        clang_flags.push(String::from("-ffast-math"));
        clang_flags.push(String::from("-funroll-loops"));
    } else if target == "aarch64-unknown-linux-gnu" {
    } else if target == "wasm32-unknown-emscripten" {
        clang_flags.push(String::from("-fvisibility=default"));
    }

    let mut celt_cmd = cc::Build::new();

    celt_cmd
        .warnings(false)
        .include(includes)
        .include(opus_includes)
        .include(celt_includes)
        .include(silk_includes)
        .include(silk_fixed_includes)
        .include(silk_float_includes);

    for source in &celt_sources {
        celt_cmd.file(source);
    }

    for (key, value) in &defines {
        celt_cmd.define(key, *value);
    }

    celt_cmd
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
        .file("vendor/celt/vq.c");

    celt_cmd.compile("celt");

    let mut silk_cmd = cc::Build::new();

    silk_cmd
        .warnings(false)
        .include(includes)
        .include(opus_includes)
        .include(celt_includes)
        .include(silk_includes)
        .include(silk_fixed_includes)
        .include(silk_float_includes)
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
        .file("vendor/silk/float/apply_sine_window_FLP.c")
        .file("vendor/silk/float/corrMatrix_FLP.c")
        .file("vendor/silk/float/encode_frame_FLP.c")
        .file("vendor/silk/float/find_LPC_FLP.c")
        .file("vendor/silk/float/find_LTP_FLP.c")
        .file("vendor/silk/float/find_pitch_lags_FLP.c")
        .file("vendor/silk/float/find_pred_coefs_FLP.c")
        .file("vendor/silk/float/LPC_analysis_filter_FLP.c")
        .file("vendor/silk/float/LTP_analysis_filter_FLP.c")
        .file("vendor/silk/float/LTP_scale_ctrl_FLP.c")
        .file("vendor/silk/float/noise_shape_analysis_FLP.c")
        .file("vendor/silk/float/process_gains_FLP.c")
        .file("vendor/silk/float/regularize_correlations_FLP.c")
        .file("vendor/silk/float/residual_energy_FLP.c")
        .file("vendor/silk/float/warped_autocorrelation_FLP.c")
        .file("vendor/silk/float/wrappers_FLP.c")
        .file("vendor/silk/float/autocorrelation_FLP.c")
        .file("vendor/silk/float/burg_modified_FLP.c")
        .file("vendor/silk/float/bwexpander_FLP.c")
        .file("vendor/silk/float/energy_FLP.c")
        .file("vendor/silk/float/inner_product_FLP.c")
        .file("vendor/silk/float/k2a_FLP.c")
        .file("vendor/silk/float/LPC_inv_pred_gain_FLP.c")
        .file("vendor/silk/float/pitch_analysis_core_FLP.c")
        .file("vendor/silk/float/scale_copy_vector_FLP.c")
        .file("vendor/silk/float/scale_vector_FLP.c")
        .file("vendor/silk/float/schur_FLP.c")
        .file("vendor/silk/float/sort_FLP.c");

    for (key, value) in &defines {
        silk_cmd.define(key, *value);
    }

    for source in &silk_sources {
        silk_cmd.file(source);
    }

    silk_cmd.compile("silk");

    let mut opus_cmd = cc::Build::new();

    opus_cmd
        .warnings(false)
        .include(includes)
        .include(opus_includes)
        .include(celt_includes)
        .include(silk_includes)
        .include(silk_fixed_includes)
        .include(silk_float_includes)
        .define("USE_ALLOCA", "1")
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
        .file("vendor/src/repacketizer.c");

    for (key, value) in &defines {
        opus_cmd.define(key, *value);
    }

    opus_cmd.compile("opus");

    // generate the bindings for opus headers
    let mut builder = bindgen::Builder::default();

    for value in &clang_flags {
        builder = builder.clang_arg(value);
    }

    let bindings = builder
        .clang_arg("-Ivendor/")
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
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate opus bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("opus.rs"))
        .expect("Couldn't write opus bindings!");
}
