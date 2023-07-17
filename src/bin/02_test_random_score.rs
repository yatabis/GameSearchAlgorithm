use std::fmt::{Debug, Formatter};
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

const H: usize = 3;
const W: usize = 4;
const END_TURN: i32 = 4;

#[derive(Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct MazeState {
    points: [[i32; W]; H],
    turn: i32,
    character: Coord,
    game_score: i32,
}

#[allow(non_upper_case_globals)]
impl MazeState {
    const dx: [i32; 4] = [1, -1, 0, 0];
    const dy: [i32; 4] = [0, 0, 1, -1];

    fn new(seed: u64) -> Self {
        let mut rng_for_construct = if seed < u64::MAX {
            SmallRng::seed_from_u64(seed)
        } else {
            SmallRng::from_entropy()
        };
        let y = rng_for_construct.next_u32() as usize % H;
        let x = rng_for_construct.next_u32() as usize % W;
        let character = Coord { x, y };
        let mut points = [[0; W]; H];
        for y in 0..H {
            for x in 0..W {
                if y == character.y && x == character.x { continue; }
                points[y][x] = rng_for_construct.next_u32() as i32 % 10;
            }
        }
        Self {
            points,
            turn: 0,
            character,
            game_score: 0,
        }
    }

    fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    fn advance(&mut self, action: usize) {
        self.character.x = (self.character.x as i32 + Self::dx[action]) as usize;
        self.character.y = (self.character.y as i32 + Self::dy[action]) as usize;
        if self.points[self.character.y][self.character.x] > 0 {
            self.game_score += self.points[self.character.y][self.character.x];
            self.points[self.character.y][self.character.x] = 0;
        }
        self.turn += 1;
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = Vec::new();
        for action in 0..4 {
            let ty = self.character.y as i32 + Self::dy[action];
            let tx = self.character.x as i32 + Self::dx[action];
            if ty >= 0 && ty < H as i32 && tx >= 0 && tx < W as i32 {
                actions.push(action);
            }
        }
        actions
    }
}

impl Debug for MazeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = (0..H).map(|h| {
            (0..W).map(|w| {
                if self.character.y == h && self.character.x == w {
                    "@"
                } else if self.points[h][w] > 0 {
                    ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][self.points[h][w] as usize]
                } else {
                    "."
                }
            }).collect::<Vec<_>>().join("")
        }).collect::<Vec<_>>().join("\n");
        writeln!(f, "turn:\t{}\nscore:\t{}\n{}", self.turn, self.game_score, s)
    }
}

type State = MazeState;

fn random_action(state: &State, rng: &mut SmallRng) -> usize {
    let legal_actions = state.legal_actions();
    legal_actions[rng.next_u32() as usize % legal_actions.len()]
}

fn test_ai_score(game_number: i32) {
    let mut rng_for_action = SmallRng::seed_from_u64(0);
    let mut rng_for_construct = SmallRng::seed_from_u64(0);
    let mut score_mean = 0.0;
    for _ in 0..game_number {
        let mut state = MazeState::new(rng_for_construct.next_u64());
        while !state.is_done() {
            state.advance(random_action(&state, &mut rng_for_action));
        }
        let score = state.game_score;
        score_mean += score as f64;
    }
    score_mean /= game_number as f64;
    println!("Score:\t{score_mean}");
}

fn main() {
    test_ai_score(100);
}