pub fn initialize_block(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let block_pattern = vec![
        (0, 0), (0, 1),
        (1, 0), (1, 1),
    ];

    for (dx, dy) in block_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_blinker(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let blinker_pattern = vec![
        (0, 0),
        (1, 0),
        (2, 0),
    ];

    for (dx, dy) in blinker_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_toad(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let toad_pattern = vec![
        (1, 0), (2, 0), (3, 0),
        (0, 1), (1, 1), (2, 1),
    ];

    for (dx, dy) in toad_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_beacon(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let beacon_pattern = vec![
        (0, 0), (0, 1),
        (1, 0), (1, 1),
        (2, 2), (2, 3),
        (3, 2), (3, 3),
    ];

    for (dx, dy) in beacon_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_pulsar(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let pulsar_pattern = vec![
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];

    for (dx, dy) in pulsar_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_glider(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let glider_pattern = vec![
        (0, 1),
        (1, 2),
        (2, 0), (2, 1), (2, 2),
    ];

    for (dx, dy) in glider_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_spaceship(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let spaceship_pattern = vec![
        (0, 1), (0, 2), (0, 3), (0, 4),
        (1, 0), (1, 4),
        (2, 4),
        (3, 0), (3, 3),
    ];

    for (dx, dy) in spaceship_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_glider_gun(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let gun_pattern = vec![
        (0, 24),
        (1, 22), (1, 24),
        (2, 12), (2, 13), (2, 20), (2, 21), (2, 34), (2, 35),
        (3, 11), (3, 15), (3, 20), (3, 21), (3, 34), (3, 35),
        (4, 0), (4, 1), (4, 10), (4, 16), (4, 20), (4, 21),
        (5, 0), (5, 1), (5, 10), (5, 14), (5, 16), (5, 17), (5, 22), (5, 24),
        (6, 10), (6, 16), (6, 24),
        (7, 11), (7, 15),
        (8, 12), (8, 13),
    ];

    for (dx, dy) in gun_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_beehive(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let beehive_pattern = vec![
        (0, 1), (0, 2),
        (1, 0), (1, 3),
        (2, 1), (2, 2),
    ];

    for (dx, dy) in beehive_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_pentadecathlon(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let pentadecathlon_pattern = vec![
        (0, 1), (0, 3),
        (1, 0), (1, 2), (1, 4),
        (2, 1), (2, 3),
        (3, 1), (3, 3),
        (4, 0), (4, 2), (4, 4),
        (5, 1), (5, 3),
    ];

    for (dx, dy) in pentadecathlon_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}

pub fn initialize_puffer_train(board: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    let puffer_train_pattern = vec![
        (0, 2), (0, 3), (0, 4), (0, 6), (0, 7),
        (1, 1), (1, 5), (1, 8),
        (2, 0), (2, 1), (2, 5), (2, 9),
        (3, 0), (3, 5), (3, 9),
        (4, 5), (4, 9),
        (5, 1), (5, 5), (5, 8),
        (6, 1), (6, 2), (6, 3), (6, 4), (6, 6), (6, 7), (6, 8),
    ];

    for (dx, dy) in puffer_train_pattern {
        if y + dy < board.len() && x + dx < board[0].len() {
            board[y + dy][x + dx] = 1;
        }
    }
}