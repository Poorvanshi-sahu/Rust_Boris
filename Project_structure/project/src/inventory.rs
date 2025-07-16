pub mod product;
pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER :&str = "Ivan_inventory opo";

pub fn talk_to_manager(){
    // ******************* Absolute means complete from top, using crate
    println!("Hey {} how's your coffee!", crate::inventory::MANAGER);
}  


// submodules
// 1.inline
// 2.inventory/product.rs
// 3.inventory/product/mod.rs

// ******************* Anything declared within module is private

