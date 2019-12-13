use std::str::from_utf8;

fn hash_layer(layer_chars: &Vec<char>) -> (usize, usize, usize){
    let counts:Vec<usize> = vec!('0', '1', '2').iter().map(|c| count_chars(layer_chars, c)).collect();
    (counts[0], counts[1], counts[2])
}

fn count_chars(layer_chars: &Vec<char>, c: &char) -> usize {
    layer_chars.iter().filter(|c1| *c1 == c).count()
}

fn hash_layers(chars_chunked: Vec<String>) -> Vec<(usize, usize, usize)> {
    chars_chunked.iter().map(|c| hash_layer(&c.chars().collect())).collect()
}

fn chunk_string(input: &String, s: usize) -> Vec<String> {
    input.as_bytes().chunks(s).map(|c| from_utf8(c).unwrap().to_string()).collect()
}

pub fn day8_part1(input: &String, width: usize, height: usize) -> usize {
    let mut hashed_layers = hash_layers(chunk_string(input, width * height));
    hashed_layers.sort_by_key(|layer_data| layer_data.0);
    hashed_layers.iter().map(|layer_data| layer_data.1 * layer_data.2).next().unwrap()

}

pub fn day8_part2(input: &String, width: usize, height: usize) -> Vec<String>{
    let layers = input.len() / width / height;
    let layer_size = width * height;
    let bytes = input.as_bytes();
    let mut image = Vec::with_capacity(height);
    for y in 0..height {
        let mut line = String::with_capacity(width);
        for x in 0..width {
            let mut char='2';
            for l in 0..layers {
                let index = x + (width * y) +(layer_size * l);
                char = match char {
                    '2' => match bytes[index] as char {
                        '1' => '*',
                        '0' => ' ',
                        _ => '2',
                    },
                    c => c,
                };
            }
            line.push(char);
        }
        image.push(line);
    }
    image
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    use super::*;

    #[test]
    fn test_day8_part1() {
        assert_eq!(day8_part1(&"123456789012".to_string(), 3, 2),1);
    }

    #[test]
    fn test_day8_part1_test2() {
        assert_eq!(day8_part1(&"123123012012".to_string(), 3, 2),4);
    }

    #[test]
    fn test_day8_part1_assignment() {
        let f = File::open("input8.txt").unwrap();
        let file = BufReader::new(&f);
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(day8_part1(lines[0].as_ref().unwrap(), 25, 6),1820);

    }

    #[test]
    fn test_day8_part2_assignment() {
        let f = File::open("input8.txt").unwrap();
        let file = BufReader::new(&f);
        let lines: Vec<_> = file.lines().collect();
        day8_part2(lines[0].as_ref().unwrap(), 25, 6);
    }

}
