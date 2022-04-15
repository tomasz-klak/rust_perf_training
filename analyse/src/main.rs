#![allow(dead_code)]

use memmap2::Mmap;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

type Data = BTreeMap<String, Vec<Entry>>;

#[derive(Debug)]
struct Entry {
    id: u64,
    author_age: u64,
    pages: u64,
    publication_age: u64,
    author_nationality: String,
}

#[derive(Debug)]
struct Stats {
    min: u64,
    max: u64,
    mean: u64,
    med: u64,
    p95: u64,
}

#[inline(never)]
fn read_data() -> Result<Data, Box<dyn Error>> {
    let mut all_data: Data = Default::default();
    let file = File::open("../data.csv")?;
    let file = unsafe { Mmap::map(&file)? };
    let input: &str = std::str::from_utf8(file.as_ref())?;
    for line in input.lines().skip(1) {
        let mut items = line.split(", ");
        let country = items.next().unwrap();
        let id = items.next().unwrap().parse()?;
        let author_age = items.next().unwrap().parse()?;
        let pages = items.next().unwrap().parse()?;
        let publication_age = items.next().unwrap().parse()?;
        let author_nationality = items.next().unwrap().to_owned();
        let e = Entry {
            id,
            author_age,
            pages,
            publication_age,
            author_nationality,
        };
        if all_data.contains_key(country) {
            all_data.get_mut(country).unwrap().push(e);
        } else {
            all_data.insert(country.to_owned(), vec![e]);
        }
    }
    Ok(all_data)
}

#[inline(never)]
fn analyse(data: Data) {
    data.into_iter()
        .for_each(|(country, data)| analyse_country(country, data));
}

fn analyse_country(country: String, data: Vec<Entry>) {
    let author_age = analyse_prop(data.iter().map(|d| d.author_age));
    let pages = analyse_prop(data.iter().map(|d| d.pages));
    let publication_age = analyse_prop(data.iter().map(|d| d.publication_age));
    let mut pub_by_nationality: BTreeMap<String, usize> = Default::default();
    for nat in data.iter().map(|e| &e.author_nationality) {
        if pub_by_nationality.contains_key(nat) {
            *pub_by_nationality.get_mut(nat).unwrap() += 1;
        } else {
            pub_by_nationality.insert(nat.to_owned(), 1);
        }
    }
    let most_common_nat = pub_by_nationality.into_iter().max_by_key(|(_, v)| *v);
    println!("{country} stats: most common nationality: {most_common_nat:?} author_age: {author_age:?} pages: {pages:?} publication_age: {publication_age:?}");
}

fn analyse_prop(values: impl Iterator<Item = u64>) -> Stats {
    let mut values: Vec<u64> = values.collect();
    values.sort();
    let mean = values.iter().sum::<u64>() / values.len() as u64;
    Stats {
        mean,
        min: values[0],
        max: *values.last().unwrap(),
        med: values[values.len() / 2],
        p95: values[(values.len() as f64 * 0.95) as usize],
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let total = Instant::now();
    let start = Instant::now();
    let all_data = read_data()?;
    eprintln!("read data: {:?}", start.elapsed());

    let start = Instant::now();
    analyse(all_data);
    eprintln!("  analyse: {:?}", start.elapsed());
    eprintln!("    total: {:?}", total.elapsed());
    Ok(())
}
