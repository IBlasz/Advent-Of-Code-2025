fn main() { 
    println!("Day 1");
    //wczytuje input w czasie kompilacji
    let input = include_str!("../../inputs/day1.txt");    
    
    //pozycja na tarczy (zaczynam od 50)
    let mut pos: i32 = 50;
    let mut zero_count: i32 = 0;

    //iterator
    let mut lines = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty());

    //iteruje po niepustych liniach
    loop{
        let line = match lines.next(){
            Some(l) => l,
            None => break, //koniec pliku -> wyjście z pętli
        };

        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].trim().parse().unwrap();

        match dir {
            'L' => pos = (pos-dist).rem_euclid(100), 
            //%100 będzie źle bo -1 % 100 = -1
            //żeby to naprawić trzeba dać ((pos - dist) % 100 + 100) % 100
            //rem_euclid to matematyczne modulo
            'R' => pos = (pos+dist).rem_euclid(100),
            _ => panic!("Unknown direction"),
        }

        if pos == 0 {
            zero_count += 1;
        }
    }
    println!("Times dial pointed at 0: {}", zero_count);
}


