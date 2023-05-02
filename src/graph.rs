// 計算量は(E+V)logV
pub struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        const INF: usize = 1 << 60;
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((std::cmp::Reverse(0), i));
            }
            heap.push((std::cmp::Reverse(INF), i));
        }
        while let Some((std::cmp::Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            for &(next, cost) in &edges[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((std::cmp::Reverse(distance[next]), next));
                    parent[next] = target;
                }
            }
        }
        Self { distance, parent }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }

    pub fn get_path(&self, target: usize) -> Vec<usize> {
        const INF: usize = 1 << 60;
        let mut current = target;
        let mut res = vec![current];
        while self.parent[current] != INF as usize {
            let next = self.parent[current];
            res.push(next);
            current = next;
        }
        res.reverse();
        res
    }
}

// 距離が1のときにはdfsでじゅうぶん。(木なら使える)
fn dfs(prev: usize, cur: usize, edges: &Vec<Vec<usize>>, d: &mut Vec<usize>) {
    for &next in &edges[cur] {
        if next == prev {
            continue;
        }
        d[next] = d[cur] + 1;
        dfs(cur, next, edges, d);
    }
}

// コストつき
fn dfs_cost(prev: usize, cur: usize, edges: &Vec<Vec<(usize, usize)>>, d: &mut Vec<usize>) {
    for &(next, cost) in &edges[cur] {
        if next == prev {
            continue;
        }
        d[next] = d[cur] + cost;
        dfs_cost(cur, next, edges, d);
    }
}

// スタックオーバーフロー対策、ループがあるやつ対策
fn bfs(init: usize, n: usize, edges: &Vec<Vec<usize>>) -> Vec<i64> {
    let mut d = vec![-1; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back((init, 0));
    while let Some((cur, dist)) = q.pop_front() {
        if d[cur] != -1 {
            continue;
        }
        d[cur] = dist;
        for &next in &edges[cur] {
            if d[next] == -1 {
                q.push_back((next, dist + 1));
            }
        }
    }
    d
}

fn topo(n: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut ind = vec![0; n];
    for i in 0..n {
        for j in 0..g[i].len() {
            ind[g[i][j]] += 1;
        }
    }
    let mut q = std::collections::VecDeque::new();
    for i in 0..n {
        if ind[i] == 0 {
            q.push_back(i);
        }
    }
    let mut topo = vec![];
    while let Some(cur) = q.pop_front() {
        topo.push(cur);
        for &next in g[cur].iter() {
            ind[next] -= 1;
            if ind[next] == 0 {
                q.push_back(next);
            }
        }
    }
    topo
}

// 計算量はE×V
pub struct BellmanFord {
    distance: Vec<i64>,
    has_neg_loop: bool,
}

impl BellmanFord {
    // n:usize 頂点の数
    // edges: Vec<(usize,usize,i64)> edges[i] = [(0,2,3), (1,3,-1), (From,To,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: Vec<(usize, usize, i64)>, init: usize) -> Self {
        let mut distance = vec![1 << 60; n];
        distance[init] = 0;
        let mut has_neg_loop = false;

        for i in 0..n {
            for edge in &edges {
                let from = edge.0;
                let to = edge.1;
                let cost = edge.2;
                if distance[to] > distance[from] + cost {
                    distance[to] = distance[from] + cost;
                    if i == n - 1 {
                        has_neg_loop = true;
                        break;
                    }
                }
            }
        }
        Self {
            distance,
            has_neg_loop,
        }
    }

    pub fn distance(&self, target: usize) -> i64 {
        self.distance[target]
    }
}

// 計算量はN^3
// 負の場合でも使用でき、任意の点の最短距離がすべて求まる
pub struct WarshallFloyd {
    distance: Vec<Vec<i64>>,
}

impl WarshallFloyd {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,i64)>> edges[i] = [(2,3), (3,-1), (To,重み)]
    pub fn new(n: usize, edges: &Vec<Vec<(usize, i64)>>) -> Self {
        let mut distance = vec![vec![1 << 60; n]; n];
        for i in 0..n {
            for &(j, c) in &edges[i] {
                distance[i][j] = c;
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    // 普通に足す場合は足してこの場合はmaxをとる問題だった
                    distance[i][j] = distance[i][j].min(distance[i][k].max(distance[k][j]));
                }
            }
        }
        Self { distance }
    }
}

// SCC(強連結成分分解)
// n もとの頂点数
// sizes 強連結成分をまとめたときのサイズ
// new_num もとの頂点->まとめたあとの頂点のマッピング
// new_edges まとめたあとの辺(トポロジカルソート済)
pub struct SCC {
    n: usize,
    g: Vec<Vec<usize>>,
    rev_g: Vec<Vec<usize>>,
    sizes: Vec<usize>,
    new_num: Vec<usize>,
    new_edges: Vec<Vec<usize>>,
}

impl SCC {
    pub fn new(n: usize) -> Self {
        let g = vec![vec![]; n];
        let rev_g = vec![vec![]; n];
        let sizes = vec![];
        let new_num = vec![0; n];
        let new_edges = vec![];
        Self {
            n,
            g,
            rev_g,
            sizes,
            new_num,
            new_edges,
        }
    }

    pub fn add_edges(&mut self, u: usize, v: usize) {
        self.g[u].push(v);
        self.rev_g[v].push(u);
    }

    // edges/rev_edges もとの辺
    pub fn execute(&mut self) {
        let n = self.n;
        let mut used = vec![false; n];
        // num[i] -> i番目の番号がどの頂点か(一度目のdfsの結果を記録)
        let mut num = vec![0; n];
        let mut count = 0;

        fn dfs(
            cur: usize,
            mut count: usize,
            used: &mut Vec<bool>,
            num: &mut Vec<usize>,
            edges: &Vec<Vec<usize>>,
        ) -> usize {
            used[cur] = true;
            for &next in edges[cur].iter() {
                if !used[next] {
                    count = dfs(next, count, used, num, edges);
                }
            }
            num[count] = cur;
            count + 1
        }

        for i in 0..n {
            if !used[i] {
                count = dfs(i, count, &mut used, &mut num, &self.g);
            }
        }
        // 初期化して二度目のdfsで使い回し
        used = vec![false; n];
        let mut count = 0;

        fn rev_dfs(
            cur: usize,
            count: usize,
            mut size: usize,
            new_num: &mut Vec<usize>,
            used: &mut Vec<bool>,
            rev_edges: &Vec<Vec<usize>>,
        ) -> usize {
            used[cur] = true;
            for &next in rev_edges[cur].iter() {
                if !used[next] {
                    size = rev_dfs(next, count, size, new_num, used, rev_edges);
                }
            }
            new_num[cur] = count;
            size + 1
        }

        for i in (0..n).rev() {
            let target = num[i];
            if !used[target] {
                let size = rev_dfs(target, count, 0, &mut self.new_num, &mut used, &self.rev_g);
                self.sizes.push(size);
                count += 1;
            }
        }
        let mut new_edges = vec![std::collections::BTreeSet::new(); self.sizes.len()];
        for i in 0..n {
            for &edge in &self.g[i] {
                if self.new_num[i] != self.new_num[edge] {
                    new_edges[self.new_num[i]].insert(self.new_num[edge]);
                }
            }
        }
        self.new_edges = new_edges
            .iter()
            .map(|s| s.iter().map(|i| *i).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        //return (sizes, new_num, v);
    }
}

// // SCC(強連結成分分解)
// // n もとの頂点数
// // sizes 強連結成分をまとめたときのサイズ
// // new_num もとの頂点->まとめたあとの頂点のマッピング(トポロジカルソート済)
// // new_edges まとめたあとの辺
// pub struct SCC {
//     n: usize,
//     sizes: Vec<usize>,
//     new_num: Vec<usize>,
//     new_edges: Vec<Vec<usize>>,
// }

// impl SCC {
//     pub fn new(n: usize) -> Self {
//         let sizes = vec![];
//         let new_num = vec![0; n];
//         let new_edges = vec![];
//         Self {
//             n,
//             sizes,
//             new_num,
//             new_edges,
//         }
//     }

//     // edges/rev_edges もとの辺
//     pub fn execute(&mut self, edges: Vec<Vec<usize>>, rev_edges: Vec<Vec<usize>>) {
//         let n = self.n;
//         let mut used = vec![false; n];
//         // num[i] -> i番目の番号がどの頂点か(一度目のdfsの結果を記録)
//         let mut num = vec![0; n];
//         let mut count = 0;
//         for i in 0..n {
//             if !used[i] {
//                 count = self.dfs(i, count, &mut used, &mut num, &edges);
//             }
//         }
//         // 初期化して二度目のdfsで使い回し
//         used = vec![false; n];
//         let mut count = 0;
//         for i in (0..n).rev() {
//             let target = num[i];
//             if !used[target] {
//                 let size = self.rev_dfs(target, count, 0, &mut used, &rev_edges);
//                 self.sizes.push(size);
//                 count += 1;
//             }
//         }
//         let mut new_edges = vec![std::collections::BTreeSet::new(); self.sizes.len()];
//         for i in 0..n {
//             for &edge in &edges[i] {
//                 if self.new_num[i] != self.new_num[edge] {
//                     new_edges[self.new_num[i]].insert(self.new_num[edge]);
//                 }
//             }
//         }
//         self.new_edges = new_edges
//             .iter()
//             .map(|s| s.iter().map(|i| *i).collect::<Vec<_>>())
//             .collect::<Vec<_>>();
//         //return (sizes, new_num, v);
//     }

//     fn dfs(
//         &mut self,
//         cur: usize,
//         mut count: usize,
//         used: &mut Vec<bool>,
//         num: &mut Vec<usize>,
//         edges: &Vec<Vec<usize>>,
//     ) -> usize {
//         used[cur] = true;
//         for &next in edges[cur].iter() {
//             if !used[next] {
//                 count = self.dfs(next, count, used, num, edges);
//             }
//         }
//         num[count] = cur;
//         count + 1
//     }

//     fn rev_dfs(
//         &mut self,
//         cur: usize,
//         count: usize,
//         mut size: usize,
//         used: &mut Vec<bool>,
//         rev_edges: &Vec<Vec<usize>>,
//     ) -> usize {
//         used[cur] = true;
//         for &next in rev_edges[cur].iter() {
//             if !used[next] {
//                 size = self.rev_dfs(next, count, size, used, rev_edges);
//             }
//         }
//         self.new_num[cur] = count;
//         size + 1
//     }
// }

// 最小全域木をつくるアルゴリズム
// 計算量|E|log|V|
// n:usize 頂点の数
// edges: Vec<(usize,usize,i64)> edges[i] = [(0,2,3), (1,3,-1), (From,To,重み)]
fn kruskal(n: usize, mut edges: Vec<(usize, usize, i64)>) -> i64 {
    edges.sort_by_key(|e| e.2);
    let mut uf = petgraph::unionfind::UnionFind::new(n);
    let mut res = 0;
    for (u, v, cost) in edges {
        if uf.union(u, v) {
            res += cost;
        }
    }
    res
}

// 最小全域木をつくるアルゴリズム(0からの最小全域)
// 計算量|E|log|V|
// n:usize 頂点の数
// edges: Vec<Vec<(usize,i64)>> edges[i] = [(2,3), (3,1), (頂点への道,重み)]
fn prim(n: usize, edges: Vec<Vec<(usize, i64)>>) -> i64 {
    let mut res = 0;
    let mut used = vec![false; n];
    let mut heap = std::collections::BinaryHeap::new();
    heap.push((std::cmp::Reverse(0), 0));
    while let Some((std::cmp::Reverse(d), target)) = heap.pop() {
        if used[target] {
            continue;
        }
        used[target] = true;
        res += d;
        for &(next, cost) in &edges[target] {
            heap.push((std::cmp::Reverse(cost), next));
        }
    }
    res
}

// 最小全域木をつくるアルゴリズム(できるだけバージョン、森でもOK)
// 計算量|E|log|V|
// n:usize 頂点の数
// edges: Vec<Vec<(usize,i64)>> edges[i] = [(2,3), (3,1), (頂点への道,重み)]
fn prim_forest(n: usize, edges: Vec<Vec<(usize, i64)>>) -> i64 {
    let mut res = 0;
    let mut remain: std::collections::HashSet<usize> = (0..n).collect();
    let mut heap = std::collections::BinaryHeap::new();
    while !remain.is_empty() {
        heap.push((std::cmp::Reverse(0), *remain.iter().next().unwrap()));
        while let Some((std::cmp::Reverse(d), target)) = heap.pop() {
            if !remain.remove(&target) {
                continue;
            }
            res += d;
            for &(next, cost) in &edges[target] {
                heap.push((std::cmp::Reverse(cost), next));
            }
        }
    }
    res
}

// LCA (Lowest Common Ancestor)
// 実装スキル不足でnewしたあとにinitをしなきゃいけない構造になってしまった・・・
pub struct LCA {
    n: usize,
    k: usize,
    // とある頂点の1,2,4...個先の頂点を保持
    // parent[0][i]...iの頂点の1つ祖先
    parent: Vec<Vec<usize>>,
    dist: Vec<usize>,
}

impl LCA {
    pub fn new(n: usize) -> Self {
        const INF: usize = 1 << 60;
        let mut k = 1;
        while 1 << k < n {
            k += 1;
        }
        let parent = vec![vec![INF; n]; k];
        let dist = vec![INF; n];
        Self { n, k, parent, dist }
    }

    pub fn init(&mut self, edges: &Vec<Vec<usize>>, init: usize) {
        const INF: usize = 1 << 60;
        self.dfs(init, init, edges);
        for i in 0..self.k - 1 {
            for j in 0..self.n {
                if self.parent[i][j] == INF {
                    self.parent[i + 1][j] = INF;
                } else {
                    self.parent[i + 1][j] = self.parent[i][self.parent[i][j]];
                }
            }
        }
    }

    // uとvのLCAを求める
    pub fn query(&self, mut u: usize, mut v: usize) -> usize {
        // uを深い方として計算する
        if self.dist[u] < self.dist[v] {
            std::mem::swap(&mut u, &mut v);
        }
        // LCAまでの距離を同じにする
        for i in 0..self.k {
            if (self.dist[u] - self.dist[v]) >> i & 1 == 1 {
                u = self.parent[i][u];
            }
        }
        if u == v {
            return u;
        }
        // 一定以上の祖先は同じはずなので、上のbitから検証して揃えていく
        for i in (0..self.k).rev() {
            if self.parent[i][u] != self.parent[i][v] {
                u = self.parent[i][u];
                v = self.parent[i][v];
            }
        }
        return self.parent[0][u];
    }

    // 2点間の距離
    pub fn get_dist(&self, u: usize, v: usize) -> usize {
        self.dist[u] + self.dist[v] - 2 * self.dist[self.query(u, v)]
    }

    // 2点間のパス上に点pが存在するか判定
    pub fn is_on_path(&self, u: usize, v: usize, p: usize) -> bool {
        self.get_dist(u, p) + self.get_dist(p, v) == self.get_dist(u, v)
    }

    // 根からの距離を求める
    fn dfs(&mut self, prev: usize, cur: usize, edges: &Vec<Vec<usize>>) {
        for &next in &edges[cur] {
            if next == prev {
                continue;
            }
            self.dist[next] = self.dist[cur] + 1;
            self.parent[0][next] = cur;
            self.dfs(cur, next, edges);
        }
    }
}

// 木の直径を求める。
// 木の任意のノードからの距離の最大値はLかRへの距離となる
// 参考：https://atcoder.jp/contests/abc267/submissions/34584812
// 直径の片方ともう片方と直径
fn tree_diameter(edges: &Vec<Vec<usize>>) -> (usize, usize, usize) {
    let l = tree_diameter_dfs(edges, 0, !0);
    let r = tree_diameter_dfs(edges, l.1, !0);
    (l.1, r.1, r.0)
}

// (距離, to)の最大値を返却する
fn tree_diameter_dfs(edges: &Vec<Vec<usize>>, cur: usize, parent: usize) -> (usize, usize) {
    let mut ret = (0, cur);
    for &to in &edges[cur] {
        if to == parent {
            continue;
        }
        let mut next = tree_diameter_dfs(edges, to, cur);
        next.0 += 1;
        ret = ret.max(next);
    }
    ret
}

// コストつきの場合
fn tree_diameter_cost(edges: &Vec<Vec<(usize, usize)>>) -> (usize, usize, usize) {
    let l = tree_diameter_dfs_cost(edges, 0, !0);
    let r = tree_diameter_dfs_cost(edges, l.1, !0);
    (l.1, r.1, r.0)
}

fn tree_diameter_dfs_cost(
    edges: &Vec<Vec<(usize, usize)>>,
    cur: usize,
    parent: usize,
) -> (usize, usize) {
    let mut ret = (0, cur);
    for &(to, cost) in &edges[cur] {
        if to == parent {
            continue;
        }
        let mut next = tree_diameter_dfs_cost(edges, to, cur);
        next.0 += cost;
        ret = ret.max(next);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkistra() {
        let n = 6;
        let abc = vec![
            (0, 1, 5),
            (0, 3, 9),
            (0, 4, 2),
            (1, 2, 2),
            (2, 3, 3),
            (3, 5, 2),
            (4, 5, 3),
        ];
        let mut path = vec![vec![]; n];
        for (a, b, c) in abc {
            path[a].push((b, c));
            path[b].push((a, c));
        }
        let d = Dijkstra::new(n, &path, 0);

        assert_eq!(d.distance, vec![0, 5, 7, 7, 2, 5]);

        assert_eq!(d.get_path(0), vec![0]);
        assert_eq!(d.get_path(1), vec![0, 1]);
        assert_eq!(d.get_path(2), vec![0, 1, 2]);
        assert_eq!(d.get_path(3), vec![0, 4, 5, 3]);
        assert_eq!(d.get_path(4), vec![0, 4]);
        assert_eq!(d.get_path(5), vec![0, 4, 5]);
    }

    #[test]
    fn test_bellmanford() {
        let n = 6;
        let abc = vec![
            (0, 1, 5),
            (0, 3, 9),
            (0, 4, 2),
            (1, 2, 2),
            (2, 3, 3),
            (3, 5, 2),
            (4, 5, 3),
        ];
        let mut edges = vec![];
        for (a, b, c) in abc {
            edges.push((a, b, c));
            edges.push((b, a, c));
        }
        let b = BellmanFord::new(n, edges, 0);

        assert_eq!(b.distance, vec![0, 5, 7, 7, 2, 5]);
        assert_eq!(b.has_neg_loop, false);
    }

    #[test]
    fn test_scc() {
        let n = 12;
        let uv = vec![
            (0, 1),
            (1, 2),
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 2),
            (4, 5),
            (5, 6),
            (6, 7),
            (6, 8),
            (6, 9),
            (7, 5),
            (8, 10),
            (9, 8),
            (9, 11),
            (10, 8),
        ];
        let mut scc = SCC::new(n);
        for (u, v) in uv {
            scc.add_edges(u, v);
        }
        scc.execute();
        assert_eq!(scc.sizes, vec![1, 1, 3, 3, 1, 1, 2]);
        assert_eq!(scc.new_num, vec![0, 1, 2, 2, 2, 3, 3, 3, 6, 4, 6, 5]);
        assert_eq!(
            scc.new_edges,
            vec![
                vec![1],
                vec![2],
                vec![3],
                vec![4, 6],
                vec![5, 6],
                vec![],
                vec![]
            ]
        );
    }

    #[test]
    fn test_kruskal() {
        let n = 10;
        let abc = vec![
            (4, 3, 6831),
            (1, 3, 4583),
            (0, 0, 6592),
            (0, 1, 3063),
            (3, 3, 4975),
            (1, 3, 2049),
            (4, 2, 2104),
            (2, 2, 781),
        ];
        let mut edges = vec![];
        for (a, b, c) in abc {
            edges.push((a, 5 + b, -c));
            edges.push((5 + b, a, -c));
        }
        assert_eq!(10000 * n as i64 + kruskal(n, edges), 71071);
    }

    #[test]
    fn test_prim() {
        let n = 10;
        let abc = vec![
            (4, 3, 6831),
            (1, 3, 4583),
            (0, 0, 6592),
            (0, 1, 3063),
            (3, 3, 4975),
            (1, 3, 2049),
            (4, 2, 2104),
            (2, 2, 781),
        ];
        let mut edges = vec![vec![]; n];
        for (a, b, c) in abc {
            edges[a].push((5 + b, -c));
            edges[5 + b].push((a, -c));
        }
        assert_eq!(10000 * n as i64 + prim_forest(n, edges), 71071);
    }

    #[test]
    fn test_lca() {
        let n = 8;
        let uv = vec![(0, 1), (0, 2), (1, 7), (2, 3), (2, 6), (3, 4), (3, 5)];
        let mut edges = vec![vec![]; n];
        for (u, v) in uv {
            edges[u].push(v);
            edges[v].push(u);
        }
        let mut lca = LCA::new(n);
        lca.init(&edges, 0);
        assert_eq!(lca.query(1, 6), 0);
        assert_eq!(lca.query(4, 6), 2);
        assert_eq!(lca.query(4, 5), 3);

        assert_eq!(lca.get_dist(7, 2), 3);
        assert_eq!(lca.get_dist(1, 6), 3);
        assert_eq!(lca.get_dist(3, 4), 1);

        assert_eq!(lca.is_on_path(1, 3, 6), false);
        assert_eq!(lca.is_on_path(2, 4, 3), true);
        assert_eq!(lca.is_on_path(7, 5, 2), true);
    }
}
