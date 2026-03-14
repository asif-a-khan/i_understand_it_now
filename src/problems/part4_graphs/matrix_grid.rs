use rand::Rng;

use crate::problems::{Difficulty, Problem, SolutionResult, TestCase};
use crate::solutions::part4_graphs::matrix_grid as solutions;
use crate::tracker::OperationLog;

pub fn problems() -> Vec<Box<dyn Problem>> {
    vec![
        Box::new(MatrixGridFloodFill),
        Box::new(MatrixGridIslandPerimeter),
        Box::new(MatrixGridMaxAreaIsland),
        Box::new(MatrixGridCountIslands),
        Box::new(MatrixGridSurroundedRegions),
        Box::new(MatrixGridRottingOranges),
        Box::new(MatrixGridWallsAndGates),
        Box::new(MatrixGrid01Matrix),
        Box::new(MatrixGridWordSearch),
        Box::new(MatrixGridUniquePaths),
        Box::new(MatrixGridShortestPathObstacles),
        Box::new(MatrixGridSwimInWater),
        Box::new(MatrixGridMakingLargeIsland),
        Box::new(MatrixGridLongestIncreasingPath),
        Box::new(MatrixGridTreasureIsland),
    ]
}

// ── Easy 1: Flood Fill ──────────────────────────────────────────────────

struct MatrixGridFloodFill;

struct FloodFillTest {
    image: Vec<Vec<i32>>,
    sr: usize,
    sc: usize,
    new_color: i32,
}

impl Problem for MatrixGridFloodFill {
    fn id(&self) -> &str { "matrix_grid_flood_fill" }
    fn name(&self) -> &str { "Flood Fill" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given an m x n integer grid `image`, a starting pixel (sr, sc), and a `new_color`, \
         perform a flood fill. Starting from (sr, sc), color all connected pixels that share \
         the same original color with `new_color`. Pixels are connected 4-directionally \
         (up, down, left, right).\n\n\
         Return the modified image.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 50\n\
         - 0 <= image[i][j], new_color <= 65535\n\
         - 0 <= sr < m, 0 <= sc < n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(1..=8);
            let cols = rng.random_range(1..=8);
            let image: Vec<Vec<i32>> = (0..rows)
                .map(|_| (0..cols).map(|_| rng.random_range(0..=3)).collect())
                .collect();
            let sr = rng.random_range(0..rows);
            let sc = rng.random_range(0..cols);
            let new_color = rng.random_range(0..=5);
            TestCase { data: Box::new(FloodFillTest { image, sr, sc, new_color }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<FloodFillTest>().unwrap();
        let expected = ref_flood_fill(&t.image, t.sr, t.sc, t.new_color);
        let actual = solutions::flood_fill(&t.image, t.sr, t.sc, t.new_color);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!(
                "image={:?}, sr={}, sc={}, new_color={}", t.image, t.sr, t.sc, t.new_color
            ),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_flood_fill(image: &[Vec<i32>], sr: usize, sc: usize, new_color: i32) -> Vec<Vec<i32>> {
    let mut result = image.to_vec();
    let orig = result[sr][sc];
    if orig == new_color {
        return result;
    }
    fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, orig: i32, new: i32) {
        if grid[r][c] != orig {
            return;
        }
        grid[r][c] = new;
        if r > 0 { dfs(grid, r - 1, c, orig, new); }
        if r + 1 < grid.len() { dfs(grid, r + 1, c, orig, new); }
        if c > 0 { dfs(grid, r, c - 1, orig, new); }
        if c + 1 < grid[0].len() { dfs(grid, r, c + 1, orig, new); }
    }
    dfs(&mut result, sr, sc, orig, new_color);
    result
}

// ── Easy 2: Island Perimeter ────────────────────────────────────────────

struct MatrixGridIslandPerimeter;

struct IslandPerimeterTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for MatrixGridIslandPerimeter {
    fn id(&self) -> &str { "matrix_grid_island_perimeter" }
    fn name(&self) -> &str { "Island Perimeter" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a row x col grid where grid[i][j] = 1 represents land and 0 represents \
         water, determine the perimeter of the island. There is exactly one island (one or \
         more connected land cells). The island does not have lakes.\n\n\
         Each land cell is a 1x1 square. The perimeter is the total number of edges of land \
         cells that touch water or the grid boundary.\n\n\
         Constraints:\n\
         - 1 <= row, col <= 100\n\
         - grid[i][j] is 0 or 1\n\
         - There is exactly one island"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=8);
            let cols = rng.random_range(2..=8);
            let mut grid = vec![vec![0i32; cols]; rows];
            // Grow a connected island using BFS from a random start
            let sr = rng.random_range(0..rows);
            let sc = rng.random_range(0..cols);
            grid[sr][sc] = 1;
            let size = rng.random_range(1..=(rows * cols / 2).max(1));
            let mut frontier: Vec<(usize, usize)> = vec![(sr, sc)];
            let mut count = 1;
            while count < size && !frontier.is_empty() {
                let idx = rng.random_range(0..frontier.len());
                let (r, c) = frontier[idx];
                let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
                for (dr, dc) in dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        if grid[nr][nc] == 0 && count < size {
                            grid[nr][nc] = 1;
                            frontier.push((nr, nc));
                            count += 1;
                        }
                    }
                }
                if frontier.len() > 1 {
                    frontier.swap_remove(idx);
                }
            }
            TestCase { data: Box::new(IslandPerimeterTest { grid }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<IslandPerimeterTest>().unwrap();
        let expected = ref_island_perimeter(&t.grid);
        let actual = solutions::island_perimeter(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_island_perimeter(grid: &[Vec<i32>]) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut perimeter = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 1 {
                perimeter += 4;
                if r > 0 && grid[r - 1][c] == 1 { perimeter -= 2; }
                if c > 0 && grid[r][c - 1] == 1 { perimeter -= 2; }
            }
        }
    }
    perimeter
}

// ── Easy 3: Max Area of Island ──────────────────────────────────────────

struct MatrixGridMaxAreaIsland;

struct MaxAreaIslandTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for MatrixGridMaxAreaIsland {
    fn id(&self) -> &str { "matrix_grid_max_area_island" }
    fn name(&self) -> &str { "Max Area of Island" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given a binary grid where 1 represents land and 0 represents water, return the \
         maximum area of an island. An island is a group of 1s connected 4-directionally. \
         If there is no island, return 0.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 50\n\
         - grid[i][j] is 0 or 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=8);
            let cols = rng.random_range(2..=8);
            let grid: Vec<Vec<i32>> = (0..rows)
                .map(|_| (0..cols).map(|_| rng.random_range(0..=1)).collect())
                .collect();
            TestCase { data: Box::new(MaxAreaIslandTest { grid }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MaxAreaIslandTest>().unwrap();
        let expected = ref_max_area_island(&t.grid);
        let actual = solutions::max_area_island(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_max_area_island(grid: &[Vec<i32>]) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut max_area = 0;

    fn dfs(grid: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, r: usize, c: usize) -> i32 {
        if visited[r][c] || grid[r][c] == 0 {
            return 0;
        }
        visited[r][c] = true;
        let mut area = 1;
        if r > 0 { area += dfs(grid, visited, r - 1, c); }
        if r + 1 < grid.len() { area += dfs(grid, visited, r + 1, c); }
        if c > 0 { area += dfs(grid, visited, r, c - 1); }
        if c + 1 < grid[0].len() { area += dfs(grid, visited, r, c + 1); }
        area
    }

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 1 && !visited[r][c] {
                max_area = max_area.max(dfs(grid, &mut visited, r, c));
            }
        }
    }
    max_area
}

// ── Easy 4: Number of Islands ───────────────────────────────────────────

struct MatrixGridCountIslands;

struct CountIslandsTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for MatrixGridCountIslands {
    fn id(&self) -> &str { "matrix_grid_count_islands" }
    fn name(&self) -> &str { "Number of Islands" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given an m x n binary grid where 1 represents land and 0 represents water, return \
         the number of islands. An island is surrounded by water and is formed by connecting \
         adjacent lands horizontally or vertically.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 300\n\
         - grid[i][j] is 0 or 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=10);
            let cols = rng.random_range(2..=10);
            let grid: Vec<Vec<i32>> = (0..rows)
                .map(|_| (0..cols).map(|_| rng.random_range(0..=1)).collect())
                .collect();
            TestCase { data: Box::new(CountIslandsTest { grid }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<CountIslandsTest>().unwrap();
        let expected = ref_count_islands(&t.grid);
        let actual = solutions::count_islands(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_count_islands(grid: &[Vec<i32>]) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut count = 0;

    fn dfs(grid: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, r: usize, c: usize) {
        if visited[r][c] || grid[r][c] == 0 {
            return;
        }
        visited[r][c] = true;
        if r > 0 { dfs(grid, visited, r - 1, c); }
        if r + 1 < grid.len() { dfs(grid, visited, r + 1, c); }
        if c > 0 { dfs(grid, visited, r, c - 1); }
        if c + 1 < grid[0].len() { dfs(grid, visited, r, c + 1); }
    }

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 1 && !visited[r][c] {
                count += 1;
                dfs(grid, &mut visited, r, c);
            }
        }
    }
    count
}

// ── Easy 5: Surrounded Regions ──────────────────────────────────────────

struct MatrixGridSurroundedRegions;

struct SurroundedRegionsTest {
    board: Vec<Vec<char>>,
}

impl Problem for MatrixGridSurroundedRegions {
    fn id(&self) -> &str { "matrix_grid_surrounded_regions" }
    fn name(&self) -> &str { "Surrounded Regions" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Easy }
    fn description(&self) -> &str {
        "Given an m x n board containing 'X' and 'O', capture all regions that are \
         4-directionally surrounded by 'X'. A region is captured by flipping all 'O's \
         into 'X's in that surrounded region.\n\n\
         An 'O' on the border (or connected to a border 'O') cannot be captured.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 200\n\
         - board[i][j] is 'X' or 'O'"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(3..=8);
            let cols = rng.random_range(3..=8);
            let board: Vec<Vec<char>> = (0..rows)
                .map(|_| {
                    (0..cols)
                        .map(|_| if rng.random_range(0..3) == 0 { 'O' } else { 'X' })
                        .collect()
                })
                .collect();
            TestCase { data: Box::new(SurroundedRegionsTest { board }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SurroundedRegionsTest>().unwrap();
        let expected = ref_surrounded_regions(&t.board);
        let actual = solutions::surrounded_regions(&t.board);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("board={:?}", t.board),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_surrounded_regions(board: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = board.len();
    let cols = board[0].len();
    let mut result = board.to_vec();

    fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
        if grid[r][c] != 'O' {
            return;
        }
        grid[r][c] = 'S'; // safe (connected to border)
        if r > 0 { dfs(grid, r - 1, c); }
        if r + 1 < grid.len() { dfs(grid, r + 1, c); }
        if c > 0 { dfs(grid, r, c - 1); }
        if c + 1 < grid[0].len() { dfs(grid, r, c + 1); }
    }

    // Mark all O's connected to border
    for r in 0..rows {
        if result[r][0] == 'O' { dfs(&mut result, r, 0); }
        if result[r][cols - 1] == 'O' { dfs(&mut result, r, cols - 1); }
    }
    for c in 0..cols {
        if result[0][c] == 'O' { dfs(&mut result, 0, c); }
        if result[rows - 1][c] == 'O' { dfs(&mut result, rows - 1, c); }
    }

    for r in 0..rows {
        for c in 0..cols {
            match result[r][c] {
                'O' => result[r][c] = 'X', // captured
                'S' => result[r][c] = 'O', // restore safe
                _ => {}
            }
        }
    }
    result
}

// ── Medium 6: Rotting Oranges ───────────────────────────────────────────

struct MatrixGridRottingOranges;

struct RottingOrangesTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for MatrixGridRottingOranges {
    fn id(&self) -> &str { "matrix_grid_rotting_oranges" }
    fn name(&self) -> &str { "Rotting Oranges" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "In a grid, each cell can have one of three values:\n\
         - 0: empty cell\n\
         - 1: fresh orange\n\
         - 2: rotten orange\n\n\
         Every minute, any fresh orange adjacent (4-directionally) to a rotten orange \
         becomes rotten. Return the minimum number of minutes until no fresh orange remains. \
         Return -1 if impossible.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 10\n\
         - grid[i][j] is 0, 1, or 2"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=6);
            let cols = rng.random_range(2..=6);
            let grid: Vec<Vec<i32>> = (0..rows)
                .map(|_| (0..cols).map(|_| rng.random_range(0..=2)).collect())
                .collect();
            TestCase { data: Box::new(RottingOrangesTest { grid }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<RottingOrangesTest>().unwrap();
        let expected = ref_rotting_oranges(&t.grid);
        let actual = solutions::rotting_oranges(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_rotting_oranges(grid: &[Vec<i32>]) -> i32 {
    use std::collections::VecDeque;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut g = grid.to_vec();
    let mut queue = VecDeque::new();
    let mut fresh = 0;

    for r in 0..rows {
        for c in 0..cols {
            if g[r][c] == 2 {
                queue.push_back((r, c));
            } else if g[r][c] == 1 {
                fresh += 1;
            }
        }
    }

    if fresh == 0 {
        return 0;
    }

    let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
    let mut minutes = 0;

    while !queue.is_empty() {
        let size = queue.len();
        let mut rotted_any = false;
        for _ in 0..size {
            let (r, c) = queue.pop_front().unwrap();
            for (dr, dc) in &dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    if g[nr][nc] == 1 {
                        g[nr][nc] = 2;
                        fresh -= 1;
                        queue.push_back((nr, nc));
                        rotted_any = true;
                    }
                }
            }
        }
        if rotted_any {
            minutes += 1;
        }
    }

    if fresh > 0 { -1 } else { minutes }
}

// ── Medium 7: Walls and Gates ───────────────────────────────────────────

struct MatrixGridWallsAndGates;

struct WallsAndGatesTest {
    rooms: Vec<Vec<i32>>,
}

const INF: i32 = 2_147_483_647;

impl Problem for MatrixGridWallsAndGates {
    fn id(&self) -> &str { "matrix_grid_walls_and_gates" }
    fn name(&self) -> &str { "Walls and Gates" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an m x n grid `rooms` initialized with these three possible values:\n\
         - -1: a wall or obstacle\n\
         - 0: a gate\n\
         - INF (2147483647): an empty room\n\n\
         Fill each empty room with the distance to its nearest gate. If it is impossible \
         to reach a gate, leave it as INF.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 250\n\
         - rooms[i][j] is -1, 0, or 2147483647"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=8);
            let cols = rng.random_range(2..=8);
            let rooms: Vec<Vec<i32>> = (0..rows)
                .map(|_| {
                    (0..cols)
                        .map(|_| {
                            let r = rng.random_range(0..=4);
                            match r {
                                0 => 0,       // gate
                                1 => -1,      // wall
                                _ => INF,     // empty room
                            }
                        })
                        .collect()
                })
                .collect();
            TestCase { data: Box::new(WallsAndGatesTest { rooms }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WallsAndGatesTest>().unwrap();
        let expected = ref_walls_and_gates(&t.rooms);
        let actual = solutions::walls_and_gates(&t.rooms);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("rooms={:?}", t.rooms),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_walls_and_gates(rooms: &[Vec<i32>]) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;
    let rows = rooms.len();
    let cols = rooms[0].len();
    let mut result = rooms.to_vec();
    let mut queue = VecDeque::new();

    for r in 0..rows {
        for c in 0..cols {
            if result[r][c] == 0 {
                queue.push_back((r, c));
            }
        }
    }

    let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if result[nr][nc] == INF {
                    result[nr][nc] = result[r][c] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
    result
}

// ── Medium 8: 01 Matrix ────────────────────────────────────────────────

struct MatrixGrid01Matrix;

struct ZeroOneMatrixTest {
    mat: Vec<Vec<i32>>,
}

impl Problem for MatrixGrid01Matrix {
    fn id(&self) -> &str { "matrix_grid_01_matrix" }
    fn name(&self) -> &str { "01 Matrix" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an m x n binary matrix `mat`, return the distance of the nearest 0 for each \
         cell. The distance between two adjacent cells is 1 (4-directional).\n\n\
         Constraints:\n\
         - 1 <= m, n <= 10000\n\
         - mat[i][j] is 0 or 1\n\
         - There is at least one 0 in the matrix"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=8);
            let cols = rng.random_range(2..=8);
            let mut mat: Vec<Vec<i32>> = (0..rows)
                .map(|_| (0..cols).map(|_| rng.random_range(0..=1)).collect())
                .collect();
            // Ensure at least one zero
            let zr = rng.random_range(0..rows);
            let zc = rng.random_range(0..cols);
            mat[zr][zc] = 0;
            TestCase { data: Box::new(ZeroOneMatrixTest { mat }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ZeroOneMatrixTest>().unwrap();
        let expected = ref_01_matrix(&t.mat);
        let actual = solutions::zero_one_matrix(&t.mat);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("mat={:?}", t.mat),
            expected: format!("{expected:?}"),
            actual: format!("{actual:?}"),
        }
    }
}

fn ref_01_matrix(mat: &[Vec<i32>]) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;
    let rows = mat.len();
    let cols = mat[0].len();
    let mut dist = vec![vec![i32::MAX; cols]; rows];
    let mut queue = VecDeque::new();

    for r in 0..rows {
        for c in 0..cols {
            if mat[r][c] == 0 {
                dist[r][c] = 0;
                queue.push_back((r, c));
            }
        }
    }

    let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if dist[nr][nc] > dist[r][c] + 1 {
                    dist[nr][nc] = dist[r][c] + 1;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
    dist
}

// ── Medium 9: Word Search ──────────────────────────────────────────────

struct MatrixGridWordSearch;

struct WordSearchTest {
    board: Vec<Vec<char>>,
    word: String,
}

impl Problem for MatrixGridWordSearch {
    fn id(&self) -> &str { "matrix_grid_word_search" }
    fn name(&self) -> &str { "Word Search" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "Given an m x n grid of characters `board` and a string `word`, return true if \
         `word` exists in the grid. The word can be constructed from letters of sequentially \
         adjacent cells (horizontally or vertically). The same cell may not be used more \
         than once.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 6\n\
         - 1 <= word.length <= 15\n\
         - board and word consist of only lowercase/uppercase English letters"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=5);
            let cols = rng.random_range(2..=5);
            let board: Vec<Vec<char>> = (0..rows)
                .map(|_| {
                    (0..cols)
                        .map(|_| (b'a' + rng.random_range(0..=5u8)) as char)
                        .collect()
                })
                .collect();
            // Sometimes create a word that exists in the board
            let word_len = rng.random_range(2..=4);
            let word = if rng.random_range(0..2) == 0 {
                // Try to pick a path through the board
                let mut w = String::new();
                let mut r = rng.random_range(0..rows);
                let mut c = rng.random_range(0..cols);
                w.push(board[r][c]);
                for _ in 1..word_len {
                    let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
                    let valid: Vec<(usize, usize)> = dirs.iter()
                        .filter_map(|(dr, dc)| {
                            let nr = r as i32 + dr;
                            let nc = c as i32 + dc;
                            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                                Some((nr as usize, nc as usize))
                            } else {
                                None
                            }
                        })
                        .collect();
                    if valid.is_empty() {
                        break;
                    }
                    let idx = rng.random_range(0..valid.len());
                    r = valid[idx].0;
                    c = valid[idx].1;
                    w.push(board[r][c]);
                }
                w
            } else {
                crate::problems::helpers::random_string_from(
                    &mut rng, word_len, b"abcdef",
                )
            };
            TestCase { data: Box::new(WordSearchTest { board, word }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<WordSearchTest>().unwrap();
        let expected = ref_word_search(&t.board, &t.word);
        let actual = solutions::word_search(&t.board, &t.word);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("board={:?}, word={:?}", t.board, t.word),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_word_search(board: &[Vec<char>], word: &str) -> bool {
    let rows = board.len();
    let cols = board[0].len();
    let chars: Vec<char> = word.chars().collect();
    let mut visited = vec![vec![false; cols]; rows];

    fn dfs(
        board: &[Vec<char>], chars: &[char], visited: &mut Vec<Vec<bool>>,
        r: usize, c: usize, idx: usize,
    ) -> bool {
        if idx == chars.len() {
            return true;
        }
        if r >= board.len() || c >= board[0].len() || visited[r][c] || board[r][c] != chars[idx] {
            return false;
        }
        visited[r][c] = true;
        let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nc >= 0 {
                if dfs(board, chars, visited, nr as usize, nc as usize, idx + 1) {
                    visited[r][c] = false;
                    return true;
                }
            }
        }
        visited[r][c] = false;
        false
    }

    for r in 0..rows {
        for c in 0..cols {
            if dfs(board, &chars, &mut visited, r, c, 0) {
                return true;
            }
        }
    }
    false
}

// ── Medium 10: Unique Paths ────────────────────────────────────────────

struct MatrixGridUniquePaths;

struct UniquePathsTest {
    m: usize,
    n: usize,
}

impl Problem for MatrixGridUniquePaths {
    fn id(&self) -> &str { "matrix_grid_unique_paths" }
    fn name(&self) -> &str { "Unique Paths" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Medium }
    fn description(&self) -> &str {
        "A robot is located at the top-left corner of an m x n grid. It can only move \
         either down or right at any point in time. The robot is trying to reach the \
         bottom-right corner. How many possible unique paths are there?\n\n\
         Constraints:\n\
         - 1 <= m, n <= 100"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let m = rng.random_range(1..=15);
            let n = rng.random_range(1..=15);
            TestCase { data: Box::new(UniquePathsTest { m, n }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<UniquePathsTest>().unwrap();
        let expected = ref_unique_paths(t.m, t.n);
        let actual = solutions::unique_paths(t.m, t.n);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("m={}, n={}", t.m, t.n),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_unique_paths(m: usize, n: usize) -> i32 {
    let mut dp = vec![vec![1i64; n]; m];
    for r in 1..m {
        for c in 1..n {
            dp[r][c] = dp[r - 1][c] + dp[r][c - 1];
        }
    }
    dp[m - 1][n - 1] as i32
}

// ── Hard 11: Shortest Path Eliminating Obstacles ────────────────────────

struct MatrixGridShortestPathObstacles;

struct ShortestPathObstaclesTest {
    grid: Vec<Vec<i32>>,
    k: i32,
}

impl Problem for MatrixGridShortestPathObstacles {
    fn id(&self) -> &str { "matrix_grid_shortest_path_obstacles" }
    fn name(&self) -> &str { "Shortest Path Eliminating Obstacles" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an m x n grid where 0 is an empty cell and 1 is an obstacle, return the \
         minimum number of steps to walk from the upper-left corner (0,0) to the lower-right \
         corner (m-1, n-1). You can eliminate at most `k` obstacles. Return -1 if it is not \
         possible.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 40\n\
         - 0 <= grid[i][j] <= 1\n\
         - 0 <= k <= m * n"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=8);
            let cols = rng.random_range(2..=8);
            let grid: Vec<Vec<i32>> = (0..rows)
                .map(|_| (0..cols).map(|_| if rng.random_range(0..=3) == 0 { 1 } else { 0 }).collect())
                .collect();
            let k = rng.random_range(0..=(rows * cols / 3) as i32);
            TestCase { data: Box::new(ShortestPathObstaclesTest { grid, k }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<ShortestPathObstaclesTest>().unwrap();
        let expected = ref_shortest_path_obstacles(&t.grid, t.k);
        let actual = solutions::shortest_path_eliminating_obstacles(&t.grid, t.k);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}, k={}", t.grid, t.k),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_shortest_path_obstacles(grid: &[Vec<i32>], k: i32) -> i32 {
    use std::collections::VecDeque;
    let rows = grid.len();
    let cols = grid[0].len();
    if rows == 1 && cols == 1 {
        return 0;
    }
    let k = k as usize;
    // visited[r][c][obstacles_eliminated]
    let mut visited = vec![vec![vec![false; k + 1]; cols]; rows];
    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize, 0usize, 0i32)); // r, c, eliminated, steps
    visited[0][0][0] = true;

    let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
    while let Some((r, c, elim, steps)) = queue.pop_front() {
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;
            let new_elim = elim + grid[nr][nc] as usize;
            if new_elim > k || visited[nr][nc][new_elim] {
                continue;
            }
            if nr == rows - 1 && nc == cols - 1 {
                return steps + 1;
            }
            visited[nr][nc][new_elim] = true;
            queue.push_back((nr, nc, new_elim, steps + 1));
        }
    }
    -1
}

// ── Hard 12: Swim in Rising Water ──────────────────────────────────────

struct MatrixGridSwimInWater;

struct SwimInWaterTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for MatrixGridSwimInWater {
    fn id(&self) -> &str { "matrix_grid_swim_in_water" }
    fn name(&self) -> &str { "Swim in Rising Water" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an n x n integer grid where grid[i][j] represents elevation, at time t the \
         depth of water everywhere is t. You can swim from one cell to another 4-directionally \
         if the elevation of both cells is at most t.\n\n\
         Return the minimum time t at which you can swim from the top-left (0,0) to the \
         bottom-right (n-1, n-1).\n\n\
         The grid contains all integers from 0 to n*n - 1 (a permutation).\n\n\
         Constraints:\n\
         - 2 <= n <= 50"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=7);
            let mut vals: Vec<i32> = (0..n * n).map(|i| i as i32).collect();
            // Shuffle
            for i in (1..vals.len()).rev() {
                let j = rng.random_range(0..=i);
                vals.swap(i, j);
            }
            let grid: Vec<Vec<i32>> = vals.chunks(n).map(|c| c.to_vec()).collect();
            TestCase { data: Box::new(SwimInWaterTest { grid }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<SwimInWaterTest>().unwrap();
        let expected = ref_swim_in_water(&t.grid);
        let actual = solutions::swim_in_water(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_swim_in_water(grid: &[Vec<i32>]) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let n = grid.len();
    let mut dist = vec![vec![i32::MAX; n]; n];
    let mut heap = BinaryHeap::new();
    dist[0][0] = grid[0][0];
    heap.push(Reverse((grid[0][0], 0usize, 0usize)));

    let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
    while let Some(Reverse((cost, r, c))) = heap.pop() {
        if r == n - 1 && c == n - 1 {
            return cost;
        }
        if cost > dist[r][c] {
            continue;
        }
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                let new_cost = cost.max(grid[nr][nc]);
                if new_cost < dist[nr][nc] {
                    dist[nr][nc] = new_cost;
                    heap.push(Reverse((new_cost, nr, nc)));
                }
            }
        }
    }
    -1
}

// ── Hard 13: Making A Large Island ─────────────────────────────────────

struct MatrixGridMakingLargeIsland;

struct MakingLargeIslandTest {
    grid: Vec<Vec<i32>>,
}

impl Problem for MatrixGridMakingLargeIsland {
    fn id(&self) -> &str { "matrix_grid_making_large_island" }
    fn name(&self) -> &str { "Making A Large Island" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an n x n binary grid, return the size of the largest island after changing \
         at most one 0 to 1. An island is a 4-directionally connected group of 1s.\n\n\
         Constraints:\n\
         - 1 <= n <= 500\n\
         - grid[i][j] is 0 or 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let n = rng.random_range(2..=8);
            let grid: Vec<Vec<i32>> = (0..n)
                .map(|_| (0..n).map(|_| rng.random_range(0..=1)).collect())
                .collect();
            TestCase { data: Box::new(MakingLargeIslandTest { grid }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<MakingLargeIslandTest>().unwrap();
        let expected = ref_making_large_island(&t.grid);
        let actual = solutions::making_large_island(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_making_large_island(grid: &[Vec<i32>]) -> i32 {
    use std::collections::HashSet;
    let n = grid.len();
    let mut island_id = vec![vec![0usize; n]; n];
    let mut island_sizes: Vec<i32> = vec![0]; // index 0 unused
    let mut id = 0usize;

    fn dfs(
        grid: &[Vec<i32>], island_id: &mut Vec<Vec<usize>>,
        r: usize, c: usize, id: usize,
    ) -> i32 {
        if island_id[r][c] != 0 || grid[r][c] == 0 {
            return 0;
        }
        island_id[r][c] = id;
        let mut size = 1;
        let n = grid.len();
        if r > 0 { size += dfs(grid, island_id, r - 1, c, id); }
        if r + 1 < n { size += dfs(grid, island_id, r + 1, c, id); }
        if c > 0 { size += dfs(grid, island_id, r, c - 1, id); }
        if c + 1 < n { size += dfs(grid, island_id, r, c + 1, id); }
        size
    }

    // Label each island
    for r in 0..n {
        for c in 0..n {
            if grid[r][c] == 1 && island_id[r][c] == 0 {
                id += 1;
                let size = dfs(grid, &mut island_id, r, c, id);
                island_sizes.push(size);
            }
        }
    }

    let mut max_size = *island_sizes.iter().skip(1).max().unwrap_or(&0);

    let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
    for r in 0..n {
        for c in 0..n {
            if grid[r][c] == 0 {
                let mut seen = HashSet::new();
                let mut total = 1; // the flipped cell
                for (dr, dc) in &dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        let iid = island_id[nr][nc];
                        if iid > 0 && seen.insert(iid) {
                            total += island_sizes[iid];
                        }
                    }
                }
                max_size = max_size.max(total);
            }
        }
    }
    max_size
}

// ── Hard 14: Longest Increasing Path in a Matrix ────────────────────────

struct MatrixGridLongestIncreasingPath;

struct LongestIncreasingPathTest {
    matrix: Vec<Vec<i32>>,
}

impl Problem for MatrixGridLongestIncreasingPath {
    fn id(&self) -> &str { "matrix_grid_longest_increasing_path" }
    fn name(&self) -> &str { "Longest Increasing Path in a Matrix" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "Given an m x n integers matrix, return the length of the longest increasing path \
         in the matrix. From each cell, you can move in 4 directions. You may not move \
         diagonally or outside the boundary. A path is increasing if each subsequent cell \
         has a strictly greater value.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 200\n\
         - 0 <= matrix[i][j] <= 2^31 - 1"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(2..=8);
            let cols = rng.random_range(2..=8);
            let matrix: Vec<Vec<i32>> = (0..rows)
                .map(|_| (0..cols).map(|_| rng.random_range(0..=20)).collect())
                .collect();
            TestCase { data: Box::new(LongestIncreasingPathTest { matrix }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<LongestIncreasingPathTest>().unwrap();
        let expected = ref_longest_increasing_path(&t.matrix);
        let actual = solutions::longest_increasing_path(&t.matrix);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("matrix={:?}", t.matrix),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_longest_increasing_path(matrix: &[Vec<i32>]) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut memo = vec![vec![0i32; cols]; rows];

    fn dfs(matrix: &[Vec<i32>], memo: &mut Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
        if memo[r][c] != 0 {
            return memo[r][c];
        }
        let mut best = 1;
        let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < matrix.len() as i32 && nc >= 0 && nc < matrix[0].len() as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if matrix[nr][nc] > matrix[r][c] {
                    best = best.max(1 + dfs(matrix, memo, nr, nc));
                }
            }
        }
        memo[r][c] = best;
        best
    }

    let mut ans = 0;
    for r in 0..rows {
        for c in 0..cols {
            ans = ans.max(dfs(matrix, &mut memo, r, c));
        }
    }
    ans
}

// ── Hard 15: Treasure Island ───────────────────────────────────────────

struct MatrixGridTreasureIsland;

struct TreasureIslandTest {
    grid: Vec<Vec<char>>,
}

impl Problem for MatrixGridTreasureIsland {
    fn id(&self) -> &str { "matrix_grid_treasure_island" }
    fn name(&self) -> &str { "Treasure Island" }
    fn topic(&self) -> &str { "matrix_grid" }
    fn difficulty(&self) -> Difficulty { Difficulty::Hard }
    fn description(&self) -> &str {
        "You are given a grid of characters where:\n\
         - 'S' is the starting cell (always at (0,0))\n\
         - 'T' is the treasure\n\
         - 'D' is a dangerous cell (cannot pass through)\n\
         - 'O' is an open cell\n\n\
         Return the minimum number of steps to reach the treasure from the start. \
         Return -1 if the treasure cannot be reached. Movement is 4-directional.\n\n\
         Constraints:\n\
         - 1 <= m, n <= 100\n\
         - Exactly one 'S' (at (0,0)) and at least one 'T'"
    }

    fn generate_tests(&self) -> Vec<TestCase> {
        let mut rng = rand::rng();
        (0..10).map(|_| {
            let rows = rng.random_range(3..=8);
            let cols = rng.random_range(3..=8);
            let mut grid: Vec<Vec<char>> = (0..rows)
                .map(|_| {
                    (0..cols)
                        .map(|_| {
                            let r = rng.random_range(0..=4);
                            if r == 0 { 'D' } else { 'O' }
                        })
                        .collect()
                })
                .collect();
            grid[0][0] = 'S';
            // Place treasure somewhere not at start
            let tr = rng.random_range(0..rows);
            let tc = rng.random_range(0..cols);
            if tr != 0 || tc != 0 {
                grid[tr][tc] = 'T';
            } else {
                grid[rows - 1][cols - 1] = 'T';
            }
            TestCase { data: Box::new(TreasureIslandTest { grid }) }
        }).collect()
    }

    fn run_solution(&self, test: &TestCase, _log: &mut OperationLog) -> SolutionResult {
        let t = test.data.downcast_ref::<TreasureIslandTest>().unwrap();
        let expected = ref_treasure_island(&t.grid);
        let actual = solutions::treasure_island(&t.grid);
        SolutionResult {
            is_correct: expected == actual,
            input_description: format!("grid={:?}", t.grid),
            expected: format!("{expected}"),
            actual: format!("{actual}"),
        }
    }
}

fn ref_treasure_island(grid: &[Vec<char>]) -> i32 {
    use std::collections::VecDeque;
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0usize, 0i32));
    visited[0][0] = true;

    let dirs: [(i32, i32); 4] = [(-1,0),(1,0),(0,-1),(0,1)];
    while let Some((r, c, steps)) = queue.pop_front() {
        if grid[r][c] == 'T' {
            return steps;
        }
        for (dr, dc) in &dirs {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let nr = nr as usize;
                let nc = nc as usize;
                if !visited[nr][nc] && grid[nr][nc] != 'D' {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc, steps + 1));
                }
            }
        }
    }
    -1
}
