use lib::* ;

fn main ( ) -> Result < Void > {

    let mut text : Vec < String > = Vec::new ( ) ;

    loop {

        match read_line ( Some ( "|  " ) ) {

            Result::Fine  ( value ) => {

                if &value == "end!" {

                    break ;

                } else {

                    text .push ( value ) ;

                }

            } ,

            Result::Error ( error ) => { return Result::Error ( error ) } ,

        } ;

    }

    let mut all_frames     : Vec < Vec < Vec < ( String , u64 ) > > > = Vec::new ( ) ;
    let mut looping_frames : Vec < Vec <       ( String , u64 )   > > = Vec::new ( ) ;
    let mut frames         : Vec <             ( String , u64 )     > = Vec::new ( ) ;

    let mut is_loop : bool = false ;

    let mut range : usize = 0_usize ;

    for command in text .clone ( ) {

        if is_loop && command .starts_with ( ":" ) {

            frames .push ( ( String::from ( command .trim_start_matches ( ":" ) ) , 100_u64 ) ) ;

        } ;

        if command .ends_with ( "[" ) {

            is_loop = true ;

            range = command .trim_end_matches ( "[" ) .parse:: < usize > ( ) .expect ( "[ Unparsible Loop Number ]" ) ;

        } ;

        if command .starts_with ( "]" ) {

            is_loop = false ;

            for _void in 0_usize .. range {

                looping_frames .push ( frames .clone ( ) ) ;

            }

            all_frames .push ( looping_frames .clone ( ) ) ;

        } ;

        if !is_loop && command .starts_with ( ":" ) {

            all_frames .push ( vec! [ vec! [ ( String::from ( command .trim_start_matches ( ":" ) ) , 100_u64 ) ] ] )

        }

    }

    animate ( all_frames ) ;

    println! ( "" ) ;

    return Result::Fine ( Void ) ;

}