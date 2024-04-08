use rand::Rng;

fn main() {
    // figure out number of shots
    let shots = 10;
    let balistic_skill = 3;
    let strength = 5;
    let toughness = 5;
    let armour_save = 3;
    let armour_penetration = 1;

    // Start loop
    let mut total_hits: Vec<i32> = Vec::new();
    let mut total_wounds: Vec<i32> = Vec::new();
    let mut total_failed_saves: Vec<i32> = Vec::new();

    // Loop to get a bunch of results to work out an average
    for _ in 0..100000 {
        // Make shots
        let successful_hits = hit(shots, balistic_skill);

        // Make wounds
        let successful_wounds = wound(&successful_hits, strength, toughness);

        // Make saves
        let failed_saves = save(&successful_wounds, armour_save, armour_penetration);

        // add results to totals
        total_hits.push(successful_hits);
        total_wounds.push(successful_wounds);
        total_failed_saves.push(failed_saves);
    }

    match average(&total_hits) {
        Some(avg) => println!("Average number of hits was {:?}", avg),
        None => println!("No successful hits"),
    }
    match average(&total_wounds) {
        Some(avg) => println!("Average number of wounds was {:?}", avg),
        None => println!("No successful wounds"),
    }
    match average(&total_failed_saves) {
        Some(avg) => println!("Average number of failed saves was {:?}", avg),
        None => println!("No failed saves"),
    }
}

fn d6() -> i32 {
    rand::thread_rng().gen_range(1..6)
}

fn hit(shots: i32, bs: i32) -> i32 {
    let mut successful_hits: Vec<i32> = Vec::new();
    for _ in 0..shots {
        let hit = d6();
        if hit >= bs {
            successful_hits.push(hit)
        }
    }
    successful_hits.len() as i32
}

fn wound(hits: &i32, strength: i32, toughness: i32) -> i32 {
    let mut successful_wounds: i32 = 0;
    let mut target_score: i32;
    for _ in 0..*hits {
        let wound = d6();

        if toughness >= (strength * 2) {
            // Is toughness twice the strength or more
            target_score = 6
        } else if toughness > strength {
            // Is toughness greater than strength
            target_score = 5
        } else if strength == toughness {
            // is toughness equil to strengh
            target_score = 4
        } else if strength >= (toughness * 2) {
            // is toughness half of strength
            target_score = 2
        } else {
            // if no other match then toughness is less than strengh but not half.
            target_score = 3
        }

        if wound >= target_score {
            successful_wounds += 1
        }
    }
    successful_wounds
}

fn save(wounds: &i32, armour_save: i32, armour_penetration: i32) -> i32 {
    let mut failed_saves = 0;
    for _ in 0..*wounds {
        let save = d6();
        if save >= armour_save + armour_penetration {
            failed_saves += 1;
        }
    }
    failed_saves
}

fn average(list: &[i32]) -> Option<i32> {
    let sum: i32 = list.iter().sum();
    let count: i32 = list.len() as i32;
    if count > 0 {
        Some(sum / count)
    } else {
        None
    }
}
