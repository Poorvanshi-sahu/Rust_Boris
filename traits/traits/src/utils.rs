// use crate::lodging::{Accommodation, Description};
use super::lodging::{Accommodation, Description};

fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str, days: u32){
    // println!("{}", entity.get_description());
    entity.book(guest, days);
}

// fn mix_and_match<T: Accommodation, U: Accommodation>(first: &mut T ,second: &mut U, guest: &str ){
// fn mix_and_match(first: &mut (impl Accommodation + Description) ,second: &mut (impl Accommodation + Description), guest: &str ){
fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str )
where 
    T: Accommodation + Description,
    U: Accommodation,
{    
    first.book(guest, 1);
    second.book(guest, 1);

}
 