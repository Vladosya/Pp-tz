use grlib::Graph;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_n = if args.len() > 1 {
        &args[1]
    } else {
        "gu/src/datas.tgf"
    };

    let s: String = std::fs::read_to_string(file_n).unwrap();

    let g: Graph<u32> = Graph::<u32>::des(&s);
    for node in g.get_all_nodes() {
        print!("node id: {}, neighbors: [", node.id);
        for nbr in g.get_connected(node) {
            print!(" {}", nbr.id);
        }
        println!(" ], val: {}", node.val);
    }
}
