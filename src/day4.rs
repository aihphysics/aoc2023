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