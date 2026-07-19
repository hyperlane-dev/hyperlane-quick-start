use super::*;

/// Implementation of methods for `GomokuDomain`.
impl GomokuDomain {
    /// Creates a new gomoku game room with the specified owner as the black player.
    ///
    /// # Arguments
    ///
    /// - `&str`: The room identifier.
    /// - `&str`: The owner's user identifier.
    ///
    /// # Returns
    ///
    /// - `GomokuRoom`: The newly created room with initialized board and waiting status.
    #[instrument_trace]
    pub fn create_room(room_id: &str, owner_id: &str) -> GomokuRoom {
        let mut room: GomokuRoom = GomokuRoom::default();
        let mut owner: GomokuPlayer = GomokuPlayer::default();
        let mut players: Vec<GomokuPlayer> = vec![];
        owner
            .set_user_id(owner_id.to_string())
            .set_color(StoneColor::Black);
        players.push(owner);
        room.set_room_id(room_id.to_string())
            .set_owner_id(owner_id.to_string())
            .set_players(players)
            .set_status(GameStatus::Waiting)
            .set_next_turn(StoneColor::Black);
        Self::ensure_board(&mut room);
        room
    }

    /// Adds a player to the room, assigning the opposite color of existing players.
    ///
    /// # Arguments
    ///
    /// - `&mut GomokuRoom`: The room to add the player to.
    /// - `&str`: The user identifier of the player to add.
    ///
    /// # Returns
    ///
    /// - `Result<StoneColor, String>`: The assigned stone color, or an error if the room is full.
    #[instrument_trace]
    pub fn add_player(room: &mut GomokuRoom, user_id: &str) -> Result<StoneColor, String> {
        if let Some(color) = Self::get_player_color(room, user_id) {
            return Ok(color);
        }
        if room.get_players().len() >= 2 {
            return Err("Room is full".to_string());
        }
        let mut player: GomokuPlayer = GomokuPlayer::default();
        let color: StoneColor = if room.get_players().is_empty() {
            StoneColor::Black
        } else {
            StoneColor::White
        };
        player.set_user_id(user_id.to_string()).set_color(color);
        room.get_mut_players().push(player);
        Ok(color)
    }

    /// Adds a spectator to the room if they are not already a player or spectator.
    ///
    /// # Arguments
    ///
    /// - `&mut GomokuRoom`: The room to add the spectator to.
    /// - `&str`: The user identifier of the spectator.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the spectator was added, `false` if they are already in the room.
    #[instrument_trace]
    pub fn add_spectator(room: &mut GomokuRoom, user_id: &str) -> bool {
        if Self::get_player_color(room, user_id).is_some() {
            return false;
        }
        if room
            .get_spectators()
            .iter()
            .any(|item: &String| item == user_id)
        {
            return false;
        }
        room.get_mut_spectators().push(user_id.to_string());
        true
    }

    /// Removes a user from the room (as player or spectator), ending the game if a player leaves during play.
    ///
    /// # Arguments
    ///
    /// - `&mut GomokuRoom`: The room to remove the user from.
    /// - `&str`: The user identifier to remove.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the user was found and removed, `false` otherwise.
    #[instrument_trace]
    pub fn remove_user(room: &mut GomokuRoom, user_id: &str) -> bool {
        let mut removed: bool = false;
        if let Some(index) = room
            .get_players()
            .iter()
            .position(|player: &GomokuPlayer| player.get_user_id() == user_id)
        {
            room.get_mut_players().remove(index);
            removed = true;
        }
        if let Some(index) = room
            .get_spectators()
            .iter()
            .position(|spectator: &String| spectator == user_id)
        {
            room.get_mut_spectators().remove(index);
            removed = true;
        }
        if removed && room.get_status() == &GameStatus::InProgress {
            room.set_status(GameStatus::Finished);
            let winner: Option<StoneColor> = room
                .get_players()
                .first()
                .map(|player: &GomokuPlayer| *player.get_color());
            room.set_winner(winner);
        }
        removed
    }

    /// Starts the game if exactly two players are present, initializing the game board.
    ///
    /// # Arguments
    ///
    /// - `&mut GomokuRoom`: The room to start the game in.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if waiting for the second player.
    #[instrument_trace]
    pub fn start_game(room: &mut GomokuRoom) -> Result<(), String> {
        if room.get_players().len() != 2 {
            return Err("Waiting for second player".to_string());
        }
        Self::ensure_board(room);
        room.set_status(GameStatus::InProgress);
        Ok(())
    }

    /// Ensures the game board is a valid 15x15 grid, replacing it with an empty board if invalid.
    ///
    /// # Arguments
    ///
    /// - `&mut GomokuRoom`: The room whose board should be validated.
    #[instrument_trace]
    pub fn ensure_board(room: &mut GomokuRoom) {
        let size: usize = 15;
        let mut is_valid: bool = true;
        let board: &Vec<Vec<u8>> = room.get_board();
        if board.len() != size {
            is_valid = false;
        } else {
            for row in board.iter() {
                let row_len: usize = row.len();
                if row_len != size {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            return;
        }
        let new_board: Vec<Vec<u8>> = Self::build_empty_board(size);
        *room.get_mut_board() = new_board;
    }

    /// Builds an empty square board of the given size filled with zeros.
    ///
    /// # Arguments
    ///
    /// - `usize`: The size of the board (width and height).
    ///
    /// # Returns
    ///
    /// - `Vec<Vec<u8>>`: The empty board.
    #[instrument_trace]
    fn build_empty_board(size: usize) -> Vec<Vec<u8>> {
        let mut board: Vec<Vec<u8>> = vec![];
        let row: Vec<u8> = vec![0; size];
        let mut index: usize = 0;
        while index < size {
            board.push(row.clone());
            index += 1;
        }
        board
    }

    /// Places a stone on the board for the specified player at the given position.
    ///
    /// Validates the game state, turn order, and position before placing the stone.
    /// Checks for a win condition (five in a row) or a draw after each placement.
    ///
    /// # Arguments
    ///
    /// - `&mut GomokuRoom`: The room where the stone is placed.
    /// - `&str`: The user identifier of the player placing the stone.
    /// - `usize`: The x-coordinate (column) of the position.
    /// - `usize`: The y-coordinate (row) of the position.
    ///
    /// # Returns
    ///
    /// - `Result<GomokuPlaceResult, String>`: The result of the placement including game status and winner, or an error.
    #[instrument_trace]
    pub fn place_stone(
        room: &mut GomokuRoom,
        user_id: &str,
        x: usize,
        y: usize,
    ) -> Result<GomokuPlaceResult, String> {
        if room.get_status() != &GameStatus::InProgress {
            return Err("Game is not in progress".to_string());
        }
        Self::ensure_board(room);
        let player_color: StoneColor =
            Self::get_player_color(room, user_id).ok_or("Player not found".to_string())?;
        if &player_color != room.get_next_turn() {
            return Err("Not your turn".to_string());
        }
        let board_len: usize = room.get_board().len();
        if y >= board_len {
            return Err("Invalid position".to_string());
        }
        let row_len: usize = room.get_board()[y].len();
        if x >= row_len {
            return Err("Invalid position".to_string());
        }
        let step: usize = room.get_moves().len() + 1;
        let value: u8 = player_color.to_value();
        {
            let board: &mut Vec<Vec<u8>> = room.get_mut_board();
            if board[y][x] != 0 {
                return Err("Position occupied".to_string());
            }
            board[y][x] = value;
        }
        let mut move_data: GomokuMove = GomokuMove::default();
        move_data
            .set_x(x)
            .set_y(y)
            .set_color(player_color)
            .set_step(step);
        room.get_mut_moves().push(move_data.clone());
        let board: &[Vec<u8>] = room.get_board();
        let mut result: GomokuPlaceResult = GomokuPlaceResult::default();
        result.set_move_data(move_data);
        if Self::check_five(board, x, y, value) {
            room.set_status(GameStatus::Finished);
            room.set_winner(Some(player_color));
            result
                .set_status(GameStatus::Finished)
                .set_winner(Some(player_color))
                .set_is_draw(false);
            return Ok(result);
        }
        if Self::is_board_full(board) {
            room.set_status(GameStatus::Finished);
            room.set_winner(None);
            result
                .set_status(GameStatus::Finished)
                .set_winner(None)
                .set_is_draw(true);
            return Ok(result);
        }
        room.set_next_turn(player_color.opposite());
        result
            .set_status(GameStatus::InProgress)
            .set_winner(None)
            .set_is_draw(false);
        Ok(result)
    }

    /// Returns the stone color assigned to a specific user in the room.
    ///
    /// # Arguments
    ///
    /// - `&GomokuRoom`: The room to search.
    /// - `&str`: The user identifier.
    ///
    /// # Returns
    ///
    /// - `Option<StoneColor>`: The stone color if the user is a player, or `None`.
    #[instrument_trace]
    fn get_player_color(room: &GomokuRoom, user_id: &str) -> Option<StoneColor> {
        for player in room.get_players().iter() {
            if player.get_user_id() == user_id {
                return Some(*player.get_color());
            }
        }
        None
    }

    /// Checks if the board is completely filled with no empty positions.
    ///
    /// # Arguments
    ///
    /// - `&[Vec<u8>]`: The game board.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the board is full, `false` otherwise.
    #[instrument_trace]
    fn is_board_full(board: &[Vec<u8>]) -> bool {
        for row in board.iter() {
            if row.contains(&0) {
                return false;
            }
        }
        true
    }

    /// Checks if placing a stone at the given position creates five in a row.
    ///
    /// Examines four directions (horizontal, vertical, and two diagonals) from the position.
    ///
    /// # Arguments
    ///
    /// - `&[Vec<u8>]`: The game board.
    /// - `usize`: The x-coordinate of the position.
    /// - `usize`: The y-coordinate of the position.
    /// - `u8`: The stone value to check.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if five in a row is detected, `false` otherwise.
    #[instrument_trace]
    fn check_five(board: &[Vec<u8>], x: usize, y: usize, value: u8) -> bool {
        let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];
        for (dx, dy) in directions.iter() {
            let mut count: usize = 1;
            count += Self::count_direction(board, x, y, *dx, *dy, value);
            count += Self::count_direction(board, x, y, -*dx, -*dy, value);
            if count >= 5 {
                return true;
            }
        }
        false
    }

    /// Counts consecutive stones of the same value in a single direction from a starting position.
    ///
    /// # Arguments
    ///
    /// - `&[Vec<u8>]`: The game board.
    /// - `usize`: The x-coordinate of the starting position.
    /// - `usize`: The y-coordinate of the starting position.
    /// - `isize`: The x-direction step.
    /// - `isize`: The y-direction step.
    /// - `u8`: The stone value to match.
    ///
    /// # Returns
    ///
    /// - `usize`: The count of consecutive matching stones in that direction.
    #[instrument_trace]
    fn count_direction(
        board: &[Vec<u8>],
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
        value: u8,
    ) -> usize {
        let mut count: usize = 0;
        let mut cx: isize = x as isize + dx;
        let mut cy: isize = y as isize + dy;
        let board_len: isize = board.len() as isize;
        while cx >= 0 && cy >= 0 && cx < board_len && cy < board_len {
            let row: &Vec<u8> = &board[cy as usize];
            if cx as usize >= row.len() {
                break;
            }
            if row[cx as usize] != value {
                break;
            }
            count += 1;
            cx += dx;
            cy += dy;
        }
        count
    }
}
