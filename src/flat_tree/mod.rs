//! The flat tree stores a tree structure in an Vec.
//! This gives every element an unique index and allows easy traversing in the key.

pub mod tree;
mod index;
mod navigator;

pub use tree::FlatTree;
pub use tree::Builder as FlatTreeBuilder;
pub use index::Neighbors as FlatTreeNeighbors;
pub use navigator::Navigator;
pub use navigator::NavigatorWithValues;
