use std::{
    time::Duration,
    thread::sleep,
    collections::VecDeque,
    io::stdin,
};

const FRAME_INTARVAL: u64 = 500; // milliseconds

fn read() -> std::io::Result<Vec<Vec<usize>>> {
    let mut buf = String::with_capacity(1<<8);
    stdin().read_line(&mut buf)?;
    let mut iter = buf.split_whitespace();
    let first_line: Vec<usize> = iter.map(|e| e.parse::<usize>().unwrap()).collect();
    let n = first_line.len();

    let mut costs: Vec<Vec<usize>> = vec![vec![0; n]; n];
    costs[0] = first_line;

    for i in 1..n {
        let mut buf = String::with_capacity(1<<8);
        stdin().read_line(&mut buf)?;
        iter = buf.split_whitespace();
        costs[i] = iter.map(|e| e.parse::<usize>().unwrap()).collect();
    }
    Ok(costs)
}

fn fmt_svec(svec: &Vec<Vec<String>>) -> String {
    let n = svec.len();
    let mut s = String::new();
    for i in 0..n {
        for j in 0..n {
            s.push_str(&svec[i][j]);
            s.push_str("\t");
        }
        s.push_str("\n");
    }
    s
}

fn main() {
    let mut costs = read().unwrap();
    let n = costs.len();

    let mut svec: Vec<Vec<String>> = costs.iter()
        .map(|c| c.iter().map(|c| format!("{}", c)).collect()).collect();
    let mut queue: VecDeque<String> = VecDeque::with_capacity(n.pow(3));
    queue.push_back( fmt_svec(&svec) );

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let m = costs[i][k] + costs[k][j];
                let ij = svec[i][j].clone();
                let ik = svec[i][k].clone();
                let kj = svec[k][j].clone();

                // make cost[i][k] & cost[k][j] blue
                svec[i][k] = format!("\x1b[34m{} \x1b[0m", svec[i][k]);
                svec[k][j] = format!("\x1b[34m{} \x1b[0m", svec[k][j]);

                if costs[i][j] > m {
                    costs[i][j] = m;
                    // make cost[i][j] red if the value is replaced
                    svec[i][j] = format!("\x1b[31m{} \x1b[0m", m);
                    queue.push_back( fmt_svec(&svec) );
                    svec[i][j] = format!("{}", m);
                } else {
                    // otherwise green
                    svec[i][j] = format!("\x1b[32m{} \x1b[0m", ij);
                    queue.push_back( fmt_svec(&svec) );
                    svec[i][j] = format!("{}", ij);
                }
                svec[i][k] = format!("{}", ik);
                svec[k][j] = format!("{}", kj);
            }
        }
    }

    print!("\x1b[0;0H");
    print!("{}", " ".repeat(n*n*100));
    let d = Duration::from_millis(FRAME_INTARVAL);
    for (i, s) in queue.iter().enumerate() {
        print!("\x1b[0;0H\x1b[0m");
        println!("step {}", i);
        print!("{}", s);
        sleep(d);
    }
}
