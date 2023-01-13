use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

// the graph structure, use min heap to store adjacent point.
type G = HashMap<String, BinaryHeap<Reverse<String>>>;
// the arrow line counts, to find the start point.
type ArrowCount = HashMap<String, (i32, i32)>; //(in count, out count)

// if no result, return vec![].
// Sometimes the answer is not unique, use use min lexicographical order.
pub fn find_total_path(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut g: G = HashMap::new();
    let mut c: ArrowCount = HashMap::new();
    let mut start_point = String::new();
    let mut end_point = String::new();

    for t in tickets {
        g.entry(t[0].clone())
            .or_default()
            .push(Reverse(t[1].clone()));

        c.entry(t[0].clone()).or_default().1 += 1;
        c.entry(t[1].clone()).or_default().0 += 1;
    }

    // Check the scenery
    for (p, count) in c {
        if count.1 - count.0 == 1 {
            // out - in == 1
            if !start_point.is_empty() {
                // more than one start point found, wrong data
                return res;
            }
            start_point = p.clone();
        } else if count.0 - count.1 == 1 {
            // in - out == 1
            if !end_point.is_empty() {
                // more than one end point found, wrong data
                return res;
            }
            end_point = p.clone();
        } else if count.0 != count.1 {
            // in != out, wrong data
            return res;
        }
    }
    if start_point.is_empty() {
        // loop graph, every point can be start point, we choose the min lexicographical one.
        start_point = g.iter().min_by_key(|entry| entry.0).unwrap().0.clone();
    }

    let mut stack: Vec<String> = vec![start_point];
    while !stack.is_empty() {
        // dfs to search each line, prefer to choose the min lexicographical one which is on the heap's top.
        while g.contains_key(stack.last().unwrap())
            && !g.get(stack.last().unwrap()).unwrap().is_empty()
        {
            let heaps = g.get_mut(stack.last().unwrap()).unwrap();
            let point = heaps.pop().unwrap().0;
            stack.push(point);
        }
        res.push(stack.pop().unwrap())
    }

    // backtrace, so reverse the order
    vec![
        res.last().unwrap().to_string(),
        res.first().unwrap().to_string(),
    ]
}

#[test]
fn test() {
    use rustgym_util::{vec_string, vec_vec_string};
    let tickets: Vec<Vec<String>> = vec_vec_string![["A", "B"], ["B", "C"], ["C", "D"]];
    let res: Vec<String> = vec_string!["A", "D"];
    assert_eq!(find_total_path(tickets), res);

    let tickets: Vec<Vec<String>> = vec_vec_string![["A", "B"], ["B", "A"]];
    let res: Vec<String> = vec_string!["A", "A"];
    assert_eq!(find_total_path(tickets), res);

    let tickets: Vec<Vec<String>> = vec_vec_string![["A", "B"]];
    let res: Vec<String> = vec_string!["A", "B"];
    assert_eq!(find_total_path(tickets), res);

    let tickets: Vec<Vec<String>> =
        vec_vec_string![["A", "B"], ["B", "C"], ["C", "B"], ["B", "D"], ["D", "B"]];
    let res: Vec<String> = vec_string!["A", "B"];
    assert_eq!(find_total_path(tickets), res);

    let tickets: Vec<Vec<String>> = vec_vec_string![["A", "B"], ["C", "D"]];
    let res: Vec<String> = vec_string![];
    assert_eq!(find_total_path(tickets), res);
}
