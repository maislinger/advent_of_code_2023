pub fn solve(input: &str) -> anyhow::Result<String> {
    let mut sum_part_one: u64 = 0;

    let mut sum_part_two: u64 = 0;

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let mut max = Set::zeros();

        let (game, rest) = split_game(line)?;

        let mut do_add = true;
        for set in set_iter(rest) {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                do_add = false;
            }

            if max.red < set.red {
                max.red = set.red;
            }

            if max.green < set.green {
                max.green = set.green;
            }

            if max.blue < set.blue {
                max.blue = set.blue;
            }
        }

        if do_add {
            sum_part_one += game;
        }

        sum_part_two += max.prod();
    }

    Ok(format!(
        "02/01 = {}, 02/02 = {}\n",
        sum_part_one, sum_part_two
    ))
}

#[derive(Debug)]
struct Set {
    red: u64,
    green: u64,
    blue: u64,
}

impl Set {
    fn zeros() -> Self {
        Set {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn prod(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

fn split_game(s: &str) -> anyhow::Result<(u64, &str)> {
    let mut iter = s.split(": ");

    let game = iter.next().ok_or(anyhow::anyhow!("invalid input"))?.trim();
    let game_number: u64 = game
        .split("Game ")
        .nth(1)
        .ok_or(anyhow::anyhow!("invalid input"))?
        .parse()?;

    let rest = iter.next().ok_or(anyhow::anyhow!("invalid input"))?.trim();
    Ok((game_number, rest))
}

fn set_iter(s: &str) -> impl Iterator<Item = Set> + '_ {
    s.split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|p| {
            let mut r = Set {
                red: 0,
                green: 0,
                blue: 0,
            };
            for c in p.split(',').map(|s| s.trim()) {
                let mut inner_iter = c.split_whitespace();
                let n: u64 = inner_iter.next().unwrap().parse().unwrap();

                let color_str = inner_iter.next().unwrap().trim();
                match color_str {
                    "red" => r.red = n,
                    "green" => r.green = n,
                    "blue" => r.blue = n,
                    _ => unreachable!(),
                }
            }
            r
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d02() {
        let input = "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        let solution = solve(input).unwrap();
        assert_eq!(solution, "02/01 = 8, 02/02 = 2286\n");
    }
}
