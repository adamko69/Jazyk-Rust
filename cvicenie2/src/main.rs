mod task;

use std::io::{Write, stdin, stdout};

use chrono::{Local, NaiveDate, TimeDelta};

use task::task::Task;

use crate::task::task::TaskManager;

fn main() {
    let mut tm: TaskManager = TaskManager::new();

    let t = Task {
        id: 0,
        nazov: String::from("Task"),
        popis: String::from("popis tasku"),
        priorita: 0,
        planovany_zaciatok: Local::now().date_naive(),
        skutocny_zaciatok: Option::from(Local::now().date_naive()),
        planovane_trvanie: TimeDelta::days(1),
        skutocne_trvanie: Option::from(TimeDelta::days(1)),
    };
    // upravme vytvaranie instancii struktury
    // aby bolo nacitane od pouzivatela z klavesnice

    tm.add_task(t);

    println!("{tm:?}");

    let task = tm.find_task_by_id(5);

    println!("{task:?}");

    let id: i32 = read_i32("Zadajte ID tasku: ");
    let nazov = read_string("Zadajte nazov tasku: ");
    let popis = read_string("Zadajte popis tasku: ");
    let priorita = read_i32("Zadajte prioritu: ");
    let planovany_zaciatok = read_date("Zadajte planovany datum: ");
    let skutocny_zaciatok = Option::from(read_date("Zadajte skutocny datum: "));
    let planovane_trvanie = TimeDelta::days(read_i32("Zadajte planovane trvanie: ").into());
    let skutocne_trvanie = Option::from(TimeDelta::days(
        read_i32("Zadajte skutocne trvanie: ").into(),
    ));
    let t = Task {
        id,
        nazov,
        popis,
        priorita,
        planovany_zaciatok,
        skutocny_zaciatok,
        planovane_trvanie,
        skutocne_trvanie,
    };
    println!("{t:?}");
    let skutocny_koniec = t.vypocitaj_skutocny_koniec();
    println!("{skutocny_koniec:?}");

    stdout().flush();

    tm.print_all_tasks();
}

fn read_string(message: &str) -> String {
    print!("{message}");
    stdout().flush().expect("Error while flushing");
    let mut buffer = String::new();
    let _ = stdin().read_line(&mut buffer);
    buffer
}
// nacitanie cisla
fn read_i32(message: &str) -> i32 {
    let mut buffer = read_string(message);
    let cislo = buffer.trim().parse().expect("Cannot be parsed to i32");
    cislo
}
// nacitanie datumu
fn read_date(message: &str) -> NaiveDate {
    let mut buffer = read_string(message);
    let datum =
        NaiveDate::parse_from_str(buffer.trim(), "%d.%m.%Y").expect("Cannot parse from string");
    datum
}
