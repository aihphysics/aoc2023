#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> u32 {

    let pairs: Vec< ( &str, &str )> = input.lines().map( | line | {
            ( &line[10..39], &line[42..] )
        }).collect();

    let game = pairs.iter().map( | ( a , b ) | {
        (   a.trim().split( ' ' ).filter_map(  | draw | {
                    match draw.parse::<u32>() {
                        Ok ( value ) => Some( value ),
                        Err ( _ ) => None
                    }
                }
            ).collect::<Vec<u32>>(),
            b.trim().split( ' ' ).filter_map(  | game | {
                    match  game.trim().parse::<u32>() {
                        Ok ( value ) => Some( value ),
                        Err ( _ ) => None
                    }
                }
            ).collect::<Vec<u32>>()
        )
    } );
        
    let base: u32 = 2;
    game.map( | ( draw, mut targets ) |{
        targets.sort();
        draw.iter().map( |x| {
            match targets.binary_search( x ) {
                Ok( _ ) => 1,
                Err( _ ) => 0,
            }
        }).fold( 0u32, | mut acc, res | { acc += res; acc } )
    }).filter( |&x| x > 0 ).map( |x| { 
        base.pow( x.saturating_sub(1) )
    } ).sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u32 {

    let pairs: Vec< ( &str, &str )> = input.lines().map( | line | {
            ( &line[10..39], &line[42..] )
        }).collect();

    let game = pairs.iter().map( | ( a , b ) | {
        (   a.trim().split( ' ' ).filter_map(  | draw | {
                    match draw.parse::<u32>() {
                        Ok ( value ) => Some( value ),
                        Err ( _ ) => None
                    }
                }
            ).collect::<Vec<u32>>(),
            b.trim().split( ' ' ).filter_map(  | game | {
                    match  game.trim().parse::<u32>() {
                        Ok ( value ) => Some( value ),
                        Err ( _ ) => None
                    }
                }
            ).collect::<Vec<u32>>()
        )
    } );
        
    let matches = game.map( | ( draw, mut targets ) |{
        targets.sort();
        draw.iter().map( |x| {
            match targets.binary_search( x ) {
                Ok( _ ) => 1,
                Err( _ ) => 0,
            }
        }).fold( 0usize, | mut acc, res | { acc += res; acc } )
    });

    let mut copies = vec![ 1u32; pairs.len() ];
    for ( start, mtch ) in matches.enumerate() { 
        for index in start+1..start+1+mtch {
            copies[ index ] += copies[ start ]
        }
    }
    copies.iter().sum()

    // so i have a vector of matches and a vector of copies, m and c
    // start by zipping these together
    // then for each match, reach forwards for the next m cards
    // add c to each
}