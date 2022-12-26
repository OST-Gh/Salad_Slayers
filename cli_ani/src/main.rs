use lib::* ;
use std::fs::read_to_string ;
use std::path::PathBuf ;

fn main ( ) -> Result < Void > {

    loop {

        let mut text : Vec < String > = Vec::new ( ) ;

        let mut quit : bool = false ;

        let mut debug : bool = false ;

        loop {

            match read_line ( "|  " , | _void : char | -> bool { false } ) {

                Result::Good ( value ) => {

                    if value == "@" {

                        quit = true ;

                        break ;

                    } ;

                    if value == "start!" { break ; } ;

                    if value .starts_with ( "debug!" ) { debug = true ; } ;

                    if value .starts_with ( "script!" ) {

                        let name : &str = value .trim ( ) .trim_start_matches ( "script!" ) .trim ( ) ;

                        let Some ( mut path ) : Option < PathBuf > = dirs::home_dir ( ) else {

                            return Result::Evil ( Error::Environment ) ;

                        } ;

                        path .push ( &format! ( "{name}.txt" ) ) ; 

                        let Ok ( lines ) : std::result::Result < String > = read_to_string ( &path ) else {

                            return Result::Evil ( Error::Read ) ;

                        } ;

                        text = lines .split ( "\n" ) .map ( | line : &str | -> String { String::from ( line ) } ) .collect:: < Vec < String > > ( ) ;

                        break ;

                    } ;

                    text .push ( value ) ;

                } ,

                Result::Evil ( error ) => { return Result::Evil ( error ) } ,

            } ;

        }
        
        if quit { break ; } ;

        text = text .into_iter ( ) .map ( | line : String | -> String { line .replace ( "\\n" , "\n" ) } ) .collect:: < Vec < String > > ( ) ;

        let start : usize = match text .clone ( ) .iter ( ) .position ( | text | text .trim ( ) == "run!" ) {

            Some ( value ) => { value                                     } ,
            None           => { return Result::Evil ( Error::Position ) ; } ,

        } ;
        
        let stop  : usize = match text .clone ( ) .iter ( ) .position ( | text | text .trim ( ) == "end!" ) {

            Some ( value ) => { value                                      } ,
            None           => { return Result::Evil ( Error::Position ) ; } ,

        } ;
        
        let mut frames : Vec < ( String , u64 ) > = Vec::new ( ) ;
        let mut buffer : Vec < ( String , u64 ) > = Vec::new ( ) ;

        let mut is_loop : bool = false ;

        let mut range : usize = 0_usize ;

        for command in &text .clone ( ) [ start .. stop ] {

            let command : &str = command .trim ( ) ;

            if command .starts_with ( "#" ) {

                let command : Vec < &str > = command .trim_start_matches ( "#" ) .split ( ";" ) .collect:: < Vec < &str > > ( ) ;

                let length : usize = command [ 1_usize ] .clone ( ) .chars ( ) .count ( ) ;

                let Some ( start ) = command [ 1_usize ] .clone ( ) .chars ( )          .position ( | character | character == '"' ) else { return Result::Evil ( Error::Position ) ; } ;
                let Some ( end   ) = command [ 1_usize ] .clone ( ) .chars ( ) .rev ( ) .position ( | character | character == '"' ) else { return Result::Evil ( Error::Position ) ; } ;

                let duration : u64 = match command [ 0_usize ] .trim ( ) .parse:: < u64 > ( ) {

                    Ok  ( value ) => { value                                   } ,
                    Err ( _void ) => { return Result::Evil ( Error::Parse ) ; } ,

                } ;

                if is_loop {

                    buffer .push ( ( String::from ( &( command [ 1_usize ] .clone ( ) [ start + 1_usize .. length - end - 1_usize ] ) ) , duration ) ) ;

                } else {

                    frames .push ( ( String::from ( &( command [ 1_usize ] .clone ( ) [ start + 1_usize .. length - end - 1_usize ] ) ) , duration ) ) ;

                }
                    

            } ;

            if command .ends_with ( "[" ) {

                is_loop = !( is_loop ) ;

                range = match command .trim_end_matches ( "[") .trim ( ) .parse:: < usize > ( ) {

                    Ok  ( value ) => { value                                  } ,
                    Err ( _void ) => { return Result::Evil ( Error::Parse ) ; } ,

                } ;

            } ;

            if command .starts_with ( "]" ) {

                is_loop = !( is_loop ) ;

                let condition : usize = match command .trim_start_matches ( "]") .trim ( ) .parse:: < usize > ( ) {

                    Ok  ( value ) => { value                                  } ,
                    Err ( _void ) => { return Result::Evil ( Error::Parse ) ; } ,

                } ;

                if condition != range { return Result::Evil ( Error::Compare ) ; } ;

                for _void in 0_usize .. range {

                    for frame in buffer .clone ( ) {

                        frames .push ( frame .clone ( ) ) ;

                    }

                }

            } ;

        }
        
        if debug {

            println! ( "{frames:#?}" ) ;

        }

        let mut timings : std::vec::IntoIter < u64 > = frames .clone ( ) .into_iter ( ) .map ( | frame : ( String , u64 ) | -> u64 { frame .1 } ) .collect:: < Vec < u64 > > ( ) .into_iter ( ) ;

        let frames  : Vec < String > = frames .clone ( ) .into_iter ( ) .map ( | frame : ( String , u64 ) | -> String { frame .0 } ) .collect:: < Vec < String > > ( ) ;

        let Some ( last ) : Option < String > = frames .clone ( ) .into_iter ( ) .last ( ) else {

            return Result::Evil ( Error::Position ) ;

        } ;

        animate! (

            frames => last ; {

                let Some ( time ) = timings .next ( ) else { break ; } ;

                std::thread::sleep ( std::time::Duration::from_millis ( time ) ) ;

            }

        ) ;

        println! ( "" ) ;

    }

    return Result::Good( Void ) ;

}
