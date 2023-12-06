use grid::*;
struct Window { right: usize, left: usize, upper: usize, lower: usize }

#[inline(always)]
fn part1_pred(c: char) -> bool { c != '.' && !c.is_ascii_digit() }

#[inline(always)]
fn part2_pred(c: char) -> bool { c == '*' }

#[inline(always)]
fn part1_oper( nums: Vec<u32> ) -> u32 { nums.iter().sum() }

#[inline(always)]
fn part2_oper( nums: Vec<u32> ) -> u32 { 
    if nums.len() == 2 { nums[0]*nums[1] }
    else { 0 }
}

fn make_window ( x: usize, y: usize, x_max: usize, y_max: usize ) -> Window {
    Window { 
        right: if x+1 >= x_max {x_max} else { x+1 } + 1, left: x.saturating_sub( 1 ),
        upper: y.saturating_sub( 1 ), lower: if y+1 >= y_max {y_max} else { y+1 } + 1,
    }
}

fn get_windows ( cgrd: &Grid<char>, x_max: usize, y_max: usize, pred: impl Fn(char) -> bool ) ->Vec< Window >{
    cgrd.indexed_iter()
        .filter( | ( _, chr ) |
            pred( **chr )
        ).map( | ( ( row, col ), _ ) | {
            make_window( col, row, x_max, y_max )
        } ).collect::<Vec<Window>>()
}

fn search_window( cgrd: &Grid<char>, wnd: &Window, x_max: usize, oper: impl Fn( Vec<u32> ) -> u32 ) -> u32 {
    let mut num_char_vec: Vec<char> = Vec::with_capacity( 3 );
    let mut nums: Vec<u32> = Vec::with_capacity( 9 );
    for y in wnd.upper..wnd.lower {
            let mut deacc: usize = 0;
            for x in wnd.left..wnd.right {
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
                        num_char_vec.push( cgrd[ ( y, number_idx ) ] );
                    }
                    nums.push( num_char_vec.iter().collect::<String>().parse::<u32>().unwrap() );
                    num_char_vec.clear()
                }
            }
        }
    oper( nums )
}

fn parse_in( input: &str ) -> ( usize, usize, Grid<char> ) {
    let x_max = input.find('\n').expect("no linebreaks");
    let vecd: Vec<char> = input.chars().filter( |c| *c != '\n' ).collect();
    let cgrd = Grid::from_vec( vecd, x_max );
    let y_max = cgrd.cols()-1;
    ( x_max, y_max, cgrd )
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let ( x_max, y_max, cgrd ) = parse_in( input );
    let windows: Vec<Window> = get_windows( &cgrd, x_max, y_max, part1_pred );
    windows.iter().map( |wnd| { search_window( &cgrd, wnd, x_max, part1_oper ) }).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let ( x_max, y_max, cgrd ) = parse_in( input );
    let windows: Vec<Window> = get_windows( &cgrd, x_max, y_max, part2_pred );
    windows.iter().map( |wnd| { search_window( &cgrd, wnd, x_max, part2_oper ) }).sum()
}