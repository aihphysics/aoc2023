fn to_block(  string_block: &str ) -> Vec<i64> {
    string_block.split( [' ', '\n' ] )
        .filter_map( |x| {
            x.parse::<i64>().ok()
        } 
    ).collect()
}

/*
current_seeds, next_seeds, mappedd
for block in blocks {
    for mapping in mappings {
        for seed in seeds { 
            if seed in mapping
                nextseeds[ seedindex ]  = map( seed )
        }
    } 
    seeds = nextseeds
}
*/

struct Seedmap{ source: i64, destination: i64, length:i64, offset: i64 }

impl Seedmap {

    fn from_slice( input: &[i64] ) -> Seedmap {
        //let difference: i64 = match input[0].checked_sub( input[1] ){
        //    Some( result ) => result as i64,
        //    None => panic!( "uh oh" )
        //};
        Seedmap { source: input[1], destination: input[0], length:input[2], offset: input[0] - input[1] }
    }

    fn map ( &mut self, seed: i64 ) -> Option< i64 > {
        if seed > self.source && seed < self.source + self.length{
            Some( seed + self.offset )
        } else {
            None
        }
    }
}


#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i64 {

    let mut lines = input.lines();
    //let seeds = lines.nth( 0 ).unwrap().rsplit_once( ':' ).unwrap().0.split( ' ' ).map( |x| { x.parse::<i64>() } );
    //let seeds : Vec<i64> = lines.nth( 0 ).unwrap().split_once( ':' ).unwrap().1.split( ' ' ).map( |x| { 
    let seeds : Vec<i64> = lines.nth( 0 ).unwrap().split_once( ':' ).unwrap().1.trim().split( ' ' ).map( |x| {
            x.parse::<i64>().unwrap()
        }
    ).collect();
    lines.nth( 1 );

    let mut input_blocks = input.split( "\n\n" );
    input_blocks.next();

    let blocks: Vec< _ > = input_blocks.map( |x| to_block( x ) ).collect();

    let mut current_seeds = seeds.clone();
    let mut next_seeds = seeds.clone();
    let mut mapped = vec![ false; current_seeds.len() ];

    blocks.iter()
        .for_each( |block| {
            block.chunks( 3 )
            .map( Seedmap::from_slice )
            .for_each( | mut mapping | {
                for ( idx, seed ) in current_seeds.iter().enumerate() {
                    if let Some( newseed ) = mapping.map( *seed ) {
                        next_seeds[ idx ] = newseed;
                        mapped[ idx ] = true;
                    }
                }
            } );
            current_seeds = next_seeds.clone();
            mapped = vec![ false; current_seeds.len() ];
        } );
    *current_seeds.iter().min().unwrap()
    
}