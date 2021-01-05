use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<()> {
    let lines: Vec<String> = BufReader::new(File::open("input.txt")?).lines().map(|l| l.unwrap()).collect();
    let foods: Vec<Food> = lines.iter().map(|l| parse_food(l)).collect();

    // maps Allergen -> possible Ingredients
    let mut potential_ingr_for_allergen: HashMap<&str, HashSet<&str>> = HashMap::new();
    for food in &foods {
        let ingredient_set: HashSet<&str> = food.ingredients.iter().cloned().collect();
        for allergen in &food.allergens {
            if let Some(potential_ingr) = potential_ingr_for_allergen.get_mut(allergen) {
                // take the intersection of the existing ingredients and the ingredients for this new food
                *potential_ingr = potential_ingr.intersection(&ingredient_set).cloned().collect();
            } else {
                potential_ingr_for_allergen.insert(allergen, ingredient_set.clone());
            }
        }
    }

    // maps Ingredient -> Allergen
    let mut known_has_allergen: HashMap<&str, &str> = HashMap::new();
    while potential_ingr_for_allergen.len() > 0 {
        // singled out (allergen, ingredient) pairs
        let mut singled_out: Vec<(&str, &str)> = Vec::new();
        for (allergen, ingredients) in &potential_ingr_for_allergen {
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap();
                singled_out.push((allergen, ingredient));
                known_has_allergen.insert(ingredient, allergen);
            }
        }

        for (allergen, _) in &singled_out {
            potential_ingr_for_allergen.remove(allergen).expect("remove existing allergen");
        }
        for (_, ingredient) in &singled_out {
            for (_, ingr_set) in potential_ingr_for_allergen.iter_mut() {
                ingr_set.remove(ingredient);
                assert!(ingr_set.len() >= 1);
            }
        }
    }

    // Count how many ingredients in the input have no known allergen
    let mut safe_ingr_count = 0;
    for food in &foods {
        for ingredient in &food.ingredients {
            if !known_has_allergen.contains_key(ingredient) {
                safe_ingr_count += 1;
            }
        }
    }
    println!("Safe-ish ingredients appear {} times", safe_ingr_count);

    Ok(())
}

#[derive(Debug)]
struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

// return (Vec<Ingredients>, Vec<Allergens>)
fn parse_food(food_str: &str) -> Food {
    let paren_pos = food_str.chars().position(|c| c == '(').unwrap();
    let ingredient_list_str = &food_str[0..paren_pos-1];
    let allergen_list_str = &food_str[paren_pos+10..food_str.len()-1];

    let ingredient_vec: Vec<&str> = ingredient_list_str.split(" ").collect();
    let allergen_vec: Vec<&str> = allergen_list_str.split(", ").collect();
    Food {
        ingredients: ingredient_vec,
        allergens: allergen_vec,
    }
}
