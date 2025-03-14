// Answer 0

#[test]
fn test_config() {
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

    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };
    
    let engine = TestEngine { config };

    let returned_config = engine.config();
    
    assert_eq!(returned_config.encode_padding, true);
    assert_eq!(returned_config.decode_allow_trailing_bits, false);
    assert_eq!(returned_config.decode_padding_mode, DecodePaddingMode::RequireNone);
}

