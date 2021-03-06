/**
 PP0501A - NWD
Napisz funkcję:

 int nwd(int a, int b);
która oblicza największy wspólny dzielnik liczb a i b,

 0 <= a,b <= 1000000
Input
 

W pierwszej linii liczba testów t, w kolejnych liniach 
po dwie liczby w każdym wierszu.

Output
W każdej linii jedna liczba - wynik działania funkcji nwd

Example
Input:
5
1 4
4 1
12 48
48 100
123456 653421

Output:
1
1
12
4
3
*/ 

fn nwd(a: u32, b:u32) -> u32 {
    if a == 0 {
        b
     } else {
        nwd(b % a, a)
     }
}

fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("err");

    let t:u16 = buffer.trim().parse().unwrap();
    for _ in 0..t {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("err");
        let v: Vec<_> = buffer.split(' ').map(|el| el.trim().parse::<u32>().unwrap()).collect();
        let (a, b) = (v[0], v[1]);
        println!("{}", nwd(a, b));

    }
}
