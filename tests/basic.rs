use gtest::{Program, System};
use pebbles_game_io::*;
const OVER: u32 = 1;

#[test]
fn test_with_positive_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();
    let sender_id = 50;

    // Setting of the pebbles game
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 4,
    };
    system.mint_to(sender_id, 100000000000000);
    program.send(sender_id, init_message);
    system.run_next_block();
    // Check the initial state of the GameState
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    // Player's (User) turn
    program.send(sender_id, PebblesAction::Turn(3));
    system.run_next_block();
    // Check the state after user's turn
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    assert!(state.pebbles_remaining < 7);

    // Player gives up
    program.send(sender_id, PebblesAction::GiveUp);
    system.run_next_block();
    // Check the state after player gives up
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after giving up");

    assert_eq!(state.winner, Some(Player::Program));

    // Restart the game
    let restart_message = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 10,
    };

    program.send(sender_id, restart_message);
    system.run_next_block();
    // Check the state after restart
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after restart");

    assert!(state.pebbles_remaining <= 15);
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.winner, None);
}

#[test]
fn test_with_negative_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();
    let sender_id = 50;

    // Setting of the pebbles game
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 3,
        max_pebbles_per_turn: 4,
    };
    system.mint_to(sender_id, 100000000000000);
    program.send(sender_id, init_message);
    system.run_next_block();
    // Check the initial state of the GameState
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 3);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    // Player's (User) turn
    program.send(sender_id, PebblesAction::Turn(1));
    //assert!(state.max_pebbles_per_turn <= -5);
    system.run_next_block();
    // Check the state after user's turn
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    //assert!(state.pebbles_remaining < 0);
    if state.pebbles_remaining < OVER {
        // Restart the game
        let restart_message = PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 15,
            max_pebbles_per_turn: 10,
        };

        program.send(sender_id, restart_message);
        system.run_next_block();
        // Check the state after restart
        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after restart");

        assert!(state.pebbles_remaining <= 15);
        assert_eq!(state.pebbles_count, 15);
        assert_eq!(state.max_pebbles_per_turn, 10);
        assert_eq!(state.difficulty, DifficultyLevel::Hard);
        assert_eq!(state.winner, None);
    } else {
        // Player gives up
        program.send(sender_id, PebblesAction::GiveUp);
        system.run_next_block();
        // Check the state after player gives up
        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after giving up");
        assert_eq!(state.winner, Some(Player::Program));
    }
}

#[test]
fn test_with_illegal_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();
    let sender_id = 50;
    let pebbles_count = 10;

    // Setting of the pebbles game
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count,
        max_pebbles_per_turn: 4,
    };
    system.mint_to(sender_id, 100000000000000);
    program.send(sender_id, init_message);
    system.run_next_block();
    // Check the initial state of the GameState
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    // Player's (User) turn
    program.send(sender_id, PebblesAction::Turn(12));
    system.run_next_block();
    assert!(state.max_pebbles_per_turn <= 12);

    // Check the state after user's turn
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    //assert!(state.pebbles_remaining < 0);
    if state.pebbles_remaining >= OVER {
        // Player gives up
        program.send(sender_id, PebblesAction::GiveUp);
        system.run_next_block();
        // Check the state after player gives up
        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after giving up");

        assert_eq!(state.winner, Some(Player::Program));
    };

    // Restart the game
    let restart_message = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 10,
    };

    program.send(sender_id, restart_message);
    system.run_next_block();
    // Check the state after restart
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after restart");

    assert!(state.pebbles_remaining <= 15);
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.winner, None);
}
