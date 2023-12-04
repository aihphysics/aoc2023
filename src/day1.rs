use regex::Regex;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let lines = input.split( "\n" );
    let mut acc: u32 = 0;
    for line in lines{
        let mut nums = line.chars()
            .filter( |&c| c.is_digit( 10 ) )
            .map( |x| (x as u32)-48 );
        let mut temp =  nums.nth( 0 ).unwrap();
        temp = match nums.last(){
            Some( value ) => temp*10 + value,
            None => temp*11,
        };
        acc += temp;
    }
    acc
}
fn word_to_char(s:&str) -> char{
    match s {
        "one" => '1', "two" => '2', "three" => '3',
        "four" => '4', "five" => '5', "six" => '6',
        "seven" => '7', "eight" => '8', "nine" => '9',
        _ => '0'
    }
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let lines = input.split( "\n" );
    let mut acc: u32 = 0;
    let rgx = Regex::new( r"(one|two|three|four|five|six|seven|eight|nine)" ).unwrap();
    for line in lines{
        let mut shift = 0;
        let mut strline: String = line.to_string();
        let finds = rgx.find_iter( line );
        for found in finds{
            strline.insert( found.start()+shift, word_to_char( found.as_str() ) );
            shift+=1;
        }
        let mut nums = strline.chars()
            .into_iter()
            .filter( |&c| c.is_digit( 10 ) )
            .map( |x| (x as u32)-48 );
        let mut temp =  nums.nth( 0 ).unwrap(); 
        temp = match nums.last(){
            Some( value ) => temp*10 + value,
            None => temp*11,
        };
        acc += temp;
    }
    acc
}

