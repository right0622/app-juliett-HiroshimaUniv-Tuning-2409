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
    cost: i32,
    node: i32,
}

// Rust の BinaryHeap は最大ヒープなので、Ord と PartialOrd を逆にして最小ヒープとして使用
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
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
        let mut dist: HashMap<i32, i32> = HashMap::new();
        let mut heap = BinaryHeap::new();

        // 開始ノードをヒープに追加
        heap.push(State { cost: 0, node: from_node_id });
        dist.insert(from_node_id, 0);

        while let Some(State { cost, node }) = heap.pop() {
            // 目的地に到達した場合、コストを返す
            if node == to_node_id {
                return cost;
            }

            // より高コストの経路を見つけた場合はスキップ
            if cost > *dist.get(&node).unwrap_or(&i32::MAX) {
                continue;
            }

            // 隣接ノードを探索
            if let Some(edges) = self.edges.get(&node) {
                for edge in edges {
                    let next = State {
                        cost: cost + edge.weight,
                        node: edge.node_b_id,
                    };

                    // より短い経路が見つかった場合、距離を更新してヒープに追加
                    if next.cost < *dist.get(&next.node).unwrap_or(&i32::MAX) {
                        dist.insert(next.node, next.cost);
                        heap.push(next);
                    }
                }
            }
        }

        // 目的地に到達できない場合は i32::MAX を返す
        i32::MAX
    }
}
