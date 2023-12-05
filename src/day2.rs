fn parse_line( line: &str ) -> ( u32, u32, u32 ) {
    let ( _, game ) = line.split_once( ":" ).unwrap();
    let handfuls = game.split( [ ',', ';' ] );
    let (mut r, mut g, mut b ) : ( u32, u32, u32 ) = ( 0, 0, 0 );
    for clr_num in handfuls {
        if let Some ( ( num, color ) ) = clr_num.trim().split_once( " " ){
            let parsed_number= num.parse::<u32>().unwrap();
            match color.chars().next().unwrap() {
                'r' => if parsed_number > r { r = parsed_number },
                'g' => if parsed_number > g { g = parsed_number },
                'b' => if parsed_number > b { b = parsed_number }, 
                _ => continue,
            };
        }
        else { continue; }
    }
    ( r, g, b )
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines()
        .enumerate()
        .filter_map( |(id, line) | {
            let ( r, g, b ) = parse_line( line );
            ( r <= 12 && g <= 13 && b <= 14 ).then_some( id + 1 )
        }).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input.lines()
        .map( | line | {
            let ( r, g, b ) = parse_line( line );
            r * g * b
        }).sum()
}