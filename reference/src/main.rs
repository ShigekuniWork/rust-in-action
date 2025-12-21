use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;


fn show(table: &mut Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&mut table);
    
    println!("{:?}", table);
    
    sort_works(&mut table);
    
    
    let mut y = 32;
    let m = &mut y;
    
    *m += 32;
    assert_eq!(*m, 64);
    
    let parabola = [9, 4];
    let s = smallest(&parabola);

    assert_eq!(*s, 0);
}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut res = &v[0];
        for n in v.iter() {
            if *n < *res {
                res = n;
            }
        }
        res
}