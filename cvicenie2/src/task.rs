use chrono::NaiveDate;

use crate::read_string;

pub mod task {
    use core::task;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    use chrono::{NaiveDate, TimeDelta};

    #[derive(Debug)]
    pub struct Task {
        pub id: i32,
        pub nazov: String,
        pub popis: String,
        pub priorita: i32,
        pub planovany_zaciatok: NaiveDate,
        pub skutocny_zaciatok: Option<NaiveDate>,
        pub planovane_trvanie: TimeDelta,
        pub skutocne_trvanie: Option<TimeDelta>,
    }

    impl Task {
        pub fn vypocitaj_planovany_koniec(&self) -> NaiveDate {
            self.planovany_zaciatok + self.planovane_trvanie
        }
        pub fn vypocitaj_skutocny_koniec(&self) -> Option<NaiveDate> {
            match self.skutocny_zaciatok {
                Some(zaciatok) => match self.skutocne_trvanie {
                    Some(trvanie) => Option::from(zaciatok + trvanie),
                    None => None,
                },
                None => None,
            }
        }
        pub fn print_task(&self) {
            let skutocny_zaciatok = match self.skutocny_zaciatok {
                Some(s) => s.to_string(),
                None => "-".to_string(),
            };

            let skutocny_koniec = match self.vypocitaj_skutocny_koniec() {
                Some(s) => s.to_string(),
                None => "-".to_string(),
            };

            println!(
                "{}: {}\t{}\t{}\t{}\t{}",
                self.id,
                self.nazov,
                self.planovany_zaciatok,
                self.vypocitaj_planovany_koniec(),
                skutocny_zaciatok,
                skutocny_koniec
            );
        }
    }
    #[derive(Debug)]
    pub struct TaskManager {
        tasks: Vec<Task>,
    }

    impl TaskManager {
        pub fn new() -> TaskManager {
            TaskManager { tasks: vec![] }
        }

        pub fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }

        pub fn find_task_by_id(&self, id: i32) -> Option<&Task> {
            self.tasks.iter().find(|t| t.id == id)
        }

        pub fn print_all_tasks(&self) -> () {
            for task in self.tasks.iter() {
                task.print_task();
            }
        }

        pub fn sort_tasks_planned_duration(&mut self) -> () {
            self.tasks
                .sort_by(|a, b| a.planovane_trvanie.cmp(&b.planovane_trvanie));
        }

        pub fn sort_tasks_planned_start(&mut self) -> () {
            self.tasks
                .sort_by(|a, b| a.planovany_zaciatok.cmp(&b.planovany_zaciatok));
        }

        pub fn read_from_csb_file(&mut self, file_path: &Path) -> () {
            let file = File::open(file_path).expect("Cannot open file");
            let file_lines = BufReader::new(file).lines();
            for line in file_lines {
                match line {
                    Ok(line) => {
                        let task = self._process_line(line);
                        self.add_task(task);
                        println!("{line}")
                    }
                    Err(e) => println!("{e}"),
                }
            }
        }
        fn _process_line(line: &str) -> Task {
            let split: Vec<&str> = line.split(',').collect();
            let id: i32 = split[0].trim().parse().unwrap_or(0);
            let nazov = split[1].trim().to_string();
            let popis = split[2].trim().to_string();
            let priorita: i32 = split[3].trim().parse().unwrap_or(0);
            let planovany_zaciatok = read_date(split[4].trim());
            let skutocny_zaciatok = if split[5].trim().is_empty() {
                None
            } else {
                Some(read_date(split[5].trim()))
            };
            let planovane_trvanie =
                TimeDelta::days(split[6].trim().parse::<i32>().unwrap_or(0).into());
            let skutocne_trvanie = if split[7].trim().is_empty() {
                None
            } else {
                Some(TimeDelta::days(
                    split[7].trim().parse::<i32>().unwrap_or(0).into(),
                ))
            };

            Task {
                id,
                nazov,
                popis,
                priorita,
                planovany_zaciatok,
                skutocny_zaciatok,
                planovane_trvanie,
                skutocne_trvanie,
            }
        }
    }
}

fn read_date(message: &str) -> NaiveDate {
    let mut buffer = read_string(message);
    let datum =
        NaiveDate::parse_from_str(buffer.trim(), "%d.%m.%Y").expect("Cannot parse from string");
    datum
}
