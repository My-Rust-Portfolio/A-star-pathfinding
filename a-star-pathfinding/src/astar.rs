use crate::grid_data::GridData;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    f_score: usize,
    position: (usize, usize),
}

// BinaryHeap is a Max-Heap. To make it a Min-Heap for A*,
// reverse the comparison order (other compared to self)
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            // if scores are the same then order by position
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_score(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

pub fn find_path(grid: &GridData) -> Option<Vec<(usize, usize)>> {
    let start = grid.get_start_point()?;
    let end = grid.get_end_point()?;

    let width = grid.get_width();
    let height = grid.get_height();
    let size = width * height;

    let mut g_scores = vec![usize::MAX; size];
    let mut came_from: Vec<Option<(usize, usize)>> = vec![None; size];
    let mut open_set = BinaryHeap::new();
    let start_idx = start.1 * width + start.0;

    g_scores[start_idx] = 0;

    open_set.push(State {
        f_score: compute_score(start, end),
        position: start,
    });

    while let Some(State {
        f_score: _,
        position: current,
    }) = open_set.pop()
    {
        // target reached
        if current == end {
            let mut path = Vec::new();
            let mut curr = current;

            while let Some(prev) = came_from[curr.1 * width + curr.0] {
                path.push(curr);
                curr = prev;
            }
            path.push(start);
            path.reverse(); // reverse so it goes from start to finish
            return Some(path);
        }

        let current_idx = current.1 * width + current.0;
        let current_g = g_scores[current_idx];

        let neighbors = [
            (current.0.wrapping_sub(1), current.1), // left
            (current.0 + 1, current.1),             // right
            (current.0, current.1.wrapping_sub(1)), // up
            (current.0, current.1 + 1),             // down
        ];

        for next in neighbors {
            if next.0 >= width || next.1 >= height {
                continue;
            }

            let terrain = grid.get_terrain_type(next.0, next.1);
            let move_cost = terrain.cost();

            // impassable
            if move_cost == usize::MAX {
                continue;
            }

            // saturating_add prevents crash if current_g is somehow MAX
            let tentative_g = current_g.saturating_add(move_cost);
            let next_idx = next.1 * width + next.0;

            if tentative_g < g_scores[next_idx] {
                came_from[next_idx] = Some(current);
                g_scores[next_idx] = tentative_g;

                let f_score = tentative_g + compute_score(next, end);
                open_set.push(State {
                    f_score,
                    position: next,
                });
            }
        }
    }

    // path not found
    None
}
