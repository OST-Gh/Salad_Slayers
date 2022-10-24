use { std:: { fs::* , path::PathBuf }, lib::animate };

fn main ( ) {

    let home : PathBuf = match dirs::home_dir ( ) {

        Some ( value ) => {

            let mut path : PathBuf = value ;

            path .push ( "animation.txt" ) ;

            path

        } ,

        None => {

            println! ( "[ HOME Not Set ]" ) ;

            return ;

        } ,

    } ;

    let text : String = match read_to_string ( &home ) {

        Ok ( value ) => {

            let value = String::from ( value .split ( "run!" ) .collect:: < Vec < &str > > ( ) [ 1_usize ] ) ;

            value

        } ,

        Err ( _void ) => {

            println! ( "create {}" , home .display ( ) ) ;

            return ;

        } ,

    } ;

    let text : Vec < &str > = text .trim_start_matches ( "\n" ) .trim_end_matches ( "\n" ) .split ( "\n" ) .collect:: < Vec < &str > > ( ) ;

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

}