fn canonical_binary(
        &self,
        name: &str,
    ) -> Result<CanonicalClassQuery, Error> {
        let norm = symbolic_name_normalize(name);

        // This is a special case where 'cf' refers to the 'Format' general
        // category, but where the 'cf' abbreviation is also an abbreviation
        // for the 'Case_Folding' property. But we want to treat it as
        // a general category. (Currently, we don't even support the
        // 'Case_Folding' property. But if we do in the future, users will be
        // required to spell it out.)
        //
        // Also 'sc' refers to the 'Currency_Symbol' general category, but is
        // also the abbreviation for the 'Script' property. So we avoid calling
        // 'canonical_prop' for it too, which would erroneously normalize it
        // to 'Script'.
        //
        // Another case: 'lc' is an abbreviation for the 'Cased_Letter'
        // general category, but is also an abbreviation for the 'Lowercase_Mapping'
        // property. We don't currently support the latter, so as with 'cf'
        // above, we treat 'lc' as 'Cased_Letter'.
        if norm != "cf" && norm != "sc" && norm != "lc" {
            if let Some(canon) = canonical_prop(&norm)? {
                return Ok(CanonicalClassQuery::Binary(canon));
            }
        }
        if let Some(canon) = canonical_gencat(&norm)? {
            return Ok(CanonicalClassQuery::GeneralCategory(canon));
        }
        if let Some(canon) = canonical_script(&norm)? {
            return Ok(CanonicalClassQuery::Script(canon));
        }
        Err(Error::PropertyNotFound)
    }