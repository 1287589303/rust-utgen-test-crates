// Answer 0

#[test]
fn test_newtype_variant_seed_invalid() {
    struct Seed;

    impl<'de> de::DeserializeSeed<'de> for Seed {
        type Value = ();
    }

    let unit_only: UnitOnly<()> = UnitOnly {
        marker: PhantomData,
    };
    let _ = unit_only.newtype_variant_seed(Seed);
}

#[test]
fn test_newtype_variant_seed_empty() {
    struct EmptySeed;

    impl<'de> de::DeserializeSeed<'de> for EmptySeed {
        type Value = ();
    }

    let unit_only: UnitOnly<()> = UnitOnly {
        marker: PhantomData,
    };
    let _ = unit_only.newtype_variant_seed(EmptySeed);
}

