// Answer 0

#[test]
fn test_general_purpose_config() {
    struct TestEngine {
        config: GeneralPurposeConfig,
    }

    impl Engine for TestEngine {
        type Config = GeneralPurposeConfig;
        type DecodeEstimate = GeneralPurposeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 0 }
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    let test_config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = TestEngine { config: test_config };

    assert_eq!(engine.config(), &test_config);
}

#[test]
fn test_general_purpose_config_no_padding() {
    struct TestEngine {
        config: GeneralPurposeConfig,
    }

    impl Engine for TestEngine {
        type Config = GeneralPurposeConfig;
        type DecodeEstimate = GeneralPurposeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 0 }
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    let test_config_no_pad = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: true,
        decode_padding_mode: DecodePaddingMode::Indifferent,
    };

    let engine = TestEngine { config: test_config_no_pad };

    assert_eq!(engine.config(), &test_config_no_pad);
}

