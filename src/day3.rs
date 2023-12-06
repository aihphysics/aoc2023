use grid::*;
struct Coordinate { x: usize, y: usize }

// try this with indexed_iter -> might be a LOT simpler
fn get_symbols( cgrd:&Grid<char> ) -> Vec<Coordinate>{
    cgrd.iter_rows()
        .enumerate()
        .map( | ( y, row ) | {
            row.enumerate()
            .filter( | ( _, char_col ) | **char_col != '.' && char_col.is_ascii_punctuation() )
            .map( | ( x, _ ) | {
                Coordinate{ x, y }
            }
            ).collect::<Vec<Coordinate>>()
    } ).into_iter().flatten().collect::<Vec<Coordinate>>()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {

    let x_max = input.find('\n').expect("no linebreaks");
    let vecd: Vec<char> = input.chars().filter( |c| *c != '\n' ).collect();
    let cgrd = Grid::from_vec( vecd, x_max );
    let y_max = cgrd.cols()-1;
    let symbols: Vec<Coordinate> = get_symbols( &cgrd );
    let mut part_acc = 0u32;
    let mut num_vec: Vec<char> = Vec::with_capacity( 3 );

    for coord in symbols {

        let rightmost = if coord.x+1 >= x_max {x_max} else { coord.x+1 }; 
        let leftmost = coord.x.saturating_sub( 1 );
        let uppermost = coord.y.saturating_sub( 1 ) ;
        let lowermost = if coord.y+1 >= y_max {y_max} else { coord.y+1 }; 
        
        
        for y in uppermost..lowermost+1 {
            let mut deacc: usize = 0;
            for x in leftmost..rightmost+1 {
                if deacc > 0 { deacc -= 1; continue; }
                if cgrd[ (y , x) ].is_numeric() {
                    let mut right_step = x+1;
                    let mut left_step = x-1;
                    while cgrd[ (y, right_step) ].is_numeric() && right_step < x_max {  
                        right_step += 1;
                        if right_step == x_max { break; }
                    } 
                    loop {  
                        if ( !cgrd[ (y, left_step ) ].is_numeric() ){ left_step+=1; break; }
                        else if ( left_step == 0 ){ break; }
                        left_step -= 1;
                    };
                    deacc = right_step - x;
                    for number_idx in left_step..right_step{
                        num_vec.push( cgrd[ ( y, number_idx ) ] );
                    }
                    part_acc += num_vec.iter().collect::<String>().parse::<u32>().unwrap();
                    num_vec.clear()
                }
            }
        }
    }
    part_acc
}