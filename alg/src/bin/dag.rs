// Here is a DAG with to path:
//     a->b->d->e->f
//     a->c->f
//
// Walk through this dag with path dependences.
// The result should be:
//     a,b,c,d,e,f
// Or
//     a,c,b,d,e,f
use std::collections::HashMap;

struct Dag {
    edges: HashMap<char, char>,
}

fn main() {
    let dag = Dag {
        edges: [
            ('a', 'b'),
            ('a', 'c'),
            ('b', 'd'),
            ('d', 'e'),
            ('e', 'f'),
            ('c', 'f'),
        ]
        .iter()
        .cloned()
        .collect(),
    };
    let ret: Vec<char> = walk(&dag);
    let s: String = ret
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", s);
}

fn walk(dag: &Dag) -> Vec<char> {
    let mut enter_dig: HashMap<char, i32> = build_enter_dig(dag);
    let total = enter_dig.len();

    let mut ret: Vec<char> = Vec::new();
    while total > ret.len() {
        let tmp = calc_out_node(&mut enter_dig);
        let mut res = tmp.clone();

        for n in tmp {
            enter_dig.remove(&n);
            if let Some(to) = dag.edges.get(&n) {
                if let Some(d) = enter_dig.get_mut(&to) {
                    *d -= 1;
                }
            }
        }
        ret.append(&mut res);
    }

    ret
}

fn calc_out_node(enter_dig: &mut HashMap<char, i32>) -> Vec<char> {
    let mut res: Vec<char> = Vec::new();
    for (n, d) in enter_dig {
        if *d == 0 {
            res.push(*n);
        }
    }
    res
}

fn build_enter_dig(dag: &Dag) -> HashMap<char, i32> {
    let mut enter_dig: HashMap<char, i32> = HashMap::new();
    for (from, to) in &dag.edges {
        if !enter_dig.contains_key(&from) {
            enter_dig.insert(*from, 0);
        }
        if !enter_dig.contains_key(&to) {
            enter_dig.insert(*to, 0);
        }
        if let Some(d) = enter_dig.get_mut(&to) {
            *d += 1;
        }
    }
    enter_dig
}
