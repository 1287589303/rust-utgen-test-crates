// Answer 0

#[test]
fn test_damerau_levenshtein_impl() {
    struct HybridGrowingHashmapChar<RowId> {
        map: std::collections::HashMap<char, RowId>,
    }

    impl Default for HybridGrowingHashmapChar<isize> {
        fn default() -> Self {
            HybridGrowingHashmapChar {
                map: std::collections::HashMap::new(),
            }
        }
    }

    impl HybridGrowingHashmapChar<isize> {
        fn get(&self, key: char) -> &RowId {
            self.map.get(&key).unwrap_or(&-1)
        }
        
        fn get_mut(&mut self, key: char) -> &mut RowId {
            self.map.entry(key).or_insert(-1)
        }
    }

    fn damerau_levenshtein_impl<Iter1, Iter2>(s1: Iter1, len1: usize, s2: Iter2, len2: usize) -> usize
    where
        Iter1: Iterator<Item = char> + Clone,
        Iter2: Iterator<Item = char> + Clone,
    {
        let max_val = std::cmp::max(len1, len2) as isize + 1;
        let mut last_row_id = HybridGrowingHashmapChar::default();
        let size = len2 + 2;
        let mut fr = vec![max_val; size];
        let mut r1 = vec![max_val; size];
        let mut r: Vec<isize> = (max_val..max_val + 1).chain(0..(size - 1) as isize).collect();

        for (i, ch1) in s1.enumerate().map(|(i, ch1)| (i + 1, ch1)) {
            std::mem::swap(&mut r, &mut r1);
            let mut last_col_id: isize = -1;
            let mut last_i2l1 = r[1];
            r[1] = i as isize;
            let mut t = max_val;

            for (j, ch2) in s2.clone().enumerate().map(|(j, ch2)| (j + 1, ch2)) {
                let diag = r1[j] + isize::from(ch1 != ch2);
                let left = r[j] + 1;
                let up = r1[j + 1] + 1;
                let mut temp = std::cmp::min(diag, std::cmp::min(left, up));

                if ch1 == ch2 { continue; }

                last_col_id = j as isize; 
                fr[j + 1] = r1[j - 1]; 
                t = last_i2l1; 

                if j as isize - last_col_id == 1 {
                    let k = last_row_id.get(ch2);
                    let transpose = fr[j + 1] + (i as isize - k);
                    temp = std::cmp::min(temp, transpose);
                } 

                last_i2l1 = r[j + 1];
                r[j + 1] = temp;
            }

            last_row_id.get_mut(ch1) = i as isize;
        }

        r[len2 + 1] as usize
    }

    let s1 = "kitten".chars();
    let s2 = "sitting".chars();
    let result = damerau_levenshtein_impl(s1, 6, s2, 7);
    assert_eq!(result, 3);
}

