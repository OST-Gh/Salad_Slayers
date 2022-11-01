use lib::* ;
use std::fs::read_to_string ;
use std::path::PathBuf ;

fn main ( ) -> Result < Void > {

    loop {

        let mut text : Vec < String > = Vec::new ( ) ;

        let mut quit : bool = false ;

        let mut debug : bool = false ;

        loop {

            match read_line ( Some ( "|  " ) ) {

                Result::Fine  ( value ) => {

                    if value == "@" {

                        quit = true ;

                        break ;

                    } ;

                    if value == "start!" {

                        break ;

                    } ;

                    if value .starts_with ( "debug!" ) {

                        debug = true ;

                    } ;

                    if value .starts_with ( "script!" ) {

                        let name : &str = value .trim ( ) .trim_start_matches ( "script!" ) .trim ( ) ;

                        let home : PathBuf = match dirs::home_dir ( ) {

                            Some ( value ) => {

                                let mut path : PathBuf = value ;

                                path .push ( &format! ( "{name}.txt" ) ) ;

                                path

                            } ,

                            None => {

                                return Result::Error ( Error::Environment ) ;

                            } ,

                        } ;

                        text = match read_to_string ( &home ) {

                            Ok ( text ) => {

                                text .split ( "\n" ) .map (

                                    | line : &str | -> String {

                                        String::from ( line )

                                    }

                                ) .collect:: < Vec < String > > ( )

                            } ,

                            Err ( _void ) => { return Result::Error ( Error::Read ) ; } ,

                        } ;

                        break ;

                    } ;

                    text .push ( value ) ;

                } ,

                Result::Error ( error ) => { return Result::Error ( error ) } ,

            } ;

        }
        
        if quit { break ; } ;

        text = text .into_iter ( ) .map ( | line : String | -> String { line .replace ( "\\n" , "\n" ) } ) .collect:: < Vec < String > > ( ) ;
 
        let start : usize = match text .clone ( ) .iter ( ) .position ( | text | text .trim ( ) == "run!" ) {

            Some ( value ) => { value                                      } ,
            None           => { return Result::Error ( Error::Position ) ; } ,

        } ;
        
        let stop  : usize = match text .clone ( ) .iter ( ) .position ( | text | text .trim ( ) == "end!" ) {

            Some ( value ) => { value                                      } ,
            None           => { return Result::Error ( Error::Position ) ; } ,

        } ;
        
        let mut all_frames     : Vec < Vec < Vec < ( String , u64 ) > > > = Vec::new ( ) ;
        let mut looping_frames : Vec < Vec <       ( String , u64 )   > > = Vec::new ( ) ;
        let mut frames         : Vec <             ( String , u64 )     > = Vec::new ( ) ;

        let mut is_loop : bool = false ;

        let mut range : usize = 0_usize ;

        for command in &text .clone ( ) [ start .. stop ] {

            let command : &str = command .trim ( ) ;

            if command .starts_with ( "#" ) {

                let command : Vec < &str > = command .trim_start_matches ( "#" ) .split ( ";" ) .collect:: < Vec < &str > > ( ) ;

                let duration : u64 = match command [ 0_usize ] .trim ( ) .parse:: < u64 > ( ) {

                    Ok  ( value ) => { value                                   } ,
                    Err ( _void ) => { return Result::Error ( Error::Parse ) ; } ,

                } ;

                if is_loop {

                    frames .push ( ( String::from ( command [ 1_usize ] .clone ( ) ) , duration ) ) ;

                } else {

                    all_frames .push ( vec! [ vec! [ ( String::from ( command [ 1_usize ] .clone ( ) ) , duration ) ] ] )

                }
                    

            } ;

            if command .ends_with ( "[" ) {

                is_loop = true ;

                range = match command .trim_end_matches ( "[") .trim ( ) .parse:: < usize > ( ) {

                    Ok  ( value ) => { value                                   } ,
                    Err ( _void ) => { return Result::Error ( Error::Parse ) ; } ,

                } ;

            } ;

            if command .starts_with ( "]" ) {

                is_loop = false ;

                let condition : usize = match command .trim_start_matches ( "]") .trim ( ) .parse:: < usize > ( ) {

                    Ok  ( value ) => { value                                   } ,
                    Err ( _void ) => { return Result::Error ( Error::Parse ) ; } ,

                } ;

                if condition != range { return Result::Error ( Error::Compare ) ; } ;

                for _void in 0_usize .. range {

                    looping_frames .push ( frames .clone ( ) ) ;

                }

                all_frames .push ( looping_frames .clone ( ) ) ;

            } ;

        }
        
        if debug {

            println! ( "{all_frames:#?}" ) ;

        }

        animate ( all_frames ) ;

        println! ( "" ) ;

    }

    return Result::Fine ( Void ) ;

}
