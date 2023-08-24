use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut proby_wybor = String::new();
    let mut zakres_wybor = String::new();
    let zakres_down = 1;

    println!("Wybierz Zakres!");
    println!("1.Łatwy (1 - 100)");
    println!("2.Normalny (1 - 1,000)");
    println!("3.Trudny (1 - 10,000)");
    println!("4.Bardzo Trudny (1 - 100,000)");
    println!("5.Ekstremalnie Trudny (1 - 1,000,000)");
    println!("6.Własny Zakes");

    io::stdin()
            .read_line(&mut zakres_wybor)
            .expect("Failed to read line");
        let zakres_wybor: u32 = match zakres_wybor.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Błąd"),
        };

    match zakres_wybor {
        1 =>{
            let zakres_up = 100;
        }
        2 =>{
            let zakres_up = 1000;
        }
        3 =>{
            let zakres_up = 10000;
        }
        4 =>{
            let zakres_up = 100000;
        }
        5 =>{
            let zakres_up = 1000000;
        }
        6 =>{
            todo;
        }
        7 =>{
            panic!("Błąd"); 
        };
    };

    println!("Wybierz Ilość Prób!");
    println!("1.Łatwy (20)");
    println!("2. Normalny (10)");
    println!("3. Trudny (7)");
    println!("4. Bardzo Trudny (4)");
    println!("5. Ekstremalnie Trudny (1)");
    println!("6. Własna Ilość");

    io::stdin()
            .read_line(&mut proby_wybor)
            .expect("Failed to read line");
        let proby_wybor: u32 = match proby_wybor.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Błąd"),
        };
        
    match proby_wybor {
        1 =>{
            let rundy_set = 20;
        }
        2 =>{
            let rundy_set = 10;
        }
        3 =>{
            let rundy_set = 7;
        }
        4 =>{
            let rundy_set = 4;
        }
        5 =>{
            let rundy_set = 1;
        }
        6 =>{
            todo;
        }
        7 =>{
            panic!("Błąd");
        };
    };

    println!("Zgadnij liczbę");
    
    let sekret = rand::thread_rng().gen_range(zakres_down..=zakres_up);

    let mut runda = 0;

    loop {
        println!("Podaj swoją liczbę.");

        let mut odpowiedz = String::new();

        io::stdin()
            .read_line(&mut odpowiedz)
            .expect("Failed to read line");

        let odpowiedz: u32 = match odpowiedz.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Twoja liczba to: {odpowiedz}");

        match odpowiedz.cmp(&sekret) {
            Ordering::Less => {
                println!("Zgadywana liczba jest większa!");
                runda += 1;
            }
            Ordering::Greater => {
                println!("Zgadywana liczba jest mniejsza!");
                runda += 1;
            }
            Ordering::Equal => {
                println!("I jest to poprawana liczba gratulacje!");
                println!("Ukończyłeś grę w {runda} pytań.");
                break;
            }
        }
        if runda == rundy_set {
            println!("Niestety zmarnowałeś wszystkie swoje podejścia.");
            break;
        }
    }
}