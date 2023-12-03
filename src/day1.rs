
#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let lines = input.split( "\n" );
    let mut acc: u32 = 0;
    for line in lines{
        let mut nums = line.chars()
            .into_iter()
            .filter( |&c| c.is_digit( 10 ) )
            .map( |x| (x as u32)-48 );
        let mut temp =  nums.nth( 0 ).unwrap();
        match nums.last(){
            Some( value ) => temp = temp*10 + value,
            None => temp += temp*10,
        }
        acc += temp;
    }
    acc
}

//#[aoc(day1, part2)]
//pub fn solve_part1(input: &str) -> u32 {