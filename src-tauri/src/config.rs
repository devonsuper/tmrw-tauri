// Enforce exactly one selected
#[cfg(not(any(
  feature = "mundo", feature = "kwabena", feature = "bukyt",
  feature = "suyay", feature = "djamila", feature = "test"
)))]
compile_error!("Select exactly one person feature.");

#[cfg(any(
  all(feature="mundo",   any(feature="kwabena", feature="bukyt", feature="suyay", feature="djamila", feature="test")),
  all(feature="kwabena", any(feature="mundo",   feature="bukyt", feature="suyay", feature="djamila", feature="test")),
  all(feature="bukyt",   any(feature="mundo",   feature="kwabena", feature="suyay", feature="djamila", feature="test")),
  all(feature="suyay",   any(feature="mundo",   feature="kwabena", feature="bukyt", feature="djamila", feature="test")),
  all(feature="djamila", any(feature="mundo",   feature="kwabena", feature="bukyt", feature="suyay", feature="test")),
  all(feature="test",    any(feature="mundo",   feature="kwabena", feature="bukyt", feature="suyay", feature="djamila"))
))]
compile_error!("Select only one person feature.");

// Single constant chosen at compile time
pub const PERSON: &str =
  if cfg!(feature="mundo")   { "mundo" }
  else if cfg!(feature="kwabena") { "kwabena" }
  else if cfg!(feature="bukyt")   { "bukyt" }
  else if cfg!(feature="suyay")   { "suyay" }
  else if cfg!(feature="djamila") { "djamila" }
  else { "test" };