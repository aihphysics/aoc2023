
#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let lines = input.split( "\n" );
    let mut acc: u32 = 0;
    for line in lines{
        println!( "{}", line );
        let mut nums = line.chars()
            .into_iter()
            .filter( |&c| !c.is_alphabetic() )
            .map( |x| (x as u32)-48 );
        let mut temp =  10*nums.nth( 0 ).unwrap();
        match nums.last(){
            Some( value ) => temp += value,
            None => temp += temp/10,
        }
        acc += temp;
    }
    acc
}