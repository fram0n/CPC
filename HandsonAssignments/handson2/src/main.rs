use::handson2::SegmentTreeMax;
use::handson2::SegmentTreeSeg;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::vec;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum EventType {
    Begin,
    End,
}
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
struct Event {
    num: u64,
    event_type: EventType,
}
fn main() {

    let folder_path = "Testset_handson2_2324_p1";
    let mut n: usize;
    let mut i: usize;
    let mut j: usize;
    let mut val: u64;
    for it in 0..11 {
        // Costruisci il percorso per il file di input
        let input_file = format!("{}/input{}.txt", folder_path, it);
        // Costruisci il percorso per il file di output corrispondente
        let output_file = format!("{}/output{}.txt", folder_path, it);
        let file_in = match fs::File::open(input_file) {
            Ok(file_in) => file_in,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                std::process::exit(1);
            }
        };
        let mut reader = io::BufReader::new(file_in);
        // Leggi la prima riga dal file
        if let Some(Ok(first_line)) = reader.by_ref().lines().next() {
            let numbers: Vec<&str> = first_line.split_whitespace().collect();   //la prima riga indica i valori di n e m
            n = numbers[0].parse::<usize>().unwrap();
            let mut t1 = SegmentTreeMax::new_tree(n);

            // Leggi la seconda riga dal file
            if let Some(Ok(second_line)) = reader.by_ref().lines().next() {
                let numbers: Vec<&str> = second_line.split_whitespace().collect();   //la seconda riga indica l'array a
                let mut a = vec![0; n];
                let mut iter = 0;
                for number in numbers {
                    a[iter] = number.parse::<u64>().unwrap();
                    iter += 1;
                }
                t1.build(&a, 1, 1, n);
                //println!("{:?}",t1.tree);

            } else {
                eprintln!("Errore nella lettura della seconda riga.");
                std::process::exit(1);
            }
            let mut res = vec![];
            // Leggi tutte le righe rimanenti
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    let numbers: Vec<&str> = line_content.split_whitespace().collect();   //le rimanenti righe indicano le query
                    if numbers[0].parse::<u64>().unwrap() == 0 {    //chiama update
                        i = numbers[1].parse::<usize>().unwrap();
                        j = numbers[2].parse::<usize>().unwrap();
                        val = numbers[3].parse::<u64>().unwrap();
                        t1.update(1, 1, n, i, j, val);
                    }
                    else if numbers[0].parse::<u64>().unwrap() == 1 {   //chiama max
                        i = numbers[1].parse::<usize>().unwrap();
                        j = numbers[2].parse::<usize>().unwrap();
                        res.push(t1.max(1, 1, n, i, j));
                    }
                } else {
                    eprintln!("Errore nella lettura di una riga rimanente.");
                    std::process::exit(1);
                }
            }
            //controlla se i risultati corrispondono nel file di output
            let file_out = match fs::File::open(output_file) {
                Ok(file_out) => file_out,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    std::process::exit(1);
                }
            };
            let reader = io::BufReader::new(file_out);
            let mut iter = 0;
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    if line_content.parse::<u64>().unwrap() != res[iter] {
                        println!("Test{}: wrong, result: {} {}", it, line_content.parse::<u64>().unwrap(), res[iter]);
                        return;
                    }
                }
                iter += 1;
            }
            println!("Test{}: ok", it);
        }
    }
    println!("");
    let folder_path = "Testset_handson2_2324_p2";
    let mut n: usize;
    let mut i: usize;
    let mut j: usize;
    let mut k: i64;
    let mut a: Vec<i64>;
    for it in 0..8 {
        // Costruisci il percorso per il file di input
        let input_file = format!("{}/input{}.txt", folder_path, it);
        // Costruisci il percorso per il file di output corrispondente
        let output_file = format!("{}/output{}.txt", folder_path, it);
        let file_in = match fs::File::open(input_file) {
            Ok(file_in) => file_in,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                std::process::exit(1);
            }
        };
        let mut reader = io::BufReader::new(file_in);
        // Leggi la prima riga dal file
        if let Some(Ok(first_line)) = reader.by_ref().lines().next() {
            let numbers: Vec<&str> = first_line.split_whitespace().collect();   //la prima riga indica i valori di n e m
            n = numbers[0].parse::<usize>().unwrap();
            let mut t1 = SegmentTreeSeg::new_tree(n);
            // Leggi dalla seconda riga n segmenti
            let mut segments: Vec<(u64,u64)> = Vec::new();     //crea il vettore contenente i segmenti
            for _i in 0..n {
                if let Some(Ok(line)) = reader.by_ref().lines().next() {
                    let numbers: Vec<&str> = line.split_whitespace().collect();
                    segments.push((numbers[0].parse::<u64>().unwrap(), numbers[1].parse::<u64>().unwrap()));
                } else {
                    eprintln!("Errore nella lettura della seconda riga.");
                    std::process::exit(1);
                }
            }
             let mut events: Vec<Event> = Vec::new();
            for &(begin, end) in &segments {
                events.push(Event { num: begin, event_type: EventType::Begin });
                events.push(Event { num: end, event_type: EventType::End });
            }
            events.sort_unstable();
            let max_x = events.iter().map(|event| event.num).max().unwrap();
            let mut count: Vec<Option<Vec<Event>>>;
            count = vec![None; (max_x + 1) as usize];
            for event in events {
                if count[event.num as usize] == None {
                    count[event.num as usize] = Some(Vec::new());
                }
                count[event.num as usize].as_mut().unwrap().push(event);
            }
            a = vec![0; (max_x + 1) as usize];
            for i in 0..max_x+1 {
                if i != 0 {
                    a[i as usize] += a[(i-1) as usize];
                }
                match count[i as usize].as_mut() {
                    Some(v) => {
                        for e in v {
                            match e.event_type {
                                EventType::Begin => {
                                    a[e.num as usize] += 1;
                                }
                                EventType::End => {
                                    if e.num != max_x {
                                        a[(e.num+1) as usize] -= 1;
                                    }
                                }
                            }
                        }
                    }
                    None => {
                    }
                }
            }
            println!("{:?}", a);
            t1.build(&a, 1, 0, max_x as usize);
            let mut res = vec![];
            // Leggi tutte le righe rimanenti
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    let numbers: Vec<&str> = line_content.split_whitespace().collect();   //le rimanenti righe indicano le query
                    //chiama is_there
                    i = numbers[0].parse::<usize>().unwrap();
                    j = numbers[1].parse::<usize>().unwrap();
                    k = numbers[2].parse::<i64>().unwrap();
                    res.push(t1.is_there(1, 0, n-1, i, j, k));
                    //println!("{:?}", t1.tree);
                } else {
                    eprintln!("Errore nella lettura di una riga rimanente.");
                    std::process::exit(1);
                }
            }
            //controlla se i risultati corrispondono nel file di output
            let file_out = match fs::File::open(output_file) {
                Ok(file_out) => file_out,
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                    std::process::exit(1);
                }
            };
            let reader = io::BufReader::new(file_out);
            let mut iter = 0;
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    if line_content.parse::<u64>().unwrap() != res[iter] {
                        println!("Test{}: wrong, result: {} {}", it, line_content.parse::<u64>().unwrap(), res[iter]);
                        return;
                    }
                }
                iter += 1;
            }
            println!("Test{}: ok", it);
        }
    }
}
