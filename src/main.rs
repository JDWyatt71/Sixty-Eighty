
//use core::error;
//use std::array;
use std::f32::consts::E;
use std::io::{Error, Read};
use std::num::ParseIntError;
use std::thread::current;
//use std::intrinsics::nearbyintf128;
use std::{io, ptr::null};
use std::fmt::{self, Debug, Display};
use chrono::{Date, DateTime, Datelike};
use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::fs::OpenOptions;
use std::{fs, pin, primitive, usize};
use std::process::{self, exit};
use std::path::Path; 
use std::env;

fn main() {
    /*
    Description: Loads the primary data structures or intitialises them if they are empty. Allows user to input commands for calling all other functions;
    recursive main control loop.
    Parameters: 
    None
    Returns: 
    None
    */
    println!("Welcome to Sixty-Eighty, a database cataloguing music artists and their album releases from the 
years 1960-1980. Type 'help' if you are unsure what commands are available");
    //Loads data for the primary data structures
    let mut data:(Vec<Artist>, Vec<Album>,  Vec<(String, Vec<String>, Vec<String>)>)=load_data();
    let mut artists: Vec<Artist>=data.0;
    let mut albums: Vec<Album>=data.1;
    let mut rankings: Vec<(String, Vec<String>, Vec<String>)>=data.2;
    //If there is no saved data for the primary data structures, Initialises them
    if artists.is_empty() {artists=artists_init()}
    if albums.is_empty() {albums=albums_init()}
    if rankings.is_empty() {rankings=rankings_init()}

    loop {
        //Loads user input for command into a variable
        println!("Enter a command");
        let mut cmd: String = String::new();
        io::stdin().read_line(&mut cmd).expect("");
        //Saves data and close program if the termination command is given
        if cmd.trim().to_lowercase()=="ex" {
            println!("Program Finish");
            save_data(albums, artists, rankings);
            break
        }
        //If a command other than the termination command is given, processes the command in a seperate function
        else {
            let cmd_pass: String=cmd.clone();
            data=open_section(&cmd_pass, artists, albums, rankings);
            artists=data.0;
            albums=data.1;
            rankings=data.2;
            //if all primary data structures are empty (this is a termination signal), closes the program
            if artists.is_empty() && albums.is_empty() && rankings.is_empty() {break}
        }
    }
}

fn open_section(cmd: &str, mut artists: Vec<Artist>, mut albums: Vec<Album>, mut rankings: Vec<(String, Vec<String>, Vec<String>)>)
-> (Vec<Artist>, Vec<Album>, Vec<(String, Vec<String>, Vec<String>)>) {
    /*
    Description: 
    Calls functions handling main operations based on command inputed in main() and handles related processes. 
    Parameters: 
    cmd: &str-The command input by the user, checked against existing functions and if the names match, 
    a process is run to call and handle that function
    artists: Vec<Artist>-One of the primary data structures, a vector of every artist in the system
    albums: Vec<Artist>-One of the primary data structures, a vector of every album in the system
    rankings: Vec<(String, Vec<String>, Vec<String>)>-One of the primary data structures, 
    a vector storing the rankings and unranked albums for every artist, keyed by artist name
    Returns: 
    artists: Vec<Artist>-One of the primary data structures, a vector of every artist in the system
    albums: Vec<Artist>-One of the primary data structures, a vector of every album in the system
    rankings: Vec<(String, Vec<String>, Vec<String>)>-One of the primary data structures, 
    a vector storing the rankings and unranked albums for every artist, keyed by artist name
    */
    let cmdstr: String=cmd.to_string().clone();
    let mut entry_search=false;
    //If the command ends with a question mark, calls the function to provide info on the function queried
    if cmdstr.trim().ends_with("?") {
        function_descs(&cmdstr.to_lowercase());
        return (artists, albums, rankings)
    }
    //If the command begins with a '/', starts the routin for printing artist/album information
    else if cmdstr.chars().nth(0).unwrap()=='/'{
        entry_search=true;
        let mut letter: char;
        let mut name: String=String::new();
        let mut idx: usize=0;
        //Finds the name of the artist/album queried by removing the starting slashes from the command string and saving this to a new variable
        for (i, &item) in cmdstr.as_bytes().iter().enumerate() {
            letter=item as char;
            idx+=1;
            if letter == '/'  && idx<3 {
                continue;
            }
            name.push(letter);
            
        }
        //If the second character is a slash, calls find album on the name, otherwise calls find artist
        if cmdstr.chars().nth(1).unwrap()=='/' {
            let check_album=find_album(&albums, name.to_lowercase().trim());
            if check_album.0==false {println!("Album not found in database");}
            else {
                let album=&albums[check_album.1];
                album.to_string();
            }
        } 
        else {
            let check_artist=find_artist(&artists, name.to_lowercase().trim());
            if check_artist.0==false {println!("Artist not found in database");}
            else {
                let artist=&artists[check_artist.1];
                artist.to_string();
            }
        }
    }

    match cmd.to_lowercase().trim() {
        //Calls the addartist function, and adds the resulting artist data to the artists and rankings vectors
        "addartist" => {
            let artist: Artist=add_artist(&artists);
            //Specific command returns. Empty string returns to the start of the current section,
            //NullA returns to the main command section, and TerminateProgram saves and exits the program
            //These come up repeatedly so I won't mention these operations again
            if artist.name== "" {}
            else if artist.name=="NullA" {return (artists, albums, rankings)}
            else if artist.name=="TerminateProgram" {
                save_data(albums, artists, rankings);
                return (Vec::new(), Vec::new(), Vec::new());
            }
            else {
                println!("{} has been added", artist.name);
                rankings.push((artist.name.clone(), Vec::new(), Vec::new()));
                artists.push(artist);
                
            }
        },
        "editartist" => {
            let mut check_artist=(false, 10000);
            let artistidx: usize=loop {
                let artistname: String=add_field("Enter artist", "", "");
                if artistname=="NullA" {return (artists, albums, rankings)}
                else if artistname=="TerminateProgram" {
                    save_data(albums, artists, rankings);
                    return (Vec::new(), Vec::new(), Vec::new());
                }
                //Checks to see if the artist exists within the system, and if they do, returns them, 
                //otherwise loops until an existing artist is entered or 'return' is typed
                //This operation type also comes up repeatedly, likewise I'll only make note of it here
                check_artist=find_artist(&artists, &artistname.to_lowercase());
                if check_artist.0==false {
                    println!("Artist not found in database");
                }
                else {break check_artist.1;}
            };

            let artist=artists[artistidx].clone();
            let og_name=artist.name.clone();
            let newartist=edit_artist(artist, &artists);
            let newname=newartist.name.clone();
            if newartist.name=="NullA" {return (artists, albums, rankings)}
                else if newartist.name=="TerminateProgram" {
                    save_data(albums, artists, rankings);
                    return (Vec::new(), Vec::new(), Vec::new());
                }
            //inserts the new version of the artist entry in the same position as the original version
            artists.remove(artistidx);
            artists.insert(artistidx, newartist);
            //If the name field has been changed, updates the album and ranking entries pertaining to the edited artist and alters their name there
            if og_name.trim()!=newname.clone().trim() {
                for i in albums.clone() {
                    if i.artist.trim()==og_name.trim() {
                        let newalbum: Album=Album { name: i.name.clone(), genre: i.genre, artist: newname.clone(), format: i.format,
                            release_year: i.release_year, record_dates: i.record_dates, description: i.description };
                        let album_idx=find_album(&albums, &i.name.to_lowercase()).1;
                        albums.remove(album_idx);
                        albums.insert(album_idx, newalbum);

                    }
                }
                for i in rankings.clone() {
                    if i.0.trim()==og_name.trim() {
                        let newranking=(newname.clone(), i.1.clone(), i.2.clone());
                        let rank_idx=find_ranking(&rankings, og_name.to_lowercase().trim()).1;
                        rankings.remove(rank_idx);
                        rankings.insert(rank_idx, newranking);
                    }
                }
            }
        },
        "delartist" => {
            //Prevents user from deleting last artist in the system
            if artists.len()==1 {
                println!("Cannot delete last remaining artist in the system");
                return (artists, albums, rankings);
            }
            let mut check_artist=(false, 10000);
            let artistidx: usize=loop {
                let artistname: String=add_field("Enter artist", "", "");
                if artistname=="NullA" {return (artists, albums, rankings)}
                else if artistname=="TerminateProgram" {
                    save_data(albums, artists, rankings);
                    return (Vec::new(), Vec::new(), Vec::new());
                }
                check_artist=find_artist(&artists, &artistname.to_lowercase());
                if check_artist.0==false {
                    println!("Artist not found in database. Create artist entry before creating their albums");
                }
                else {break check_artist.1;}
            };
            //Calls delete artist function and updates all primary data structures
            let mut newstructs=delete_artist(artists, albums, rankings, artistidx);
            artists=newstructs.0;
            albums=newstructs.1;
            rankings=newstructs.2;
        },
        "addalbum" => {
            let mut check_artist=(false, 10000);
            let artist_details: (usize, String)=loop {
                let artistname: String=add_field("Enter artist", "", "");
                if artistname=="NullA" {return (artists, albums, rankings)}
                else if artistname=="TerminateProgram" {
                    save_data(albums, artists, rankings);
                    return (Vec::new(), Vec::new(), Vec::new());
                }   
                check_artist=find_artist(&artists, &artistname.to_lowercase());
                if check_artist.0==false {
                    println!("Artist not found in database. Create artist entry before creating their albums");
                }
                else {
                    //Prevents the user from entering more than 45 albums. Limit is quite arbitrary though; 
                    //James Brown has 44 during this era, if anyone has more I could raise it, not likely though
                    if artists[check_artist.1].albums.len()>45 {println!("Maximum number of albums for artist {} reached", artistname)}
                    else {
                        let true_artistname=artists[check_artist.1].name.clone();
                        break (check_artist.1, true_artistname);
                    }
                }
                
            };
            let artistidx=artist_details.0;
            let artistname=artist_details.1;
            let artist_albums=artists[artistidx].albums.clone();
            let album: Album=add_album(artistname.clone(), albums.clone());
            let albumname=album.name.clone();
            let album_copy=album.clone();
            if albumname=="NullA" {return (artists, albums, rankings)}
            if albumname=="TerminateProgram" {
                save_data(albums, artists, rankings);
                return (Vec::new(), Vec::new(), Vec::new());
            } 
            else if albumname!= "" {
                println!("{} has been added", album.name);
                //Adds new album to albums and artists
                albums.push(album);
                let artist: Artist=artists[artistidx].clone();
                let mut newalbums: Vec<Album>=artist.albums;
                newalbums=insert_artist_album(newalbums.clone(), album_copy);
                let newartist: Artist=Artist { name: artist.name, genre: artist.genre, members: artist.members, 
                    years_active: artist.years_active, description: artist.description, 
                    albums: newalbums};
                artists.remove(artistidx);
                artists.insert(artistidx, newartist);
                
                //Adds album to the list of unranked albums in its artist's ranking entry
                //Also generates an additional empty rank element based on how many albums the artist has
                let ranking_idx=find_ranking(&rankings, &artistname.to_lowercase()).1;
                let ranking_data=rankings[ranking_idx].clone();
                let mut ranking=ranking_data.1.clone();
                let mut remaining_albums=ranking_data.2.clone();
                remaining_albums.push(albumname);
                let mut newrank_str;
                if ranking.is_empty()==false {
                    let rank_str=ranking.last().unwrap();
                   let rankno=get_rankno(&rank_str);
                    let newrank=rankno+1;
                    newrank_str=newrank.to_string();
                    newrank_str.push_str(". ");
                }
                else {newrank_str=String::from("1. ")}
                ranking.push(newrank_str);
                rankings.remove(ranking_idx);
                rankings.insert(ranking_idx, (artistname, ranking, remaining_albums));
            }
        },
        "editalbum" => {
            let mut check_album=(false, 10000);
            let albumidx=loop {
                let albumname: String=add_field("Enter album", "", "");
                if albumname=="NullA" {return (artists, albums, rankings)}
                if albumname=="TerminateProgram" {
                    save_data(albums, artists, rankings);
                    return (Vec::new(), Vec::new(), Vec::new())
                }
                check_album=find_album(&albums, &albumname.to_lowercase());
                if check_album.0==false {println!("Album not found in database");}
                else {break check_album.1;}
            };
            let og_album=albums[albumidx].clone();
            let original_name=og_album.name.clone();
            let newalbum=edit_album(og_album, &albums);
            let newname=newalbum.name.clone();
            if newname=="NullA" {return (artists, albums, rankings)}
            if newname=="TerminateProgram" {
                save_data(albums, artists, rankings);
                return (Vec::new(), Vec::new(), Vec::new());
            } 
            albums.remove(albumidx);
            albums.insert(albumidx, newalbum.clone());
            
            //Adds the edited album to its artist's list of albums, 
            //using the original name of the album to find its previous location
            let artistname=&albums[albumidx].artist;
            let find_artist=find_artist(&artists, &artistname.to_lowercase());
            let artist: Artist=artists[find_artist.1].clone();
            let mut newalbums: Vec<Album>=artist.albums;
            let album_in_artist_idx=find_album(&newalbums, &original_name.to_lowercase()).1;
            newalbums.remove(album_in_artist_idx);
            newalbums.insert(album_in_artist_idx, newalbum.clone());
            let newartist: Artist=Artist { name: artist.name, genre: artist.genre, members: artist.members, 
                years_active: artist.years_active, description: artist.description, albums: newalbums};
            artists.remove(find_artist.1);
            artists.insert(find_artist.1, newartist);
            //Inserts the album into its artist's ranking entry
            let ranking_idx=find_ranking(&rankings, &artistname.to_lowercase()).1;
            let ranking_data=rankings[ranking_idx].clone();
            let mut remaining_albums=ranking_data.2.clone();
            let mut ranking=ranking_data.1;
            //Checks to see if the album was ranked or not, and inserts the new version into whichever vector it was located in
            let find_og=find(&remaining_albums, &original_name.to_lowercase());
            if find_og.0==true {
                let og_idx=find_og.1;
                remaining_albums.remove(og_idx);
                remaining_albums.insert(og_idx, newname);
            }
            else {
                let ranking_album_data=find_ranking_album(&ranking, &original_name.to_lowercase());
                let mut newrank=ranking_album_data.2.clone();
                newrank.push_str(&newname);
                ranking.remove(ranking_album_data.1);
                ranking.insert(ranking_album_data.1, newrank);
            }
            rankings.remove(ranking_idx);
            rankings.insert(ranking_idx, (artistname.to_string(), ranking, remaining_albums));
        },
        "delalbum" => {
            //Prevents the user from deleting the last album in the system
            if albums.len()==1 {
                println!("Cannot delete last remaining album in the system");
                return (artists, albums, rankings);
            }
            let mut check_album=(false, 10000);
            let albumidx=loop {
                let albumname: String=add_field("Enter album", "", "");
                if albumname=="NullA" {return (artists, albums, rankings)}
                else if albumname=="TerminateProgram" {
                    save_data(albums, artists, rankings);
                    return (Vec::new(), Vec::new(), Vec::new());
                }
                check_album=find_album(&albums, &albumname.to_lowercase());
                if check_album.0==false {println!("Album not found in database");}
                else {break check_album.1;}
            };
            let album=albums[albumidx].clone();
            let newstructs=delete_album(albums, artists, albumidx);
            artists=newstructs.0;
            albums=newstructs.1;
            let artistname=album.artist.clone();
            let mut idx=0 as usize;
            let find_ranking_data=find_ranking(&rankings, &artistname.to_lowercase()).1;
            //removes the album from its artist's ranking entry
            let ranking_data=rankings[find_ranking_data].clone();
            let mut ranking=ranking_data.1.clone();
            let mut remaining_albums=ranking_data.2.clone();
            //Checks to see if the album was located in the remaining albums or rankings vectors, 
            //removes it from whichever one it belonged to
            let find_remaining=find(&remaining_albums, &album.name.to_lowercase());
            if find_remaining.0==false {
                let rank_idx=find_ranking_album(&ranking, &album.name.to_lowercase()).1;
                ranking.remove(rank_idx);
                //As there will now be a gap of two between the ranks before and after the one which was removed, 
                //alters the ranking entries to remove this gap
                ranking=adjust_ranks(rank_idx, ranking);
            }
            else { 
                remaining_albums.remove(find_remaining.1);
                //Removes the first empty ranking in the ranking vector, then adjusts the rankings to remove the gap
                //This ensures the number of overall rankings in the rankings vector corresponds to the total number of albums
                for i in 0..ranking.len() {
                    let double_digit_true=is_two_digit(&ranking[i]);
                    if double_digit_true==true {
                        if ranking[i].len()<=4 {
                            ranking.remove(i);
                            ranking=adjust_ranks(i, ranking);
                            break;
                        }
                    }
                    else {
                        if ranking[i].len()<=3 {
                            ranking.remove(i);
                            ranking=adjust_ranks(i, ranking);
                            break;
                        }
                    }
                    
                }
            }
            //If this album is the only one possessed by an artist, clears the ranking vector of entries
            let double_digit_true=is_two_digit(&ranking[0]);
            if double_digit_true {
                if remaining_albums.is_empty() && ranking.len()==1 && ranking[0].len()<=4 {ranking.clear();}
            }
            else {
                if remaining_albums.is_empty() && ranking.len()==1 && ranking[0].len()<=3 {ranking.clear();}
            }
            rankings.remove(find_ranking_data);
            rankings.insert(find_ranking_data, (artistname, ranking, remaining_albums));

        },
        "timeline" => get_timeline(&artists),
        "artistlist" => get_artist_list(&artists),
        "albumlist" => get_album_list(&albums),
        "ranking"=> {
            let ranking_info=loop {
                let artistname=add_field("Enter artist to rank", "", "");
                if artistname=="NullA" {return (artists, albums, rankings)}
                else if artistname=="TerminateProgram" {
                    save_data(albums, artists, rankings);
                    return (Vec::new(), Vec::new(), Vec::new());
                }
                let find_ranking=find_ranking(&rankings, &artistname.to_lowercase());
                let ranking_idx=find_ranking.1.clone();
                if find_ranking.0==false {println!("Artist not found in database")}
                else {
                    let ranking=rankings[ranking_idx].1.clone();
                    //Prevents the user from accessing the ranking section for an artist if they have under two albums
                    if ranking.len()!=1 {break (artistname, ranking_idx);}
                    else {println!("An artist must have at least two albums to create a ranking. Please create another album entry for {} to continue", artistname);} 
                }
            };
            //Insertion of edited ranking into rankings
            let ranking_idx=ranking_info.1.clone();
            let artistname=ranking_info.0.clone();
            let ranking=rankings[ranking_idx].1.clone();
            let remaining_albums=rankings[ranking_idx].2.clone();
            let new_ranking_vecs:(Vec<String>, Vec<String>, String)=album_ranking(ranking, remaining_albums);
            let new_ranking: (String, Vec<String>, Vec<String>)=(artistname, new_ranking_vecs.0.clone(), new_ranking_vecs.1.clone());
            rankings.remove(ranking_idx);
            rankings.insert(ranking_idx, new_ranking);
            if new_ranking_vecs.2=="TerminateProgram" {
                save_data(albums, artists, rankings);
                return (Vec::new(), Vec::new(), Vec::new());
            }
        },
        "getranking" => {
            //Prints an artists rankings
            let ranking_idx=loop {
                let artistname=add_field("Enter artist", "", "");
                if artistname=="NullA" {return (artists, albums, rankings)}
                else if artistname=="TerminateProgram" {
                    save_data(albums, artists, rankings);
                    return (Vec::new(), Vec::new(), Vec::new());
                }
                let find_ranking=find_ranking(&rankings, &artistname.to_lowercase());
                let ranking_idx=find_ranking.1.clone();
                if find_ranking.0==false {println!("Artist not found in database")}
                else {break ranking_idx}
            };
            let ranking=rankings[ranking_idx].clone();
            get_album_ranking(&ranking);
        }
        "searchalbum" => {
            let exitline=search_album_by(&albums);
            if exitline=="TerminateProgram" {
                save_data(albums, artists, rankings);
                return (Vec::new(), Vec::new(), Vec::new());
            }
        }
        "searchartist" => {
            let exitline=search_artist_by(&artists);
            if exitline=="TerminateProgram" {
                save_data(albums, artists, rankings);
                return (Vec::new(), Vec::new(), Vec::new());
            }
        }
        "help"=> {
            //Prints available commands for the main section
            println!("addartist        editartist        delartist");
            println!("addalbum         editalbum         delalbum");
            println!("timeline         albumlist         artistlist");
            println!("ranking          getranking        searchartist");
            println!("searchalbum      ex                save");
            println!("");
            println!("If you want to know what one of the above commands does, enter the command followed by '?', e.g. addalbum?");
            println!("To view information about an artist (genre, description, etc.) type / followed by that artist's name, e.g. /The Beatles");
            println!("To view information about an album (genre, description, etc.) type // followed by that album's name, e.g. //The Wall");
            println!("ex saves entered data and exits program. Data will not be automatically saved if program is closed via other means");
        }
        "save" => {
            save_data(albums.clone(), artists.clone(), rankings.clone());
        },
        "load" => {
            load_data();
        }
        _=> {if entry_search==false {println!("Command not recognised")}}
    }
    return (artists, albums, rankings)
}

fn add_artist(artists: &Vec<Artist> )-> Artist {
    /*
    Description: 
    Prompts the user to input information for a new artist, which is saved to the system in open_section()
    Parameters: 
    artists: &Vec<Artist>-A reference to the main artists vector. Used to check if the artist being entered already exists in the system
    Returns:
    Artist-A new instance of the artist data structure
    */
    //Gets the current year for validation
    let current_date: DateTime<chrono::Utc>=chrono::Utc::now();
    let current_year: i32=current_date.year() as i32;
    'main: loop {
        //Section for entering name, genre, years active, and description. Handled in a loop to easily allow for a redo action
        let details: (String, String, (i32, i32), String)=loop {
        let name: String=add_field("Enter artist name", "", "");
        if name==String::from("") {continue;}
        if name==String::from("NullA") {return Artist::default()}
        if name==String::from("TerminateProgram") {return create_ex_artist()}
        //Checks to see if the artist exists withi the system and prompts a name re-entry if they do
        //Many such instances of this operation
        let name_exists=find_artist(&artists, &name.to_lowercase()).0;
        if name_exists==true {
            println!("Artist has already been added");
            continue;
        }
        let name_len=name.len();
        //Prevents the user adding a name which is over 30 characters, as the timeline function is based around this
        //Restrictive though so I might change this later
        //Many such instances of this validation, only difference being lengths
        if name_len>30 {
            println!("Artist name must be at most 30 characters");
            continue;
        }
        let genre: String=loop {
            let genre=add_field("Enter artist genre", "", "");
            if genre.len()>40 {
                println!("Please enter a genre name which is 40 characters or less");
            }
            else {break genre;}
        };
        if genre==String::from("") {continue;}
        if genre==String::from("NullA") {return Artist::default()}
        if genre==String::from("TerminateProgram") {return create_ex_artist()}
        let startyear: i32=loop {
            let year: i32=add_field("Enter year first active", "i32", "").parse().unwrap();
            //As the year variable is an i32, the usual command strings cannot be used, 
            //instead negative numbers are employed for each of these as they are not valid as user input anywhere in the system
            if year!=0 && year!=-1 && year!=-2 {
                if year==-3 {continue}
                //Ensures user enters a valid starting year
                //Initially the min year was 1900, turns out people get really damn old
                //Apparently there was a guy called Henry Busser who's first composition was made 'before 1890'
                //and he was still making stuff by the mid-sixties
                //So if anyone wants to add him feel free
                if year<1889 || year>1980 {
                    println!("Please enter a starting year between 1889 and 1980");
                    continue
                }
            }
            break year;
        };
        if startyear==0 {continue;}
        if startyear==-1 {return Artist::default()}
        if startyear==-2 {return create_ex_artist()}
        let endyear: i32=loop {
            let year: i32=add_field("Enter year last active", "i32", "").parse().unwrap();
            if year!=0 && year!=-1 && year!=-2 {
                if year==-3 {continue}
                else if year<1960 || year>current_year {
                    println!("Please enter an ending year between 1960 and 2025");
                    continue
                }
                //Ensures users don't enter an invalid end year based on the start year
                else if year<startyear {
                    println!("Please enter an ending year which is after or equal to the starting year");
                    continue
                }
            }
            break year;
        };
        if endyear==0 {continue;}
        if endyear==-1 {return Artist::default()}
        if startyear==-2 {return create_ex_artist()}
        let desc: String=loop {
            let desc=add_field("Enter artist description", "", "");
            if desc.len()>2000 {println!("Please enter a description which is 2000 characters or less")}
            else {break desc;}
        };
        if desc==String::from("") {continue;}
        if desc==String::from("NullA") {return Artist::default()}
        if desc==String::from("TerminateProgram") {return create_ex_artist()}
        let details: (String, String, (i32, i32), String)=(name, genre, (startyear, endyear), desc);
        break details
        };

        let mut member: String=String::from("");
        let mut members: Vec<String>=Vec::new();
        println!("If the artist is a band, enter their members. If the artist is a solo artist, type 'Solo Artist'");
        println!("type 'end' to finish adding members");
        loop {
            //Prompts user to add members until they type end
            member=add_field("Enter Member", "", "members");
            //If redo is typed and members have been added, removes the added members
            //If redo is typed while there are no members, returns to the start of the function
            //Same functionality is seen in the editartist function
            if member==String::from("") {
                if members.is_empty()==false {
                    members.clear();
                    println!("Pending members entries cleared.");
                    println!("Type 'redo' again to restart artist entry, or type another member to continue");
                }
                else {continue 'main}
            }
            else if member==String::from("NullA") {return Artist::default()}
            else if member==String::from("end") {
                //Ensures user enters at least one member
                if members.is_empty()==false {break;}
                println!("Please enter at least one member (if entry is a solo artist, type 'Solo Artist')")
            }
            else if member==String::from("TerminateProgram") {return create_ex_artist()}
            //Ensures user doesn't enter more than 15 members
            else if members.len()==15 {
                println!("Maximum number of members reached, type 'end' to finish");
                println!("Please refrain from adding members which were not a part of the band between 1960-1980");
            }
            else if member.len()>50 {println!("Please enter a member with 50 characters or less")}
            else {members.push(member.clone());}
        }
        return Artist { name: (details.0), genre: (details.1), members: (members),
            years_active: (details.2), description: (details.3), albums: (Vec::new())}
    }
}
    
fn add_album(artistname: String, albums: Vec<Album>)->Album {
    /*
    Description: 
    Prompts the user to input information for a new album, which is saved to the system in open_section()
    Parameters: 
    albums: &Vec<albums>-A reference to the main albums vector. Used to check if the artist being entered already exists in the system
    artistname: String-The name of the artist the album was created by, used as one of the album fields
    Returns:
    Album-A new instance of the album data structure
    */
    //This is mostly the same as addartist
    let details: (String, String, String, i32,  String, String)='main: loop {
    let name: String=add_field("Enter album name", "", "");
    if name==String::from("") {continue;}
    if name==String::from("NullA") {return Album::default()}
    if name==String::from("TerminateProgram") {return create_ex_album()}
    let album_exists=find_album(&albums, &name.to_lowercase()).0;
    if album_exists==true {
        println!("Album has already been added");
        continue;
    }
    let name_len=name.len();
    if name_len>25 {
        println!("Please enter an album name which is 25 characters or less");
        continue;
    }
    let genre: String=loop {
        let genre=add_field("Enter album genre", "", "");
        if genre.len()>40 {
            println!("Please enter a genre which is 40 characters or less");
        }
        else {break genre;}
    };
    if genre==String::from("") {continue;}
    if genre==String::from("NullA") {return Album::default()}
    if genre==String::from("TerminateProgram") {return create_ex_album()}
    //Ensures user enters one of four valid formats, then case-corrects that entry
    let formattypes: [String; 4]=[String::from("lp"), String::from("ep"), String::from("double"), String::from("triple")];
    let mut format: String=loop {
        let format_check=add_field("Enter album format (either LP, EP, Double, or Triple)", "", "");
        if format_check==String::from("") {continue 'main;}
        if format_check==String::from("NullA") {return Album::default()}
        if format_check==String::from("TerminateProgram") {return create_ex_album()}
        if formattypes.contains(&format_check.to_lowercase()) {break format_check}
        println!("Please enter a valid format; LP, EP, Double, or Triple")
    };
    if format.to_lowercase()=="lp" || format.to_lowercase()=="ep" {format=format.to_uppercase()}
    if format.to_lowercase()=="double" {format=String::from("Double")}
    if format.to_lowercase()=="triple" {format=String::from("Triple")}
    let releaseyear: i32=loop {
        let year: i32=add_field("Enter release year", "i32", "").parse().unwrap();
        if year!=0 && year!=-1 && year!=-2 {
            if year==-3 {continue}
            if year<1960 || year>1980 {
                println!("Please enter a release year between 1960 and 1980");
                continue
            }
        }
        break year;
    };
    if releaseyear==0 {continue;}
    if releaseyear==-1 {return Album::default()}
    if releaseyear==-2 {return create_ex_album()}
    let recorddates: String=loop {
        let record_dates=add_field("Enter album record dates", "", "");
        if record_dates.len()>50 {
            println!("Please enter record dates which are 50 characters or less");
        }
        else {break record_dates;}
    };
    if recorddates==String::from("") {continue;}
    if recorddates==String::from("NullA") {return Album::default()}
    if recorddates==String::from("TerminateProgram") {return create_ex_album()}
    let desc: String=loop {
        let desc=add_field("Enter album description", "", "");
        if desc.len()>2000 {
            println!("Please enter a description which is 2000 characters or less");
        }
        else {break desc;}
    };
    if desc==String::from("") {continue;}
    if desc==String::from("NullArtist") {return Album::default()}
    if desc==String::from("TerminateProgram") {return create_ex_album()}
    let details: (String, String, String, i32,  String, String)=(name, genre, format, releaseyear, recorddates, desc);
    break details
    };

    return Album {name: details.0, genre: details.1, artist: artistname, format: details.2, 
        release_year: details.3, record_dates: details.4, description: details.5}
}

fn edit_artist(artist: Artist, artists: &Vec<Artist>)-> Artist {
    /*
    Description: 
    Prompts the user to edit one of the fields for an existing artist, which is saved to the system in open_section()
    Parameters: 
    artists: &Vec<Artist>-A reference to the main artists vector. Used to check if the artist name being entered already exists in the system
    artist: Artist-The existing artist entry to be altered
    Returns:
    Artist-A copy of the entered artist entry with one of the fields changed
    */
    let current_date: DateTime<chrono::Utc>=chrono::Utc::now();
    let current_year: i32=current_date.year() as i32;
    //loops until a new field entry is correctly given or the user inputs 'return'
    let newartist='main: loop {
        //Allows user to input artist field to edit, then based on that prompts them to enter the new field entry
        let field=add_field("Enter field to edit", "", "editartist");
        if field== "NullA" {return artist}
        if field=="TerminateProgram" {return  create_ex_artist();}
        let newartist=match field.to_lowercase().trim() {
            "name" => {
                let new_entry=loop {
                    let name=add_field("Enter new name", "", "");
                    if name=="" {continue 'main;}
                    if name==String::from("NullA") {return artist}
                    if name=="TerminateProgram" {return create_ex_artist()}
                    let name_len=name.len();
                    if name_len>30 {
                        println!("Artist name must be at most 30 characters");
                        continue;
                    }
                    let artist_exists=find_artist_nlc(&artists, &name).0;
                    if artist_exists==true {
                        println!("Artist already exists in database");
                        continue;
                    }
                    break name
                };
                println!("Name has been changed");
                Artist { name: new_entry, genre: artist.genre, years_active: artist.years_active, 
                        members: artist.members, albums: artist.albums,
                        description: artist.description}
            }
            "genre" => {
                let new_entry: String=loop {
                    let new_entry=add_field("Enter new genre", "", "");
                    if new_entry.len()>40 {
                        println!("Please enter a genre which is 40 characters or less");
                    }
                    else {break new_entry;}
                };
                if new_entry=="" {continue}
                if new_entry==String::from("NullA") {return artist}
                if new_entry=="TerminateProgram" {return create_ex_artist()}
                println!("Genre has been changed");
                Artist { name: artist.name, genre: new_entry, years_active: artist.years_active, 
                    members: artist.members, albums: artist.albums,
                    description: artist.description}
            }
            "years active" => {
                let startyear: i32=loop {
                    let year: i32=add_field("Enter new start year", "i32", "").parse().unwrap();
                    if year!=0 && year!=-1 && year!=-2 {
                        if year==-3 {continue}
                        if year<1889 || year>1980 {
                            println!("Please enter a starting year between 1889 and 1980");
                            continue
                        }
                    }
                    break year;
                };
                if startyear==0 {continue;}
                if startyear==-1 {return artist}
                if startyear==-2 {return create_ex_artist()}
                let endyear: i32=loop {
                    let year: i32=add_field("Enter new end year", "i32", "").parse().unwrap();
                    if year!=0 && year!=-1 && year!=-2 {
                        if year==-3 {continue}
                        if year<1960 || year>current_year {
                            println!("Please enter an ending year between 1960 and 2025");
                            continue
                        }
                    }
                    break year;
                };
                if endyear==0 {continue;}
                if endyear==-1 {return artist}
                if endyear==-2 {return create_ex_artist()}
                let new_entry=(startyear, endyear);
                println!("Active years have been changed");
                Artist { name: artist.name, genre: artist.genre, years_active: new_entry, 
                    members: artist.members, albums: artist.albums,
                    description: artist.description}
            }
            "members" => {
                println!("Type 'append' or 'clear'");
                let new_artist=loop {
                    let change_type=add_field("Append or clear all and readd?", "", "editmembers");
                    let mut members=Vec::new();
                    if change_type=="append" {members=artist.members.clone();}
                    else if change_type=="clear" {}
                    else {println!("Command not recognised. Type 'append' or 'clear'.");}
                    if change_type=="append" || change_type=="clear" {
                        let mut member: String=String::from("");
                        loop {
                            println!("type 'end' to finish adding members");
                            member=add_field("Enter Member", "", "members");
                            if member==String::from("") {
                                if members.is_empty()==false {
                                    members.clear();
                                    println!("Pending members entries cleared.");
                                    println!("Type 'redo' again to restart artist entry, or type another member to continue");
                                }
                                else {continue 'main}
                            }
                            else if member==String::from("NullA") {return artist}
                            else if member==String::from("end") {
                                if members.is_empty()==false {break;}
                                println!("Please enter at least one member (if entry is a solo artist enter their name)")
                            }
                            else if member==String::from("TerminateProgram") {return create_ex_artist()}
                            else if members.len()==15 {println!("Maximum number of members reached, type 'end' to finish")}
                            else if member.len()>50 {println!("Please enter a member with 50 characters or less")}
                            else {members.push(member.clone());}
                        }
                        println!("Members have been changed");
                        break Artist { name: artist.name, genre: artist.genre, years_active: artist.years_active, 
                            members: members, albums: artist.albums,
                            description: artist.description}
                    }

                };
                new_artist
            }
            "description" => {
                let new_entry: String=loop {
                    let new_entry=add_field("Enter new description", "", "");
                    if new_entry.len()>2000 {
                        println!("Please enter a description which is 2000 characters or less");
                    }
                    else {break new_entry;}
                };
                if new_entry=="" {continue}
                if new_entry==String::from("NullA") {return artist}
                if new_entry=="TerminateProgram" {return create_ex_artist()}
                println!("Description has been changed");
                Artist { name: artist.name, genre: artist.genre, years_active: artist.years_active, 
                    members: artist.members, albums: artist.albums,
                    description: new_entry}
            }
            //If the user inputs an artist field which doesn't exist, informs them of their error then restarts the loop
            "albums" => {
                println!("An artist's albums cannot be edited from the editartists section. Please use the editalbums function instead");
                continue;
            }
            _=> {
                println!("field does not exist");
                continue;
            }
        };
        break newartist
    };
    return newartist;
}


fn edit_album(album: Album, albums: &Vec<Album>) -> Album {
    /*
    Description: 
    Prompts the user to edit one of the fields for an existing album, which is saved to the system in open_section()
    Parameters: 
    albums: &Vec<Album>-A reference to the main albums vector. Used to check if the album name being entered already exists in the system
    album: Artist-The existing album entry to be altered
    Returns:
    Album-A copy of the entered album entry with one of the fields changed
    */
    //Basically the same comments from addalbum and editartist apply here
    let newalbum='main: loop {
        let field=add_field("Enter field to edit", "", "editalbum");
        if field== "NullA" {return album}
        if field=="TerminateProgram" {return  create_ex_album();}
        let newalbum: Album=match field.to_lowercase().trim() {
            "name" => {
                let new_entry=loop {
                    let name=add_field("Enter new name", "", "");
                    if name=="" {continue 'main;}
                    if name==String::from("NullA") {return album}
                    if name=="TerminateProgram" {return create_ex_album()}
                    let name_len=name.len();
                    if name_len>30 {
                        println!("Album name must be at most 25 characters");
                        continue;
                    }
                    let album_exists=find_album_nlc(albums, &name).0;
                    if album_exists==true {
                        println!("Album already exists in database");
                        continue;
                    }
                    break name
                    };
                println!("Name has been changed");
                Album { name: new_entry, genre: album.genre, format: album.format, 
                    artist: album.artist, release_year: album.release_year, record_dates: album.record_dates,
                    description: album.description}
                }
            "genre" => {
                let new_entry: String=loop {
                    let new_entry=add_field("Enter new genre", "", "");
                    if new_entry.len()>40 {
                        println!("Please enter a genre which is 40 characters or less");
                    }
                    else {break new_entry;}
                };
                if new_entry=="" {continue}
                if new_entry==String::from("NullA") {return album}
                if new_entry=="TerminateProgram" {return create_ex_album()}
                println!("Genre has been changed");
                Album { name: album.name, genre: new_entry, format: album.format, 
                    artist: album.artist, release_year: album.release_year, record_dates: album.record_dates,
                    description: album.description}
                }
            "format" => {
                let formattypes: [String; 4]=[String::from("LP"), String::from("EP"), String::from("Double"), String::from("Triple")];
                let mut new_format=loop {
                    let new_entry=add_field("Enter new format", "", "");
                    if new_entry=="" {continue}
                    if new_entry=="NullA" {return album}
                    if new_entry=="TerminateProgram" {return create_ex_album()}
                    if formattypes.contains(&new_entry) {break new_entry}
                    println!("Please enter a valid format; LP, EP, Double, or Triple")
                };
                if new_format.to_lowercase()=="lp" || new_format.to_lowercase()=="ep" {new_format=new_format.to_uppercase()}
                if new_format.to_lowercase()=="double" {new_format=String::from("Double")}
                if new_format.to_lowercase()=="triple" {new_format=String::from("triple")}
                println!("Format has been changed");
                Album { name: album.name, genre: album.genre, format: new_format, 
                    artist: album.artist, release_year: album.release_year, record_dates: album.record_dates,
                    description: album.description}
                }
            "release year" => {
                let new_entry: i32=loop {
                    let year: i32=add_field("Enter release year", "i32", "").parse().unwrap();
                    if year!=0 && year!=-1 && year!=-2 {
                        if year==-3 {continue}
                        else if year<1960 || year>1980 {
                            println!("Please enter an release year between 1960 and 1980");
                            continue
                        }
                        else {break year;}
                    }
                };
                if new_entry==0 {continue}
                if new_entry==-1 {return album}
                if new_entry==-2 {return create_ex_album()}
                println!("Release year has been changed");
                Album { name: album.name, genre: album.genre, format: album.format, 
                    artist: album.artist, release_year: new_entry, record_dates: album.record_dates,
                    description: album.description}
                }
            "record dates" => {
                let new_entry: String=loop {
                    let new_entry=add_field("Enter new record dates", "", "");
                    if new_entry.len()>50 {
                        println!("Please enter record dates which are 50 characters or less");
                    }
                    else {break new_entry;}
                };
                if new_entry=="" {continue}
                if new_entry==String::from("NullA") {return album}
                if new_entry=="TerminateProgram" {return create_ex_album()}
                println!("Recording dates have been changed");
                Album { name: album.name, genre: album.genre, format: album.format, 
                    artist: album.artist, release_year: album.release_year, record_dates: new_entry,
                    description: album.description}
                }
            "description" => {
                let new_entry: String=loop {
                    let new_entry=add_field("Enter new description", "", "");
                    if new_entry.len()>2000 {
                        println!("Please enter a description which is 2000 characters or less");
                    }
                    else {break new_entry;}
                };
                if new_entry=="" {continue}
                if new_entry==String::from("NullA") {return album}
                if new_entry=="TerminateProgram" {return create_ex_album()}
                println!("Description has been changed");
                Album { name: album.name, genre: album.genre, format: album.format, 
                    artist: album.artist, release_year: album.release_year, record_dates: album.record_dates,
                    description: new_entry}
                }
            "NullA" => {return album}
            "TerminateProgram" => {return create_ex_album()}
            _ => {
                println!("field does not exist");
                continue
            }
        };
        break newalbum;
    };
    return newalbum
}

fn delete_artist(mut artists: Vec<Artist>, mut albums: Vec<Album>, mut rankings: Vec<(String, Vec<String>, Vec<String>)>, idx: usize) 
-> (Vec<Artist>, Vec<Album>, Vec<(String, Vec<String>, Vec<String>)>) {
    /*
    Description: 
    Deletes all information pertaining to an existing artist from the system
    Parameters:
    artists: Vec<Artist>-One of the primary data structures, a vector of every artist in the system
    albums: Vec<Artist>-One of the primary data structures, a vector of every album in the system
    rankings: Vec<(String, Vec<String>, Vec<String>)>-One of the primary data structures, 
    a vector storing the rankings and unranked albums for every artist, keyed by artist name. 
    idx: usize-The index in artists where the artist entry to be deleted is
    Returns:
    (Vec<Artist>, Vec<Album>, Vec<(String, Vec<String>, Vec<String>)>): A tuple containing the altered data for artists, albums, and rankings
    */
    let artist=artists[idx].clone();
    println!("{} has been deleted", artist.name);
    //Removes artist entry from artists, removes all of their albums from albums, and deletes their ranking as well
    artists.remove(idx);
    for i in 0..albums.len()-1 as usize {
        if i==albums.len() {break}
        while albums[i].artist.to_lowercase()==artist.name.to_lowercase() {
            albums.remove(i);
        }
    }
    for i in 0..rankings.len()-1 as usize {
        let r=rankings[i].0.clone();
        if rankings[i].0.to_lowercase()==artist.name.to_lowercase() {rankings.remove(i);}
    }
    return (artists, albums, rankings);
}

fn delete_album(mut albums: Vec<Album>, mut artists: Vec<Artist>, idx: usize) -> (Vec<Artist>, Vec<Album>) {
    /*
    Description: 
    Deletes all information pertaining to an existing album from the system
    Parameters:
    artists: Vec<Artist>-One of the primary data structures, a vector of every artist in the system
    albums: Vec<Artist>-One of the primary data structures, a vector of every album in the system
    idx: usize-The index in albums where the album entry to be deleted is
    Returns:
    (Vec<Artist>, Vec<Album>): A tuple containing the altered data for artists and albums
    */
    let album=albums[idx].clone();
    let artists_len=artists.len();
    println!("{} has been deleted", album.name);
    albums.remove(idx);
    //Finds the album's artist, then , removes it, then 
    for i in 0..artists_len {
        if artists[i].name.to_lowercase()==album.artist.to_lowercase() {
            let mut new_artist_albums=artists[i].albums.clone();
            let oldartist=artists[i].clone();
            //finds the album entry in that artist's albums
            for n in 0..new_artist_albums.len() {
                if new_artist_albums[n].name==album.name {
                    //removes it and creates a new artist instance with the album removed
                    new_artist_albums.remove(n);
                    let newartist: Artist=Artist { name: oldartist.name.clone(), genre: oldartist.genre.clone(), members: oldartist.members.clone(),
                         years_active: oldartist.years_active.clone(), description: oldartist.description.clone(), albums: new_artist_albums};
                    //replaces the old artist with the new artist
                    artists.remove(i);
                    artists.insert(i, newartist);
                    return (artists, albums);
                }
            }
        }
    }
    return (artists, albums);
}

fn album_ranking(mut ranking: Vec<String>, mut remaining_albums: Vec<String>) -> (Vec<String>, Vec<String>, String) {
    /*
    Description: 
    Allows the user to rank an artists albums from worst to best. Any change made is saved when the program is closed
    Parameters:
    ranking: Vec<String>-A vector containing the ranking for the current artist. 
    remaining_albums: Vec<String>- A vector containing the albums which have yet to be added to the ranking vector.
    Returns:
    (Vec<String>, Vec<String>, String)-A tuple containing the data for the  altered ranking and remaining albums structures,
    in addition to an 'exitline' string, which is used to handle the user exiting the program from inside this function
    */
    //BIG variable initialisation
    let mut exitline=String::new();
    let mut remaining_albums_count=remaining_albums.len() as i32;
    let mut rank=1;
    let mut og_ranking=Vec::new();
    let mut og_remaining_albums=remaining_albums.clone();
    let mut cont_true=true;
    if remaining_albums.is_empty()==true {cont_true=false;}
    let ranksleft=ranking.len()-remaining_albums.len();
    ranking=ranking_whitespace_plaster(ranking);
    //Populates og_remaining albums with every album the artist has
    for i in ranking.clone() {
        let ranklen=i.len();
        //Because this involves stringdexing from a specific character, 
        //it has to be done slightly differently if the ranking is single/double digit,
        //hence, variations on this sub-routine based on this factor
        //Another thing which comes up repeatedly
        let double_digit_true=is_two_digit(&i);
        if double_digit_true {
            if ranklen>4 {
                let mut rank_album;
                rank_album=i.as_str()[4..ranklen].trim().to_string();
                og_remaining_albums.push(rank_album);
            }
        }
        else {
            if ranklen>3 {
                let mut rank_album;
                rank_album=i.as_str()[3..ranklen].trim().to_string();
                og_remaining_albums.push(rank_album);
            }
        }
    }
    //Populates og_ranking with a number of empty ranks equal to the total count of albums in the ranking
    for i in 1..og_remaining_albums.len()+1 {
        let mut rankstr=i.to_string();
        rankstr.push_str(". ");
        og_ranking.push(rankstr);
    }
    'main: loop {
        //Prints the ranked and unranked albums in an organised manner
        //The unranked albums will always be 30 characters in, 
        //and the two sections are split by a vertical line of '|'s 
        //(what are these called? The program wouldn't work without them but I haven't bothered to look it up)
        println!("rank                         unranked");
        for i in 0..remaining_albums.len() as usize {
            let r=ranking[i].clone();
            let ranklen=ranking[i].len();
            let gap=29-ranklen;
            let mut rankstr=ranking[i].clone();
            for n in 0..gap {rankstr.push(' ');}
            rankstr.push('|');
            rankstr.push_str(&remaining_albums[i].clone());
            println!("{}", rankstr);
        }
        let ranksleft=ranking.len()-remaining_albums.len();
        let rankidx=ranking.len()-ranksleft;
        //Second loop which handles printing of ranked albums past the number of unranked albums
        for i in rankidx..ranking.len() {
            let ranklen=ranking[i].len();
            let gap=29-ranklen;
            let mut rankstr=ranking[i].clone();
            for n in 0..gap {rankstr.push(' ');}
            rankstr.push('|');
            //If there are no unranked albums, prints 'None'
            if i==0 {
                println!("{}None", rankstr);
                continue
            }
            println!("{}", rankstr);
        }
        let albumname=add_field("Enter album to place", "", "");
        if albumname=="TerminateProgram" {return (ranking, remaining_albums, String::from("TerminateProgram"));}
        if albumname=="NullA" {break 'main;}
        if albumname=="" {
            remaining_albums=og_remaining_albums.clone();
            ranking=og_ranking.clone();
            remaining_albums_count=remaining_albums.len() as i32;
            continue;
        }
        let find_remaining=find(&remaining_albums, albumname.to_lowercase().as_str());
        let find_album=find(&og_remaining_albums, albumname.to_lowercase().as_str());
        //Routine for if the album entered is unranked
        if find_remaining.0==true {
            //This is used in case the user enters an the name of an album case-incorrectly
            //So when it is inserted the correcting casing is preserved, rather than using the input itself
            let true_albumname=remaining_albums[find_remaining.1].clone();
            loop {
                let placement: i32=add_field("Enter album rank", "i32", "").parse().unwrap();
                if placement==-3 {continue}
                if placement==-2 {return (ranking, remaining_albums, String::from("TerminateProgram"));}
                if placement==-1 {break 'main;}
                //Redo routine which resets the ranking, placing all albums in unranked
                //and making all the ranked elements empty ranks
                if placement==0 {
                    remaining_albums=og_remaining_albums.clone();
                    ranking=og_ranking.clone();
                    remaining_albums_count=remaining_albums.len() as i32;
                    continue 'main;
                }
                //Check for valid placement, can't be less than 1 or more than the total number of albums in the ranking
                if placement>0 && placement<=og_remaining_albums.len() as i32 {
                    let mut new_rank=String::from("");
                    //Gets the index of the entered placement in ranked albums, which because of how arrays work
                    //will be one less than the entered placement
                    let pi=placement as usize;
                    let placement_idx=pi-1;
                    //Gets the index of the album in remaining albums
                    let album_idx=find(&remaining_albums, &albumname.to_lowercase()).1;
                    let rank_placement=ranking[placement_idx].clone();
                    let double_digit_true=is_two_digit(&rank_placement);
                    let mut album_placed_true=true;
                    //Checks if there is already an album at the placed position
                    if double_digit_true==true {
                        if rank_placement.len()<=4 {
                            new_rank=rank_placement.clone();
                            album_placed_true=false;
                        }
                    }
                    else {
                        if rank_placement.len()<=3 {
                            new_rank=rank_placement.clone();
                            album_placed_true=false;
                        }
                    }
                    if album_placed_true==true {
                        //Places the entered album into the rank and returns the previously placed album back to unranked
                        if double_digit_true==true {new_rank=rank_placement.as_str()[0..3].trim().to_string(); }
                        else {new_rank=rank_placement.as_str()[0..2].trim().to_string();}
                        new_rank.push(' ');
                        let rank_placement_len=rank_placement.len();
                        let removed_album=rank_placement.as_str()[3..rank_placement_len].trim().to_string();
                        remaining_albums.push(removed_album);
                    }
                    new_rank.push_str(&true_albumname);
                    remaining_albums.remove(album_idx);
                    ranking.remove(placement_idx);
                    ranking.insert(placement_idx, new_rank);
                    remaining_albums_count-=1;
                    break
                }
                else {println!("Please enter a valid rank")}
            }
        }
        //Routine for if the album entered is ranked
        else if find_album.0==true {
            let true_albumname=og_remaining_albums[find_album.1].clone();
            loop {
                let placement: i32=add_field("Enter new album rank", "i32", "").parse().unwrap();
                if placement==-3 {continue}
                if placement==-2 {return (ranking, remaining_albums, String::from("TerminateProgram"));}
                if placement==-1 {break 'main;}
                if placement==0 {
                    remaining_albums=og_remaining_albums.clone();
                    ranking=og_ranking.clone();
                    remaining_albums_count=remaining_albums.len() as i32;
                    continue 'main;
                }
                let ni=placement as usize;
                let newrank_idx=ni-1;
                let currentrank_idx=find_ranking_album(&ranking, &albumname.to_lowercase()).1;
                let currententry=ranking[currentrank_idx].clone();
                //gets the current number at the start of the entered album
                let currentrank=get_rankno(&currententry);
                //Additional clause here for placement validation to ensure the album isn't placed in the same place
                if placement>0 && placement<=og_remaining_albums.len() as i32 && placement!=currentrank {
                    let newrank_entry=ranking[newrank_idx].clone();
                    let ranklen=newrank_entry.len();
                    let double_digit_true=is_two_digit(&newrank_entry);
                    //Gets the album to be moved back into unranked if the placement index is populated
                    let mut removed_album=String::new();
                    if double_digit_true==true {removed_album=newrank_entry.as_str()[4..ranklen].trim().to_string();}
                    else {removed_album=newrank_entry.as_str()[3..ranklen].trim().to_string();}
                    //Constructs the rank string for the album to be placed 
                    //as well as the empty rank for the position which it used to occupy
                    let mut new_rank=get_rankno(&newrank_entry).to_string();
                    let mut empty_rank=currentrank.to_string();
                    empty_rank.push_str(". ");

                    remaining_albums.push(removed_album);
                    new_rank.push_str(". ");
                    new_rank.push_str(&true_albumname);

                    ranking.remove(currentrank_idx);
                    if currentrank_idx<ranking.len() {
                        ranking.insert(currentrank_idx, empty_rank);
                    }
                    else {ranking.push(empty_rank);}

                    ranking.remove(newrank_idx);
                    if newrank_idx<ranking.len() {
                        ranking.insert(newrank_idx, new_rank);
                    }
                    else {ranking.push(new_rank);}
                    
                    remaining_albums_count+=1;
                    break
                }
                else {println!("Please enter a valid rank")}
            }
        }
        //Error statement for if the album entered does not exist in either vector
        else {println!("Please enter a listed album");}
        //Once all albums have been placed, prints a message letting the user know their options
        //Does not occur if the ranking was initally full
        if remaining_albums.len()==0 && cont_true==true {
            println!("Ranking completed, type 'return' to conclude or enter an existing album name to continue editing");
        }
    }
    let new_ranking=(ranking, remaining_albums, exitline);
    return new_ranking;
}

fn get_timeline(artists: &Vec<Artist>) {
    /*
    Description: 
    Prints a timeline of every album released by every artist in the system from 1960-1980
    Parameters:
    artists: &Vec<Artist>-The primary artists data structure where the artist and album names are sourced from for the printing
    Returns:
    None
    */
    let mut year: i32=1960;
    let mut missed_albums: Vec<Album>=Vec::new();
    //Loops 6 times as there are six lines of years
    //I wanted to have it be one line but the terminal couldn't handle it grrrrr
    for i in 0..6 {
        //Prints the four years for the current line
        //Or if the year is 1980 prints that as the only year in the line
        let ogyear=year;
        print!("                              ");
        if year!=1980 {
            for i in 0..4 {
            print!("{}                         ", year);
            year+=1;
            }
        }
        else {
            print!("{}                         ", year);
            year+=1;
        }
        
        println!("");
        for n in artists {
            //Initialises variables for current artist and starts the current printed line with their name 
            let albums=n.albums.clone();
            let mut album_str=String::from("");
            let mut whitespace_str: String=add_whitespace(&n.name, 30);
            let mut last_albumname_len: usize=0;
            let mut line_albums_count=0;
            let mut last_release_year=1900;
            print!("{}", n.name);
            //Adds gap to string so albums from the first year of the line will be aligned
            album_str.push_str(whitespace_str.as_str());
            //Loops through albums and prints the ones which came out in one of the years on the current line
            for j in albums {   
                if j.release_year<year && j.release_year>=ogyear {
                    if j.release_year!=last_release_year {
                        //If the current album is the first album of its artist on its line, 
                        //adds a gap to the beginning based on the position of its release year in the line
                        if last_albumname_len==0 {
                            let mut gap=j.release_year-ogyear;
                            let mut gap_str=add_whitespace("", 30);
                            for k in 0..gap {album_str.push_str(&gap_str);}
                            //Weird adjustment thing this needs to align properly (I don't like this function)
                            for k in 0..gap {album_str.pop();}
                        }
                        //If there are preceeding albums, defines the gap at the beginning by the length of the preceeding albums
                        //Adds to the gap if there is more than a years difference in the release of the previous and current album
                        else {
                            let yeardif=j.release_year-last_release_year-1;
                            let mut gap=30-last_albumname_len;
                            let yeargap_str=add_whitespace("", 30);
                            let mut gap_str=add_whitespace("", gap);
                            album_str.push_str(&gap_str);
                            if yeardif>0 {
                                for i in 0..yeardif {
                                    album_str.push_str(&yeargap_str);
                                    album_str.pop();
                                }
                            }
                            album_str.pop();
                        }
                        //Pushes the album name on the string to be printed and updates the variables tracking 
                        //line album count and previous release year
                        album_str.push_str(j.name.as_str().trim());
                        last_albumname_len=j.name.len();
                        line_albums_count+=1;
                        last_release_year=j.release_year;
                    }
                    //If the release year of the current album is the same as the previous album by that artist, 
                    //adds it to a vector of albums to be printed later
                    else {missed_albums.push(j.clone());}
                }  

            }
            print!("{}", album_str);
            println!("")
        }
    }
    //Prints the albums which were missed from the timeline due to 
    //being released in the same year as another of their albums
    //Ordered by artist and release year
    if missed_albums.is_empty()==false {
        let mut lastartist=String::new();
        let mut lastyear=1900;
        println!("");
        print!("Artists with multiple albums in a year");
        for i in missed_albums {
            if i.artist!=lastartist {
                println!("");
                print!("{}:", i.artist);
                lastyear=0;
            } 
            if i.release_year!=lastyear {
                println!("");
                print!("{}-", i.release_year);
                print!("{}", i.name);

            }
            else {print!(", {}", i.name);}
            lastartist=i.artist;
            lastyear=i.release_year;
        }
        println!("");
    }
}

fn add_whitespace(last_string: &str, gap: usize) -> String {
    /*
    Description: 
    Creates a string of whitespaces for get_timeline()
    Parameters:
    last_string: &str-the previous item added in get_timeline, used to determine the length of the whitespace string
    gap: usize-The length of the gap between items excluding the length of last_string
    Returns:
    String: A string of whitespaces
    */
    //Defines the length of the gap by subtracting the previous string from the overall desired gap
    let remaining_gap_length=gap-last_string.len();
    let rgli: i32=remaining_gap_length as i32;
    let mut whitespace_str=String::from("");
    //Populates the gap string with whitespaces
    for i in 0..rgli {
        whitespace_str.push(' ');
    }
    return whitespace_str;
}

fn get_album_ranking(ranking: &(String, Vec<String>, Vec<String>)) {
    /*
    Description: 
    Prints an album ranking for a given artist
    Parameters:
    ranking: &(String, Vec<String>, Vec<String>)-A reference to a ranking entry, containing the data to be printed
    Returns:
    None
    */
    //The same routine seen in album_ranking() to print the album ranking for an artist
    //Without editing functionality
    let artist=ranking.0.clone();
    let ranks=ranking.1.clone();
    let remaining_albums=ranking.2.clone();
    println!("{} album ranking:", artist);
    println!("rank                        unranked");
    for i in 0..remaining_albums.len() as usize {
        let ranklen=ranks[i].len();
        let gap=29-ranklen;
        let mut rankstr=ranks[i].clone();
        for i in 0..gap {rankstr.push(' ');}
        rankstr.push('|');
        rankstr.push_str(&remaining_albums[i].clone());
        println!("{}", rankstr);
    }
    let ranksleft=ranks.len()-remaining_albums.len();
    let rankidx=ranks.len()-ranksleft;
    for i in rankidx..ranks.len() {
        
        let ranklen=ranks[i].len();
        let gap=29-ranklen;
        let mut rankstr=ranks[i].clone();
        for i in 0..gap {rankstr.push(' ');}
        rankstr.push('|');
        if i==0 {
            println!("{}None", rankstr);
            continue
        }
        println!("{}", rankstr);
    }
}

fn get_artist_list(artists: &Vec<Artist>) {
    /*
    Description: 
    Prints a list of every artist in the system
    Parameters:
    artists: &Vec<Artist>-The vector containing all artists in the system
    Returns:
    None
    */
    for i in artists {println!("{}", i.name);}
}

fn get_album_list(albums: &Vec<Album>) {
    /*
    Description: 
    Prints a list of every album in the system
    Parameters:
    albums: &Vec<Album>-The vector containing all albums in the system
    Returns:
    None
    */
    for i in albums {println!("{}", i.name);}
}

fn search_album_by(albums: &Vec<Album>) -> String {
    /*
    Description: 
    Allows the user to search for an album by field
    Parameters:
    albums: &Vec<Album>-The vector containing all albums in the system
    Returns:
    String-An 'exitline' string, which is used to handle the user exiting the program from inside this function
    */
    println!("Type 'return' to exit");
    //Loops so users can search multiple albums without having to re-enter the function 
    loop {
        //Processes field to be searched by
        let field=add_field("Search by release year, artist or genre?", "", "");
        if field=="NullA" {break}
        else if field=="TerminateProgram" {return String::from("TerminateProgram");}
        let mut hit=false;
        match field.to_lowercase().trim() {
            "release year" => {
                let entry=loop {
                    let entry=add_field("Enter year to search by", "i32", "search album").parse().unwrap();
                    if entry!=-3 {break entry}
                    else if entry<1960 || entry>1980 {println!("Please enter a year between 1960 and 1980");}
                    
                };
                //Returns exit command string in case user wants to exit the program while entering fields
                if entry==-2 {return String::from("TerminateProgram");}
                if entry==-1 {
                    break;
                }
                if entry==0 {continue;}
                //Prints every album in the system which released in the year specified, 
                //prints 'None' if there are noen
                for i in albums {
                    if i.release_year==entry {
                        println!("{}", i.name);
                        hit=true;
                    }
                }
                if hit==false {println!("None");}
            }
             "artist" => {
                let entry=add_field("Enter artist to search by", "", "search album");
                if entry=="TerminateProgram" {return String::from("TerminateProgram");}
                if entry=="NullA" {break;}
                if entry=="" {continue;}
                //Prints all of an artist's albums
                //Other sections in the search functions work much the same
                for i in albums {
                    if i.artist.to_lowercase()==entry.to_lowercase() {
                        println!("{}", i.name);
                        hit=true;
                    }
                }
                if hit==false {println!("None");}
            }
             "genre" => {
                let entry=add_field("Enter genre to search by", "", "search album");
                if entry=="TerminateProgram" {return String::from("TerminateProgram");}
                if entry=="NullA" {break;}
                if entry=="" {continue;}
                for i in albums {
                    if i.genre.to_lowercase()==entry.to_lowercase() {
                        println!("{}", i.name);
                        hit=true;
                    }
                }
                if hit==false {println!("None");}
            }
            _=> {
                println!("Search field not recognised, please type either 'artist', 'genre', or 'release year'");
                continue;
            }            
        }
    }
    return String::from("");
}

fn search_artist_by(artists: &Vec<Artist>) -> String {
    /*
    Description: 
    Allows the user to search for an artist by field
    Parameters:
    albums: &Vec<Artist>-The vector containing all artists in the system
    Returns:
    String-An 'exitline' string, which is used to handle the user exiting the program from inside this function
    */
    let current_date: DateTime<chrono::Utc>=chrono::Utc::now();
    let current_year: i32=current_date.year() as i32;
    println!("Type 'return' to exit");
    loop {
        let field=add_field("Search by year active, genre or member?", "", "");
        let mut hit=false;
        if field=="NullA" {break}
        else if field=="TerminateProgram" {return String::from("TerminateProgram");}
        match field.to_lowercase().trim() {
            "year active" => {
                let entry=loop {
                        let year: i32=add_field("Enter year", "i32", "").parse().unwrap();
                        if year!=0 && year!=-1 && year!=-2 {
                            if year==-3 {continue}
                            else if year<1960 || year>current_year {
                                println!("Please enter a year between 1960 and 2025");
                                continue
                            }
                        }
                        break year;
                    };
                if entry==-2 {return String::from("TerminateProgram");}
                if entry==-1 {break;}
                if entry==0 {continue;}
                //Prints every artist that was active in the years specified
                for i in artists {
                    if i.years_active.0<=entry && i.years_active.1>=entry {
                        println!("{}", i.name);
                        hit=true;
                    }
                }
                if hit==false {println!("None");}
            }
             "member" => {
                let entry=add_field("Enter member to search by", "", "search album");
                if entry=="TerminateProgram" {return String::from("TerminateProgram");}
                if entry=="NullA" {break;}
                if entry=="" {continue;}
                for i in artists {
                    let member_present=find(&i.members, &entry.to_lowercase()).0;
                    if member_present==true {
                        println!("{}", i.name);
                        hit=true;
                    }
                }
                if hit==false {println!("None");}
            }
             "genre" => {
                let entry=add_field("Enter genre to search by", "", "search album");
                if entry=="TerminateProgram" {return String::from("TerminateProgram");}
                if entry=="NullA" {break;}
                if entry=="" {continue;}
                for i in artists {
                    if i.genre.to_lowercase().trim()==entry.to_lowercase().trim() {
                        println!("{}", i.name);
                        hit=true;
                    }
                }
                if hit==false {println!("None");}
            }
            _=> {
                println!("Search field not recognised, please type either 'artist', 'genre', or 'release year'");
                continue;
            }     
        }
    }
    //Returns dud string if user does not exit the program here
    return String::from("");
}

fn load_data() -> (Vec<Artist>, Vec<Album>, Vec<(String, Vec<String>, Vec<String>)>) {
    /*
    Description: 
    Main loading function which calls and loops the individual loading functions for artists, albums, and rankings;
    creates txt files if they don't exist and reads them to strings if they do
    Parameters:
    None
    Returns:
    (Vec<Artist>, Vec<Album>, Vec<(String, Vec<String>, Vec<String>)>): 
    A tuple containing all the data for the artists, albums, and rankings functions
    */
    let mut rankings_is_present: bool=false;
    let mut albums_is_present:bool=false;
    let mut artists_is_present:bool=false;
    //Loads the txt files for each of the primary data structures into strings
    rankings_is_present = Path::new("rankings.txt").exists();
    artists_is_present = Path::new("artists.txt").exists();
    albums_is_present = Path::new("albums.txt").exists();
    if rankings_is_present==false {
        let mut new_file=File::create("rankings.txt").expect("Creation failed");
    }
    if artists_is_present==false {
        let mut new_file=File::create("artists.txt").expect("Creation failed");
    }
    if albums_is_present==false {
        let mut new_file=File::create("albums.txt").expect("Creation failed");
    }
    let mut rankings_result = File::open("rankings.txt");
    let mut artists_result = File::open("artists.txt");
    let mut albums_result = File::open("albums.txt");
    let mut rankings_content = String::new();
    let mut artists_content = String::new();
    let mut albums_content = String::new();
    let mut rankings_file = match rankings_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };
     let mut artists_file = match artists_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };
     let mut albums_file = match albums_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };
    rankings_file.read_to_string(&mut rankings_content);
    artists_file.read_to_string(&mut artists_content);
    albums_file.read_to_string(&mut albums_content);
    let mut char_no=1;
    let mut rankings: Vec<(String, Vec<String>, Vec<String>)>=Vec::new();
    let mut artists=Vec::new();
    let mut albums=Vec::new();
    if rankings_content.is_empty()==false {
        //Calls a child function to load an individual ranking then appends that to the vector and moves onto the next
        while char_no!=0 {
            let ranking_data: (String, Vec<String>, Vec<String>, usize)=load_ranking(rankings_content.clone(), char_no);
            if ranking_data.0!="" {
                rankings.push((ranking_data.0.clone(), ranking_data.1.clone(),  ranking_data.2.clone()));
                
            }
           char_no=ranking_data.3;
        }
    }
    char_no=1;
    if artists_content.is_empty()==false {
        //Same process as fr rankings
        loop {
            let artist_data=load_artist(artists_content.clone(), albums_content.clone(), char_no);
            char_no=artist_data.1;
            if char_no==0 {break}
            if artist_data.0.name!="" {artists.push(artist_data.0);}
        }

        for i in artists.clone() {
            let mut artists_albums=i.albums.clone();
            albums.append(&mut artists_albums);
        }
    }
    return (artists, albums, rankings)
    
}

fn load_artist(artists_content: String, albums_content: String, mut char_no: usize) -> (Artist, usize) {
    /*
    Description: 
    Creates an individual artist entry from the artists and albums strings loaded from load_data()
    Parameters:
    artists_content: String-A string containing fields for all artists in the system loaded from load_data()
    albums_content: String-A string containing fields for all albums in the system loaded from load_data()
    char_no: usize-The current character number in artists_content; being the starting position for the current artists data in artists_content
    Returns:
    (Artist, usize)-A tuple containing the new artist entry and the character index in artists_content where the data for the next entry begins
    */
    let mut artist_entry=String::new();
    let mut artists: Vec<Artist>=Vec::new();
    let mut section: usize=0;
    let mut name=String::new();
    let mut genre=String::new();
    let mut description=String::new();
    let mut years_active=(0, 0);
    let mut members:Vec<String>=Vec::new();
    let mut albums=Vec::new();
    if char_no!=1 {
        char_no+=2;
    };
    loop {
        //Gets character for current artist
        let char=artists_content.chars().nth(char_no).unwrap();
        //adds members to vector, ignoring line break characters
        if section==4 && char=='\n' {
            if artist_entry!="" {members.push(artist_entry.clone())};
            artist_entry=String::from("");
        }
        //loads albums to both artist vector and albums, ignoring line break characters
        else if section==5 && char=='\n' {
            if artist_entry.trim()!="" {
                let newalbum=load_album(albums_content.clone(), name.clone(), artist_entry.clone());
                albums.push(newalbum);
            }
            
            artist_entry=String::from("");
        }
        //Checks to see if the next two characters are '|'s, and if so returns a default artist
        //and a char of 0, causing the artist loading subroutine to finish
        //Otherwise appends the current character if it is not a '|'
        else if char!='|' {
            let nextchar=artists_content.chars().nth(char_no+1).unwrap();
            let nextnextchar=artists_content.chars().nth(char_no+2).unwrap();
            if nextchar=='|' && nextnextchar=='|' {
                return (Artist::default(), 0);
            }
            artist_entry.push(char);
        }
        //If the current character is a '|', saves the current field entry to its appropriate variable
        //based on the current section
        //Or if the section is 5, returns an artist structure
        else {
            let new_entry=artist_entry.clone().trim().to_string();
                match section {
                    0=> {name=new_entry;},
                    1=> {genre=new_entry;},
                    2=> {
                        let start_year=&artist_entry[0..4];
                        let end_year=&artist_entry[5..9];
                        let start_year_int: i32=start_year.parse().unwrap();
                        let end_year_int: i32=end_year.parse().unwrap();
                        years_active=(start_year_int, end_year_int);
                    },
                    3=> {description=new_entry},
                    4=> {},
                    _=> {return (Artist{name: name, genre: genre, years_active: years_active, description: description, members: members, albums: albums}, char_no)}
                }
            //As the current section is finished, moves onto the next section and resets the field entry string
            section+=1;
            artist_entry=String::from("");
            
        }
        char_no+=1;          
    }
}
    

fn load_album(albums_content: String, artistname: String, albumname: String) -> Album {
    /*
    Description: 
    Creates an individual album entry from the albums string loaded from load_data()
    Parameters:
    albums_content: String-A string containing fields for all albums in the system loaded from load_data()
    artistname: String-The name of the artist the current album belongs to
    albumname: String-The name of the album currently being loaded
    Returns:
    Album-The new album entry
    */
    let mut album_entry=String::new();
    let mut albums: Vec<Album>=Vec::new();
    let mut char_no: usize;
    let mut section: usize=0;
    let mut name=String::new();
    let mut genre=String::new();
    let mut description=String::new();
    let mut releaseyear=0;
    let mut format=String::new();
    let mut record_dates=String::new();
    let mut current_album=String::new();
    let mut current_char: char;
    let mut current_charno: usize=1;
    let mut next_current_char: char='k';
    current_char=albums_content.chars().nth(current_charno).unwrap();
    loop {
        //Locatees position of the album to be searched in the albums string and 'returns' the starting index
        char_no=current_charno;
        while current_char!='|' {
            current_album.push(current_char);
            current_charno+=1;
            current_char=albums_content.chars().nth(current_charno).unwrap();
        }
        if current_album.trim()==albumname.trim() {
            break
        }
        current_album=String::from("");
        //Skips over searching parts of the string which are for fields other than album name
        //For example, Ornette Coleman released an album called 'Free Jazz' which is also the name of a subgenre
        //So if all fields were searched this would cause the album to begin loading from an incorrect location
        //and crash the program
        loop {
            if current_char=='|' && next_current_char =='|' {break}
            current_charno+=1;
            current_char=albums_content.chars().nth(current_charno).unwrap();
            next_current_char=albums_content.chars().nth(current_charno+1).unwrap();
        }
        //Skips over the '|'s so the charno corresponds to the first character of the album
        current_charno+=2;
        current_char=albums_content.chars().nth(current_charno).unwrap();
        next_current_char=' ';
    }
    //The remainder of the function follows the same template as load_artist(), albeit simpler
    loop {
        let char=albums_content.chars().nth(char_no).unwrap();
        if char!='|' {
            album_entry.push(char);
            char_no+=1
        }
        else {
            let new_entry=album_entry.clone().trim().to_string();
            match section {
                0=> {name=new_entry;},
                1=> {genre=new_entry;},
                2=> {format=new_entry;},
                3=> {
                    releaseyear=new_entry.parse().unwrap();
                },
                4=> {record_dates=new_entry;},
                _=> {description=new_entry;}
            }
            if section!=5 {
                char_no+=1;
                section+=1;
                album_entry=String::from("");
            } 
            else {return Album{name: name, genre: genre, artist: artistname, format: format, 
                release_year: releaseyear, record_dates: record_dates, description: description}}
        }
        
    }
}

fn load_ranking(rankings_content: String, mut char_no: usize) -> (String, Vec<String>, Vec<String>, usize) {
    /*
    Description: 
    Creates an individual ranking entry from the rankings string loaded from load_data()
    Parameters:
    rankings_content: String-A string containing the data for all rankings in the system loaded from load_data()
    char_no: usize-The current character number in artists_content; being the starting position for the current artists data in artists_content
    Returns:
    (String, Vec<String>, Vec<String>, usize)-A tuple containing the new ranking and 
    the character index in rankings_content where the data for the next entry begins
    */
    let mut artist=String::new();
    let mut ranking_entry=String::new();
    let mut remaining_albums=Vec::new();
    let mut rankings=Vec::new();
    let mut section=0;
    //Adjusts position for '|' gaps, I think
    if char_no!=1 {char_no+=2};
    loop {
        let char=rankings_content.chars().nth(char_no).unwrap();
        //Every line break, adds the current ranking entry to the ranking data structures based on section
        //Then resets the entry string for the next line
        if char=='\n' {
            if section==0 && ranking_entry.trim()!="" {artist=ranking_entry.clone();}
            else if section==1 && ranking_entry.trim()!="" {rankings.push(ranking_entry.clone());}
            else if section==2 && ranking_entry.trim()!="" {remaining_albums.push(ranking_entry.clone());}
            ranking_entry=String::from("");
        }
        //If the current character is '|', checks for the next two being this as well and if so,
        //returns a termination command
        else if char=='|' {
            let nextchar=rankings_content.chars().nth(char_no+1).unwrap();
            let nextnextchar=rankings_content.chars().nth(char_no+2).unwrap();
            if nextchar=='|' && nextnextchar=='|' {
                return (String::from(""), Vec::new(), Vec::new(), 0)
            }
            //If the current section is 0 or 1, moves onto the next section and resets the entry string
            if section<2 {
                section+=1;
                ranking_entry=String::from("");
            }
            //If this is the last section, returns the ranking variables to be combined into the next rankings entry
            else {
                section=0;
                return (artist, rankings, remaining_albums, char_no)
            }
            
        }
        //Adds the current character to the entry string if it isn't '/n' or '|'
        else {
            ranking_entry.push(char);
        }
        char_no+=1;
    }
    
}

fn save_data(albums: Vec<Album>, artists: Vec<Artist>, rankings: Vec<(String, Vec<String>, Vec<String>)>) {
    /*
    Description: 
    Serialises the three primary data structures to txt files
    Parameters:
    artists: Vec<Artist>-One of the primary data structures, a vector of every artist in the system
    albums: Vec<Artist>-One of the primary data structures, a vector of every album in the system
    rankings: Vec<(String, Vec<String>, Vec<String>)>-One of the primary data structures, 
    a vector storing the rankings and unranked albums for every artist, keyed by artist name. 
    Returns:
    None
    */
    let mut rankings_is_present: bool=false;
    let mut albums_is_present:bool=false;
    let mut artists_is_present:bool=false;
    //Creates files for the primary data structures if they don't exist
    rankings_is_present = Path::new("rankings.txt").exists();
    artists_is_present = Path::new("artists.txt").exists();
    albums_is_present = Path::new("albums.txt").exists();
    if rankings_is_present==false {
        println!("Rankings file does not exist, creating file");
        let mut new_file=File::create("rankings.txt").expect("Creation failed");
    }
     if artists_is_present==false {
        println!("Artists file does not exist, creating file");
        let mut new_file=File::create("artists.txt").expect("Creation failed");
    }
    if albums_is_present==false {
        println!("Albums file does not exist, creating file");
        let mut new_file=File::create("albums.txt").expect("Creation failed");
    }
    //Creates save strings to be populated
    let mut rankings_content = String::new();
    let mut artists_content = String::new();
    let mut albums_content = String::new();
    let artists_length=artists.len();
    let albums_length=albums.len();
    //The first line is always empty. This solved an issue 
    //I don't remember what that was but just trust me okay it makes it work
    albums_content.push_str("\n");
    for i in 0..albums_length as usize {
        //Gets the current album and converts the release year into a string so it can be pushed
        let album=albums[i].clone();
        let release_year=album.release_year.clone();
        let release_year_asstr=release_year.to_string();
        //Pushes each of the album's fields onto the albums string, seperated by a '|'
        //then finishes by adding a new line with '||' to indicate the end of an album's fields
        albums_content.push_str(&album.name);
        albums_content.push('|');
        albums_content.push_str(&album.genre);
        albums_content.push('|');
        albums_content.push_str(&album.format);
        albums_content.push('|');
        albums_content.push_str(&release_year_asstr);
        albums_content.push('|');
        albums_content.push_str(&album.record_dates);
        albums_content.push('|');
        albums_content.push_str(&album.description);
        albums_content.push_str("\n||\n");
    }
    //Once all albums have been added, adds '|||' to signpost the end of the file
    albums_content.push_str("|||");

    rankings_content.push_str("\n");
    for i in 0..artists_length as usize {
        let artistname=rankings[i].0.clone();
        let ranking=rankings[i].1.clone();
        let remaining_albums=rankings[i].2.clone();
        let rankings_length=ranking.len();
        let remaining_albums_length=remaining_albums.len();
        rankings_content.push_str(&artistname.clone());
        rankings_content.push_str("\n|\n");
        for n in 0..rankings_length as usize {
            let line=ranking[n].clone();
            rankings_content.push_str(&line);
            rankings_content.push_str("\n");
        }
        rankings_content.push_str("|\n");
        for n in 0..remaining_albums_length as usize {
            let line=remaining_albums[n].clone();
            rankings_content.push_str(&line);
            rankings_content.push_str("\n");

        }
        rankings_content.push_str("||\n");
    }
    rankings_content.push_str("|||");

    artists_content.push('\n');
    for i in 0..artists_length as usize {
        let artist=artists[i].clone();
        let years_active=artist.years_active.clone();
        let albums_length=artist.albums.len();
        let members_length=artist.members.len();
        //Converts years active field to a string
        let mut years_active_asstr=years_active.0.to_string();
        years_active_asstr.push_str("-");
        years_active_asstr.push_str(years_active.1.to_string().as_str());
        //Similar formatting to albums content
        artists_content.push_str(&artist.name);
        artists_content.push('|');
        artists_content.push_str(&artist.genre);
        artists_content.push('|');
        artists_content.push_str(&years_active_asstr);
        artists_content.push('|');
        artists_content.push_str(&artist.description);
        artists_content.push_str("|\n");
        //Loops through members and albums vectors and adds each item to a new line
        for n in 0..members_length {
            artists_content.push_str(&artist.members[n]);
            artists_content.push_str("\n")
        }
        if members_length==0 {artists_content.push_str("\n");}
        artists_content.push_str("|\n");
        for n in 0..albums_length {
            artists_content.push_str(&artist.albums[n].name);
            artists_content.push_str("\n")
        }
        if albums_length==0 {artists_content.push_str("\n");}
        artists_content.push_str("||\n");
    }
    artists_content.push_str("|||");

    //Saves each contents string to a file, overwriting the previous contents
    let mut rankings_file = File::create("rankings.txt").expect("Creation failed");
    let mut artists_file = File::create("artists.txt").expect("Creation failed");
    let mut albums_file = File::create("albums.txt").expect("Creation failed");
    rankings_file.write(rankings_content.as_bytes()).expect("Rankings save failed");
    artists_file.write(artists_content.as_bytes()).expect("Artists save failed");
    albums_file.write(albums_content.as_bytes()).expect("Albums save failed");
    println!("Data saved");
}

fn add_field(msg: &str, vartype: &str, parent: &str) -> String {
    /*
    Description: 
    Validates and handles user command input 
    Parameters:
    msg: &str-The prompt for the user input to print
    vartype: &str-Used to specify if the variable return type is i32 or not
    parent: &str-Used to specify the name of the parent function to pass onto the help function
    Returns:
    String-Either the user inputted string or a special command message specifying a control course for the parent function
    */
    let mut cmd: String=String::from("");
    while cmd.trim()==String::from("") || cmd.trim()==String::from("TerminateProgram") || cmd.trim()==String::from("NullA") {
        cmd=String::new();
        //Prints the prompt, repeats upon invalid entry
        println!("{}", msg);
        io::stdin().read_line(&mut cmd).expect("Command not recognised");
        let checkstr=cmd.trim();
        let isnumeric=is_numeric(checkstr);
        //Rejects entry if the field is empty
        if checkstr=="" {
            println!("Please ensure field isn't empty");
        }
        //If entry is 'ex', returns program termination command
        else if checkstr.to_lowercase()=="ex" {
            if vartype=="i32" {return String::from("-2")}
            else {return String::from("TerminateProgram")}
        }
        //If entry is 'redo', returns section restart command
        else if checkstr.to_lowercase()=="redo" {
            if vartype=="i32" {return String::from("0")}
            else {return String::from("")}
        }
        //If entry is 'return', returns travel to main section command
        else if checkstr.to_lowercase()=="return" {
            if vartype=="i32" {return String::from("-1")}
            else {return String::from("NullA")}
        }
        //If entry is 'help', prints the available commands for the parent function
        else if checkstr=="help" {
            help(parent);
            cmd=String::from("");
        }
        //If the variable being returned to is an i32, refuses if the string cannot be parsed as an integer
        else if vartype=="i32" && isnumeric==false {
            println!("Please enter a positive integer value");
            return String::from("-3");
        }
        //If the variable being returned to is an i32, 
        //refuses if the string would be parsed as a non-positive value
        else if vartype=="i32" && isnumeric==true {
            let checki32: i32=checkstr.parse().unwrap();
            if checki32<1 {
                println!("Please enter a positive integer value");
                return String::from("-3");
            }
        }
        //User can't use '|' as this would interfere with saving and loading functions
        else if checkstr.contains("|") {println!("Please do not use the character '|' as this is used to define field seperation on serialisation")}
        //yeah
        else if checkstr=="TerminateProgram" || checkstr=="NullA" {
            println!("I don't think there's a band, person or album called that. Stop trying to cause an error")
        }
    }
    return String::from(cmd.trim());
}

fn find_artist(artists: &Vec<Artist>, artist: &str) ->(bool, usize) {
    /*
    Description: 
    Searches through all artists to see if the entered artist exists in the system
    Parameters:
    artists: &Vec<Artist>-A reference to the vector containing all artists in the system
    artist: &str-the name of the artist to be searched
    Returns:
    (bool, usize): A tuple containing a boolean referencing whether the artist is in the system or not,
    and a usize, the position of the artist in the system, or 10000 if it doesn't exist
    */
    let mut idx=0;
    for i in artists {
        if i.name.to_lowercase().trim()==artist.trim() {return (true, idx);}
        idx+=1;
    } 
    return (false, 10000);
}

fn find_album(albums: &Vec<Album>, album: &str) ->(bool, usize) {
    /*
    Description: 
    Searches through all albums to see if the entered album exists in the system
    Parameters:
    albums: &Vec<Album>-A reference to the vector containing all albums in the system
    album: &str-the name of the album to be searched
    Returns:
    (bool, usize): A tuple containing a boolean referencing whether the album is in the system or not,
    and a usize, the position of the album in the system, or 10000 if it doesn't exist
    */
    let mut idx: usize=0;
    for i in albums {
        if i.name.to_lowercase().trim()==album.trim() {return (true, idx);}
        idx+=1;
    } 
    return (false, 10000);
}

fn find_artist_nlc(artists: &Vec<Artist>, artist: &str) ->(bool, usize) {
    /*
    Description: 
    Searches through all artists to see if the entered artist exists in the system (Case specific)
    Parameters:
    artists: &Vec<Artist>-A reference to the vector containing all artists in the system
    artist: &str-the name of the artist to be searched
    Returns:
    (bool, usize): A tuple containing a boolean referencing whether the artist is in the system or not,
    and a usize, the position of the artist in the system, or 10000 if it doesn't exist
    */
    let mut idx=0;
    for i in artists {
        if i.name.trim()==artist.trim() {return (true, idx);}
        idx+=1;
    } 
    return (false, 10000);
}

fn find_album_nlc(albums: &Vec<Album>, album: &str) ->(bool, usize) {
    /*
    Description: 
    Searches through all albums to see if the entered album exists in the system (Case specific)
    Parameters:
    albums: &Vec<Album>-A reference to the vector containing all albums in the system
    album: &str-the name of the album to be searched
    Returns:
    (bool, usize): A tuple containing a boolean referencing whether the album is in the system or not,
    and a usize, the position of the album in the system, or 10000 if it doesn't exist
    */
    let mut idx: usize=0;
    for i in albums {
        if i.name.trim()==album.trim() {return (true, idx);}
        idx+=1;
    } 
    return (false, 10000);
}

fn find_ranking(rankings: &Vec<(String, Vec<String>, Vec<String>)>, artist: &str) ->(bool, usize) {
    /*
    Description: 
    Searches through all rankings to see if a ranking for the artist exists in the system
    Parameters:
    rankings: &Vec<(String, Vec<String>, Vec<String>)>-A reference to the vector containing all rankings in the system
    artist: &str-the name of the artist to be searched
    Returns:
    (bool, usize): A tuple containing a boolean referencing whether the artist is in the system or not,
    and a usize, the position of the ranking in the system, or 10000 if it doesn't exist
    */
    let mut idx: usize=0;
    for i in rankings {
        if i.0.to_lowercase().trim()==artist.trim() {return (true, idx);}
        idx+=1;
    } 
    return (false, 10000);
}

fn find(arr: &Vec<String>, thing: &str) ->(bool, usize) {
    /*
    Description: 
    Searches through an array to see if a string exists in it
    Parameters:
    arr: &Vec<String>-A vector of strings
    thing: &str-the string to be searched for
    Returns:
    (bool, usize): A tuple containing a boolean referencing whether the string is in the array or not,
    and a usize, the position of the string in the array, or 10000 if it doesn't exist
    */
    let mut idx: usize=0;
    for i in arr {
        if i.to_lowercase().trim()==thing.trim() {return (true, idx);}
        idx+=1;
    } 
    return (false, 10000);
}

fn find_ranking_album(arr: &Vec<String>, thing: &str) ->(bool, usize, String) {
    /*
    Description: 
    Searches through an array of rankings to see if an album exists in it
    Parameters:
    arr: &Vec<String>-A vector of album names
    thing: &str-the album to be searched for
    Returns:
    (bool, usize): A tuple containing a boolean referencing whether the album is in the vector or not,
    and a usize, the position of the album in the vector, or 10000 if it doesn't exist
    */
    let mut rank_idx: usize=0;
    for i in arr {
        let mut albumname=String::new();
        let mut rank=String::new();
        let mut idx=0;
        let mut name_start=false;
        //Adds characters to the albumname string once a whitespace is found in the string
        //this being the final character in the numbered section of a rank string
        for (n, &item) in i.as_bytes().iter().enumerate() {
            let letter=item as char;
            if name_start==true {albumname.push(letter);} 
            else {rank.push(letter);}
            idx+=1;
            if letter==' ' {name_start=true;}
        }
        if albumname.to_lowercase()==thing {return (true, rank_idx, rank);}
        rank_idx+=1;
    } 
    return (false, 10000, String::from(""));
}

fn adjust_ranks(rank_idx: usize, mut ranking: Vec<String>) 
-> Vec<String> {
    /*
    Description:
    Alters the numbers of an artist ranking such that there is never a gap greater than one between ranks
    Parameters:
    rank_idx: usize-The position deleted at, the point in the vector at which the loop starts operating
    ranking: Vec<String>-The vector to be altered
    Returns:
    Vec<String>-The altered ranking vector
    */
    //Loops through the point deleted at to the end of the vector
    for i in rank_idx..ranking.len() {
        //Gets the current rank number, decrements it and converts it to a string
        let mut rank=ranking[i].clone();
        let mut rank_no=get_rankno(&rank) as u8;
        rank_no-=1;
        let mut rank_no_str=rank_no.to_string();
        //Removes the number in the rank entry
        let double_digit_true=is_two_digit(&rank);
        rank.remove(0);
        if double_digit_true==true {rank.remove(0);}
        //Pushes the remaining rank name onto the new rank number string
        rank_no_str.push_str(&rank.clone());
        //Replaces the old string at the current position with the adjusted string
        ranking.remove(i);
        ranking.insert(i, rank_no_str);
    }
    return ranking
}

fn insert_artist_album(mut artist_albums: Vec<Album>, album: Album) -> Vec<Album> {
    /*
    Description:
    Inserts an an album into their artist's vector of albums such that they are ordered by release year
    Parameters:
    artist_albums: Vec<Album>-The vector of albums to be altered
    album: Album-The album to be inserted
    Returns:
    Vec<Album>-The altered vector of albums
    */
    let mut finalpos=0;
    //Checks to see if the artist has any albums before finding the index of the final album
    if artist_albums.is_empty()==false {finalpos=artist_albums.len()-1;}
    //Adds the album to the start/end of the array if it is the first to be added
    if artist_albums.is_empty() {artist_albums.push(album.clone());}
    //If the albums release year is before the first album in the vector or after the last album, 
    //inserts it at the beginning or end respectively
    else if album.clone().release_year<artist_albums[0].release_year {artist_albums.insert(0, album.clone());}
    else if album.clone().release_year>artist_albums[finalpos].release_year {artist_albums.push(album.clone());}
    //If the album is to be positioned before the end and after the beginning
    else {
        //Loops until a position is found where the previous album was released before or on the same year 
        //as the album to be inserted, and the ending year is after or the same
        //Then inserts the album at this position
        for i in 0..artist_albums.len()-1 {
            if album.clone().release_year>=artist_albums[i].release_year && album.release_year<=artist_albums[i+1].release_year {
                artist_albums.insert(i+1, album.clone());
                break
            }
        }
    }
    return artist_albums;
}

fn ranking_whitespace_plaster(mut ranking: Vec<String>) -> Vec<String> {
    /*
    Description: 
    Ensures every rank has a whitespace following the '.'
    Parameters:
    ranking: Vec<String>-The ranking vector to be altered
    Vec<String>-The altered ranking vector
    */
    //If any ranking has a length of 2 (i.e being '1.' without a whitespace), 
    //adds a whitespace to the end of the string
    for i in 0..ranking.len() {
        let mut rankstr=ranking[i].clone();
        if rankstr.len()==2 {
            rankstr.push(' ');
            ranking.remove(i);
            ranking.insert(i, rankstr);
        }
        else {
            //If the ranking has an album attached and there is no gap between the first character
            //of the album and the dot, inserts a whitespace between them
            let double_digit_true=is_two_digit(&rankstr);
            if double_digit_true==true {
                if rankstr.chars().nth(3).unwrap()!=' ' {
                    rankstr.insert(3, ' ');
                    ranking.remove(i);
                    ranking.insert(i, rankstr);
                }
            }
            else {
                if rankstr.chars().nth(2).unwrap()!=' ' {
                    rankstr.insert(2, ' ');
                    ranking.remove(i);
                    ranking.insert(i, rankstr);
                }
            }
            
        }
    }
    return ranking;
}

fn artists_init() -> Vec<Artist> {
    /*
    Description: 
    Initialises artists entries if none are present; e.g. the system has been started for the first time
    Parameters:
    None
    Returns:
    Vec<Artist>-A basic form of the artists vector
    */
    let mut artists: Vec<Artist>=Vec::new();
    artists.push(Artist {name: String::from("Miles Davis"), genre: String::from("Jazz"), members: Vec::new()
    , years_active: (1941, 1991) , description: String::from("Getting his start playing alongside Charlie Parker in the mid-forties, Miles Davis is unparalleled in the field of jazz, having originated a number of its subgenres; modal jazz, cool jazz, and jazz fusion, he was a legendary innovator and has produced many classic albums."), 
    albums: vec![Album { name: String::from("Sketches Of Spain"), genre:  String::from("Latin Jazz"), 
    artist: String::from("Miles Davis"), format: String::from("LP"), release_year: 1960, 
    record_dates: String::from("1959"), description: String::from("Overshadowed by Kind Of Blue, This Evans-Davis collaboration proves to be an excellent fusion of latin orchestration and the laid-back melodic improvisation which categorised Davis' playing during this era")}]});

    artists.push(Artist {name: String::from("King Crimson"), genre: String::from("Prog Rock"), 
    members: vec![String::from("Robert Fripp"), String::from("Others")]
    , years_active: (1968, 2021) , description: String::from("The true beginning of progressive rock as a movement is marked by the release of King Crimson's debut album in 1969. They went on to iterate on its groundbreaking ideas in 70s to mixed results, but nonetheless producing a number of classic records during this period.")
    , albums:vec![Album { name: String::from("Red"), genre:  String::from("Progressive Metal"), 
    artist: String::from("King Crimson"), format: String::from("LP"), 
    release_year: 1974, record_dates: String::from("1974"), description: String::from("The last KC record before a 7 year hiatus, the entire Red album can be seen as a successor to 21st Century Schizoid Man, the opener from their debut. It is one of the most well-regarded albums of their original run and serves as one of the earliest examples of progressive metal.")}]});

    artists.push(Artist {name: String::from("Joy Division"), genre: String::from("New Wave"), 
    members: vec![String::from("Ian Curtis"), String::from("Bernard Sumner"), String::from("Peter Hook"), 
    String::from("Stephen Morris")], years_active: (1976, 1980) , description: String::from("Initially a Sex-Pistols-inspired punk-rock outfit, Joy Division developed their sound to become one of the pioneering and most well-regarded post-punk bands by the late 70s. The band's momentum was cut short in 1980 following the tragic death of lead singer Ian Curtis, and the remaining members reformed a year later as New Order."), 
    albums: vec![Album { name: String::from("Unknown Pleasures"), genre: String::from("Post-Punk"), 
    artist: String::from("Joy Division"), format: String::from("LP"), release_year: 1979, 
    record_dates: String::from("1979"), description: String::from("Initially commercially unsuccessful, Unknown Pleasures went on to become a defining and influential post-punk album, characterised by its atmospheric production and dry vocals. The cover has since become an inconic t-shirt staple, even to those who haven't heard the album itself.")}]});

    artists.push(Artist {name: String::from("Isaac Hayes"), genre: String::from("Soul"), members: Vec::new()
    , years_active: (1963, 2008) , description: String::from("Hayes started his career as a session musician and songwriter for Stax Records in the early sixties, he released his debut album in 1968 which did poorly commercially. As a result of a label split, Stax altered its business plans, encouraging artists to record new material and giving Hayes complete creative control over his second album. The record that followed, Hot Buttered Soul, was far more successful, pioneering a more progressive sound, and this would be followed by a number of other critically acclaimed albums in the early 70s."), 
    albums: vec![Album { name: String::from("Hot Buttered Soul"), genre:  String::from("Progressive Soul"), 
    artist: String::from("Isaac Hayes"), format: String::from("LP"), release_year: 1969, record_dates: String::from("1969"), 
    description: String::from("Consisting of only four songs across forty-five minutes, Hot Buttered Soul was a landmark album in the soul genre, blending lush production with complex song structures, it is widely considered one of Hayes' greatest albums.")}]});
    return artists;
}

fn albums_init() -> Vec<Album> {
    /*
    Description: 
    Initialises album entries if none are present; e.g. the system has been started for the first time
    Parameters:
    None
    Returns:
    Vec<Album>-A basic form of the albums vector
    */
    let mut albums: Vec<Album>=Vec::new();
    albums.push(Album { name: String::from("Sketches Of Spain"), genre:  String::from("Latin Jazz"), 
    artist: String::from("Miles Davis"), format: String::from("LP"), release_year: 1960, 
    record_dates: String::from("1959"), description: String::from("Overshadowed by Kind Of Blue, This Evans-Davis collaboration proves to be an excellent fusion of latin orchestration and the laid-back melodic improvisation which categorised Davis' playing during this era")});

    albums.push(Album { name: String::from("Red"), genre:  String::from("Progressive Metal"), 
    artist: String::from("King Crimson"), format: String::from("LP"), release_year: 1974, record_dates: String::from("1974"), 
    description: String::from("The last KC record before a 7 year hiatus, the entire Red album can be seen as a successor to 21st Century Schizoid Man, the opener from their debut. It is one of the most well-regarded albums of their original run and serves as one of the earliest examples of progressive metal.")});

    albums.push(Album { name: String::from("Hot Buttered Soul"), genre:  String::from("Progressive Soul"), 
    artist: String::from("Isaac Hayes"), format: String::from("LP"), release_year: 1969, record_dates: String::from("1969"), 
    description: String::from("Consisting of only four songs across forty-five minutes, Hot Buttered Soul was a landmark album in the soul genre, blending lush production with complex song structures, it is widely considered one of Hayes' greatest albums.")});

    albums.push(Album { name: String::from("Unknown Pleasures"), genre:  String::from("Post-Punk"), artist: 
    String::from("Joy Division"), format: String::from("LP"), release_year: 1979, record_dates: String::from("1979"), 
    description: String::from("Initially commercially unsuccessful, Unknown Pleasures went on to become a defining and influential post-punk album, characterised by its atmospheric production and dry vocals. The cover has since become an inconic t-shirt staple, even to those who haven't heard the album itself.")});

    return albums;
}

fn rankings_init() ->  Vec<(String, Vec<String>, Vec<String>)> {
    /*
    Description: 
    Initialises ranking entries if none are present; e.g. the system has been started for the first time
    Parameters:
    None
    Returns:
    Vec<(String, Vec<String>, Vec<String>)>-A basic form of the ranking vector
    */
    let rankings: Vec<(String, Vec<String>, Vec<String>)>=vec![(String::from("Miles Davis"), vec![String::from("1. ")], vec![String::from("Sketches of Spain")]), 
        (String::from("Isaac Hayes"), vec![String::from("1. ")], vec![String::from("Hot Buttered Soul")]), 
        (String::from("Joy Division"), vec![String::from("1. ")], vec![String::from("Unknown Pleasures")]), 
        (String::from("King Crimson"), vec![String::from("1. ")], vec![String::from("Red")])];
    return rankings;
}

fn create_ex_artist() -> Artist {
    /*
    Description: 
    Returns a special instance of the Artist data structure which the program recognises as a command to save and close
    Parameters:
    None
    Returns:
    Artist-The 'ex-artist' instance
    */
    return Artist { name: String::from("TerminateProgram"), genre: String::from(""), members: Vec::new(), years_active: (0, 0), description: String::from(""), albums: Vec::new()}
}

fn create_ex_album() -> Album {
    /*
    Description: 
    Returns a special instance of the album data structure which the program recognises as a command to save and close
    Parameters:
    None
    Returns:
    Artist-The 'ex-album' instance
    */
    return Album { name: String::from("TerminateProgram"), genre: String::from(""), artist: String::from(""), format: String::from(""), record_dates: String::from(""),  release_year: 0, description: String::from("")};
}





fn help(parent: &str) {
    /*
    Description: 
    Prints a description of the commands currently available based on the ancestor function
    Parameters:
    parent: &str-The name of the ancestor function to be matched
    Returns:
    None
    */
    match parent {
        "editalbum" => {
            println!("name          genre         format");
            println!("release year  record dates  description");
        }
        "editartist" => {
            println!("name        genre         years active");
            println!("members     description");
        }
        "editmembers" => {
            println!("append      clear")
        }
        "search album" => {
            println!("artist      release year      genre")
        }
         "search artist" => {
            println!("member      release year      genre")
        }
        "members" => {
            println!("end: continue to next section");
            println!("redo: Erase pending entries, start section from scratch. Not always usable");
            println!("return: Goes back to section entry, doesn't save pending entry unless it is a ranking");
            println!("ex: Saves entered data and exits program. Data will not be automatically saved if program is closed via other means");
        }
        _=> {
            println!("redo: Erase pending entries, start section from scratch. Not always usable");
            println!("return: Goes back to section entry, doesn't save pending entry unless it is a ranking");
            println!("ex: Saves entered data and exits program. Data will not be automatically saved if program is closed via other means");
        }
    }
}

fn function_descs(query: &str) {
    /*
    Description: 
    Prints a description of one of the primary functions
    Parameters:
    query: &str-the name of the function to be checked
    Returns:
    None
    */
    match query.trim() {
        "addartist?" => {println!("Adds a new artist to the database")}
        "editartist?" => {println!("Edits one of the fields for a pre-existing artist")}
        "delartist?" => {println!("Deletes an artist from the database. The artists albums and rankings are also deleted")}
        "addalbum?" => {println!("Adds a new album to the database")}
        "editalbum?" => {println!("Edits one of the fields for a pre-existing album")}
        "delalbum?" => {println!("Deletes an album from the database.")}
        "ranking?" => {println!("Allows for creating, editing, and clearing a ranking for a given artist")}
        "getranking?" => {println!("Prints a ranking")}
        "timeline?" => {println!("Prints a timeline for every album released by every artist over the 20 year time period")}
        "artistlist?" => {println!("Prints all artists")}
        "albumlist?" => {println!("Prints all albums")}
        "searchartist?" => {println!("Searches for artists which match the criteria given")}
        "searchalbum?" => {println!("Searches for artists which match the criteria given")}
        _ => {println!("Function not recognised")}
    }
}

fn is_numeric(number: &str) -> bool {
    /*
    Description: 
    Checks if a string is numeric
    Parameters:
    number: &str-the string to be checked
    Returns:
    bool-True if the string is numeric, false otherwise
    */
    let digits:[char; 10]=['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    //Checks every character of the string and if any of them aren't a denary digit, returns false
    for i in number.chars() {
        if digits.contains(&i) {}
        else {return false}
    }
    return true
}

fn is_two_digit(rankstr: &str) -> bool {
    /*
    Description:
    Checks if a ranking string has a two digit rank
    Parameters:
    rankstr: &str-The string to be checked
    Returns:
    bool-True if the string begins with two digits, false otherwise
    */
    //Calls is numeric on the second digit of the rank string
    let rankcharthesecond=rankstr.chars().nth(1).unwrap();
    let next_rank_str_no=String::from(rankcharthesecond.clone());
    let nrsnc=next_rank_str_no.as_str();
    let two_digit_true=is_numeric(nrsnc);
    if two_digit_true==true {return true}
    else {return false}
}

fn get_rankno(rank_str: &str) -> i32 {
    /*
    Description:
    Gets the number from the beginning of a ranking string
    Parameters:
    rank_str: &str-The string the number is extracted from
    Returns:
    i32-The number at the beginning of the string
    */
    //Returns the first character or two from the rank string 
    //depending on whether the number is one digit or two
    let rankchar=rank_str.chars().nth(0).unwrap();
    let mut rank_str_no=String::from(rankchar);
    let two_digit_true=is_two_digit(rank_str.clone());
    if two_digit_true==true {rank_str_no.push(rank_str.chars().nth(1).unwrap());}
    let rankno: i32=rank_str_no.parse().unwrap();
    return rankno
}

struct Artist {
    name: String,
    genre: String,
    members: Vec<String>,
    years_active: (i32, i32),
    description: String,
    albums: Vec<Album>
}

impl Default for Artist {
    fn default() -> Artist {
        Artist { name: String::from(""), genre: String::from(""), members: Vec::new(), years_active: (0, 0), description: String::from(""), albums: Vec::new()}
    }
}

impl ToString for Artist {
    fn to_string(&self)->String {
        println!("Artist: {} Genre: {}",  self.name, self.genre);
        println!("Years Active: {}-{}", self.years_active.0, self.years_active.1);
        println!("Members:");
        for i in self.members.clone() {println!("{} ", i)}
        println!("Albums:");
        for i in &self.albums {println!("{}", i.name);}
        println!("Description:");
        println!("{}", self.description);
        return String::from("");
    }
}

impl Clone for Artist {
    fn clone(&self) -> Self {
        return Artist { name: self.name.clone(), genre: self.genre.clone(), members: self.members.clone(), 
            years_active: self.years_active, description: self.description.clone(), albums: self.albums.clone()}
    }
}


struct Album {
    name: String,
    genre: String,
    artist: String,
    format: String,
    release_year: i32,
    record_dates: String,
    description: String
}

impl Default for Album {
    fn default () -> Album {
        Album { name: String::from(""), artist: String::from(""), release_year: 0, genre: String::from(""), format: String::from("Null"), 
        record_dates: String::from(""), description: String::from("") }
    }
}

impl ToString for Album {
    fn to_string(&self)->String {
        println!("Album: {} Genre: {} Format: {}",  self.name, self.genre, self.format);
        println!("Artist: {} Release year: {} Recording dates: {}", self.artist, self.release_year, self.record_dates);
        println!("{}", self.description);
        return String::from("");
    }
}

impl Clone for Album {
    fn clone(&self) -> Self {
        return Album {name: self.name.clone(), genre: self.genre.clone(), artist: self.artist.clone(), format: self.format.clone(), 
            release_year: self.release_year.clone(), record_dates: self.record_dates.clone(), 
            description: self.description.clone()}}
    }

/*
Unused functions:

fn print_rankings(rankings: &Vec<(String, Vec<String>, Vec<String>)>) {
    for i in rankings {
        print!("{} ", i.0);
        for n in &i.1 {print!("{} ", n)}
        for n in &i.2 {print!("{} ", n)}
        println!("");
    }
}

fn create_ex_ranking() -> (String, Vec<String>, Vec<String>) {
    return (String::from("TerminateProgram"), Vec::new(), Vec::new())
}

fn rankings_start(artists: &Vec<Artist>) -> Vec<(String, Vec<String>, Vec<String>)> {
    let mut rank=1;
    let mut remaining_albums=Vec::new();
    let mut ranking=Vec::new();
    let mut rankings=Vec::new();
    for i in artists {
        let albums=i.albums.clone();
        let albums_length=albums.len();
        for n in 0..albums_length {
           ranking.push(String::from("1. "));
            remaining_albums.push(albums[n].name.clone());
        }
        rankings.push((i.name.clone(), ranking.clone(), remaining_albums.clone()));
        ranking.clear();
        remaining_albums.clear();
    }
    return rankings;
}

fn find_album_rank(arr: &Vec<String>, thing: &str) ->(bool, usize) {
    let mut rank_idx: usize=0;
    for i in arr {
        let mut albumname=String::new();
        let mut idx=0;
        let rank=arr[rank_idx].clone();
        for (n, &item) in i.as_bytes().iter().enumerate() {
            let letter=item as char;
            if idx<2 {albumname.push(letter);}
            idx+=1;
        }
        if albumname.to_lowercase()==thing {return (true, idx);}
        rank_idx+=1;
    } 
    return (false, 10000);
}

*/

/*
Thanks for taking the time to read all my amazing comments
While writing them I listened to the following jazz albums:

Seven Steps To Heaven-Miles Davis
Africa/Brass-John Coltrane
Sunday At The Village Vanguard-Bill Evans
Maiden Voyage-Herbie Hancock
Etcetera-Wayne Shorter

All of which are excellent and I would recommend each
*/