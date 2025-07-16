#[derive(Debug)]

struct TaylorSwiftSong{
    title: String,
    release_year: u32,
    duration_secs: u32 
}

impl TaylorSwiftSong{
    fn new(title: String, release_year: u32, duration_secs: u32)->TaylorSwiftSong{
        TaylorSwiftSong{
            title,
            release_year,
            duration_secs
        }
    } 
}

impl TaylorSwiftSong{
    // immutable struct value (self parameter takes ownership)
    // fn display_song_info(self: &TaylorSwiftSong){
    fn display_song_info(&self){
        // immutable reference to the struct instance (no ownership moved)
        // mutable reference to the struct instance (no ownership moved, have permission to mutate)
        println!("title of song: {}", self.title);
        println!("year since release: {}", self.year_since_release());
        println!("duration of song: {}", self.duration_secs);
    }

    // mutable struct value (self parameter takes ownership, this time can mutate)
    fn double_speed(&mut self){
        self.duration_secs = self.duration_secs*2;
        println!("{:#?}", self);
    }

    fn is_longer_than(&self, other: &Self)-> bool{
        // println!("here printing: {} {}", self.duration_secs, other.duration_secs);
        self.duration_secs > other.duration_secs
    }

    fn year_since_release(&self)->u32{
        2025 - self.release_year
    }

}

fn main() {
    let song: TaylorSwiftSong = TaylorSwiftSong::new(String::from("the taylor song"), 2023, 90);
    let mut new_song = TaylorSwiftSong::new(String::from("the new song"), 1290, 23);

    // let mut song = TaylorSwiftSong{
    //     title: String::from("the"),
    //     release_year : 2012,
    //     duration_secs: 13
    // };

    // let mut new_song = TaylorSwiftSong{
    //     title: String::from("new one"),
    //     release_year : 2015,
    //     duration_secs: 13
    // };

    song.display_song_info();
    // song.double_speed();
    // song.display_song_info();

    if song.is_longer_than(&new_song){
        println!("song duration is more than new song");
    }else{
        println!("new song duration is more than or equal to song");
    } 
}
