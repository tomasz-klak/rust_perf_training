#![allow(dead_code)]

use memmap2::Mmap;
use rustc_hash::FxHashMap;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

type Data = FxHashMap<String, CountryData>;

#[derive(Debug, Default)]
struct CountryData {
    id: Vec<u64>,
    author_age: Vec<u64>,
    pages: Vec<u64>,
    publication_age: Vec<u64>,
    author_nationality: Vec<String>,
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
    let file = File::open(format!("{}/../data.csv", env!("CARGO_MANIFEST_DIR")))?;
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
        if all_data.contains_key(country) {
            let e = all_data.get_mut(country).unwrap();
            e.id.push(id);
            e.author_age.push(author_age);
            e.pages.push(pages);
            e.publication_age.push(publication_age);
            e.author_nationality.push(author_nationality);
        } else {
            let mut e = CountryData::default();
            e.id.push(id);
            e.author_age.push(author_age);
            e.pages.push(pages);
            e.publication_age.push(publication_age);
            e.author_nationality.push(author_nationality);
            all_data.insert(country.to_owned(), e);
        }
    }
    Ok(all_data)
}

#[inline(never)]
fn analyse(data: Data) {
    data.into_iter()
        .for_each(|(country, data)| analyse_country(country, data));
}

fn analyse_country(country: String, data: CountryData) {
    let author_age = analyse_prop(data.author_age);
    let pages = analyse_prop(data.pages);
    let publication_age = analyse_prop(data.publication_age);
    let mut pub_by_nationality: FxHashMap<_, usize> = Default::default();
    for nat in data.author_nationality.into_iter() {
        *pub_by_nationality.entry(nat).or_default() += 1;
    }
    let most_common_nat = pub_by_nationality.into_iter().max_by_key(|(_, v)| *v);
    println!("{country} stats: most common nationality: {most_common_nat:?} author_age: {author_age:?} pages: {pages:?} publication_age: {publication_age:?}");
}

fn analyse_prop(mut values: Vec<u64>) -> Stats {
    values.sort_unstable();
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
