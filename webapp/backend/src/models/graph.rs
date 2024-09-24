use sqlx::FromRow;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    f_score: i32,
    g_score: i32,
    node: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
            .then_with(|| self.g_score.cmp(&other.g_score))
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        let mut g_scores: HashMap<i32, i32> = HashMap::new();
        let mut heap = BinaryHeap::new();

        // ヒューリスティック関数：マンハッタン距離
        let h = |node: i32| -> i32 {
            let from = self.nodes.get(&node).unwrap();
            let to = self.nodes.get(&to_node_id).unwrap();
            ((from.x - to.x).abs() + (from.y - to.y).abs()) as i32
        };

        // 開始ノードをヒープに追加
        heap.push(State {
            f_score: h(from_node_id),
            g_score: 0,
            node: from_node_id
        });
        g_scores.insert(from_node_id, 0);

        while let Some(State { f_score: _, g_score, node }) = heap.pop() {
            // 目的地に到達した場合、コストを返す
            if node == to_node_id {
                return g_score;
            }

            // より高コストの経路を見つけた場合はスキップ
            if g_score > *g_scores.get(&node).unwrap_or(&i32::MAX) {
                continue;
            }

            // 隣接ノードを探索
            if let Some(edges) = self.edges.get(&node) {
                for edge in edges {
                    let next_g_score = g_score + edge.weight;
                    let next = State {
                        f_score: next_g_score + h(edge.node_b_id),
                        g_score: next_g_score,
                        node: edge.node_b_id,
                    };

                    // より短い経路が見つかった場合、スコアを更新してヒープに追加
                    if next.g_score < *g_scores.get(&next.node).unwrap_or(&i32::MAX) {
                        g_scores.insert(next.node, next.g_score);
                        heap.push(next);
                    }
                }
            }
        }

        // 目的地に到達できない場合は i32::MAX を返す
        i32::MAX
    }
}