fn main() {
    let input = std::fs::read_to_string("input").expect("Error reading input file");
    let ingredients = parse_input(&input);
    println!("Answer #1 is {}", highest_score(&ingredients));
    println!("Answer #1 is {}", highest_score_500cal(&ingredients));
}

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            name: String::new(),
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }
}

impl std::ops::Add for &Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Self) -> Self::Output {
        Ingredient {
            name: String::new(),
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: 0,
        }
    }
}

impl std::ops::Mul<i64> for &Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i64) -> Self::Output {
        Ingredient{
            name: String::new(),
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    let mut ret = Vec::new();
    let rx = regex::Regex::new(r"^(\w+): capacity ([\-\d]+), durability ([\-\d]+), flavor ([\-\d]+), texture ([\-\d]+), calories ([\-\d]+)$").expect("Invalid regex");
    for line in input.lines() {
        if let Some(matches) = rx.captures(line) {
            let (_, [name, cap, dur, flav, tex, cal]) = matches.extract();
            ret.push(Ingredient {
                name: name.into(),
                capacity: cap.parse().unwrap(),
                durability: dur.parse().unwrap(),
                flavor: flav.parse().unwrap(),
                texture: tex.parse().unwrap(),
                calories: cal.parse().unwrap(),
            });
        } else {
            panic!("Unable to match line {}", line);
        }
    }
    ret
}

fn calculate_score(ingredients: &[Ingredient], amounts: &[i64]) -> i64 {
    assert_eq!(ingredients.len(), amounts.len());
    let mut acc = ingredients.iter()
        .zip(amounts)
        .fold(Ingredient::default(), |acc, (ingredient, &amount)|
            &acc + &(ingredient * amount)
        );
    acc.capacity = std::cmp::max(acc.capacity, 0);
    acc.durability = std::cmp::max(acc.durability, 0);
    acc.flavor = std::cmp::max(acc.flavor, 0);
    acc.texture = std::cmp::max(acc.texture, 0);
    acc.capacity * acc.durability * acc.flavor * acc.texture
}

fn calculate_calories(ingredients: &[Ingredient], amounts: &[i64]) -> i64 {
    ingredients.iter()
        .zip(amounts)
        .fold(0 as i64, |acc, (ingredient, amount)| 
            acc + (ingredient.calories * amount)
        )
}

fn enumerate_amounts_rec<F>(amounts: &mut [i64], index: usize, budget: i64, func: &mut F)
    where F: FnMut(&[i64])
{
    if index < amounts.len() - 1 {
        for i in 0..budget {
            amounts[index] = i;
            enumerate_amounts_rec(amounts, index + 1, budget - i, func);
        }
    } else {
        amounts[index] = budget;
        func(amounts);
    }
}

fn highest_score(ingredients: &[Ingredient]) -> i64 {
    let mut amounts = vec![0; ingredients.len()];
    let mut max_score = -1;
    enumerate_amounts_rec(&mut amounts, 0, 100, &mut |amounts: &[i64]| {
        let score = calculate_score(ingredients, amounts);
        max_score = std::cmp::max(max_score, score);
    });
    max_score
}

fn highest_score_500cal(ingredients: &[Ingredient]) -> i64 {
    let mut amounts = vec![0; ingredients.len()];
    let mut max_score = -1;
    enumerate_amounts_rec(&mut amounts, 0, 100, &mut |amounts: &[i64]| {
        if calculate_calories(&ingredients, &amounts) == 500 {
            let score = calculate_score(ingredients, amounts);
            max_score = std::cmp::max(max_score, score);
        }
    });
    max_score
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(&sample_input()), vec![
            Ingredient{ name: "Butterscotch".into(), capacity: -1, durability: -2, flavor: 6, texture: 3, calories: 8 },
            Ingredient{ name: "Cinnamon".into(), capacity: 2, durability: 3, flavor: -2, texture: -1, calories: 3 },
        ]);
    }

     #[test]
    fn test_calculate_score() {
        let ingredients = parse_input(&sample_input());
        assert_eq!(calculate_score(&ingredients, &[44, 56]), 62842880);
    }

    #[test]
    fn test_highest_score() {
        let ingredients = parse_input(&sample_input());
        assert_eq!(highest_score(&ingredients), 62842880);
    }
    
    #[test]
    fn test_calculate_calories() {
        let ingredients = parse_input(&sample_input());
        assert_eq!(calculate_calories(&ingredients, &[40, 60]), 500);
    }

    #[test]
    fn test_highest_score_500cal() {
        let ingredients = parse_input(&sample_input());
        assert_eq!(highest_score_500cal(&ingredients), 57600000);
    }
}
