use rating::{CalculateRating, EloStrategy, GameResult, Rating};

fn main() {
    let alice = Rating::from(1200);
    let bob = Rating::from(1453);

    let elo_calculator = EloStrategy::new(40);
    // the game result it's seen from the first player perspective, in this case Alice
    let (alice_new_rating, bob_new_rating) = elo_calculator.calculate(alice, bob, GameResult::Win);

    println!("Alice: {alice_new_rating}");
    println!("Bob: {bob_new_rating}");

    // Alice: 1232.4
    // Bob: 1420.6
}
