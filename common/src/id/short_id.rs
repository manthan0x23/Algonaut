use nanoid;

/// Generates `shortId` from `nanoid::nanoid!()`
/// ### Params
///  - size : `Option<usize>` (size of the generated id) by default `8`
pub fn short_id(size: Option<usize>) -> ShortId {
    let len = size.unwrap_or(8);
    nanoid::nanoid!(len)
}

/// Type alias for `short_id`
pub type ShortId = String;
