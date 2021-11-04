use super::is_continuation;
use super::EncodeWide;
use super::Result;

const SURROGATE_LENGTH: usize = 3;

fn to_wide(string: &[u8]) -> impl '_ + Iterator<Item = Result<u16>> {
    EncodeWide::new(string.iter().map(|&x| x))
}

pub(in super::super) fn ends_with(
    string: &[u8],
    mut suffix: &[u8],
) -> Option<bool> {
    if suffix.is_empty() {
        return Some(true);
    }

    let index = string.len().checked_sub(suffix.len())?;
    if is_continuation(string[index]) {
        let index = index.checked_sub(1)?;
        let mut wide_suffix = to_wide(suffix.get(..SURROGATE_LENGTH)?);
        let suffix_wchar = wide_suffix
            .next()
            .expect("failed decoding non-empty suffix");

        if suffix_wchar.is_err()
            || wide_suffix.next().is_some()
            || suffix_wchar != to_wide(&string[index..]).nth(1)?
        {
            return None;
        }
        suffix = &suffix[SURROGATE_LENGTH..];
    }
    Some(string.ends_with(suffix))
}

pub(in super::super) fn starts_with(
    string: &[u8],
    mut prefix: &[u8],
) -> Option<bool> {
    if let Some(&byte) = string.get(prefix.len()) {
        if is_continuation(byte) {
            let index = prefix.len().checked_sub(SURROGATE_LENGTH)?;
            let mut wide_prefix = to_wide(&prefix[index..]);
            let prefix_wchar = wide_prefix
                .next()
                .expect("failed decoding non-empty prefix");

            if prefix_wchar.is_err()
                || wide_prefix.next().is_some()
                || prefix_wchar != to_wide(&string[index..]).next()?
            {
                return None;
            }
            prefix = &prefix[..index];
        }
    }
    Some(string.starts_with(prefix))
}
