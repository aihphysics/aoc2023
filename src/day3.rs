use grid::*;
struct Window { right: usize, left: usize, upper: usize, lower: usize }

 fn make_window ( x: usize, y: usize, x_max: usize, y_max: usize ) -> Window {
    Window { 
        right: if x+1 >= x_max {x_max} else { x+1 } + 1, left: x.saturating_sub( 1 ),
        upper: y.saturating_sub( 1 ), lower: if y+1 >= y_max {y_max} else { y+1 } + 1,
    }
 }

fn get_windows ( cgrd: &Grid<char>, x_max: usize, y_max: usize ) ->Vec< Window >{
    cgrd.indexed_iter()
        .filter( | ( _, chr ) |
            **chr != '.' && chr.is_ascii_punctuation() 
        ).map( | ( ( row, col ), _ ) | {
            make_window( col, row, x_max, y_max )
        } ).collect::<Vec<Window>>()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {

    let x_max = input.find('\n').expect("no linebreaks");
    let vecd: Vec<char> = input.chars().filter( |c| *c != '\n' ).collect();
    let cgrd = Grid::from_vec( vecd, x_max );
    let y_max = cgrd.cols()-1;
    let windows: Vec<Window> = get_windows( &cgrd, x_max, y_max );

    let mut part_acc = 0u32;
    let mut num_vec: Vec<char> = Vec::with_capacity( 3 );

    // but how to do it so that its not 'c-like'?
    for window in windows {
        for y in window.upper..window.lower {
            let mut deacc: usize = 0;
            for x in window.left..window.right {
                if deacc > 0 { deacc -= 1; continue; }
                if cgrd[ (y , x) ].is_numeric() {
                    let mut right_step = x+1;
                    let mut left_step = x-1;
                    while cgrd[ (y, right_step) ].is_numeric() && right_step < x_max {  
                        right_step += 1;
                        if right_step == x_max { break; }
                    } 
                    loop {  
                        if !cgrd[ (y, left_step ) ].is_numeric() { left_step+=1; break; }
                        else if left_step == 0 { break; }
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