#[macro_use]
extern crate clap;

use std::{
    borrow::Cow,
    fs::{self, File},
    io::{self, BufReader, ErrorKind},
    path::Path,
};

use hashbrown::HashMap;
use walkdir::WalkDir;

struct PngDateTime {
    s: String,
}

impl PngDateTime {
    fn get_date_time(path: &Path) -> Self {
        let file = File::open(path).unwrap();
        let exif = exif::Reader::new(&mut BufReader::new(&file)).unwrap();
        let dt_tag = exif::Tag::DateTimeOriginal;
        let dt_field = exif.get_field(dt_tag, false).unwrap();
        Self {
            s: format!("{}", dt_field.value.display_as(dt_tag)),
        }
    }

    fn copy_file_to_dest(&self, root_dir: &Path, target_root: &Path, key_map: &mut HashMap<String, usize>) -> io::Result<()> {
        let entry = key_map.entry(self.s.clone());
        let value = entry.or_insert(0);
        *value += 1;
        let duplicate_value = match value {
            1 => Cow::Borrowed(""),
            n => Cow::Owned("-".to_string() + &n.to_string()),
        };
        let r = &self.s[0..4];
        let year_root = target_root.join(r);
        create_dir(&year_root)?;
        fs::copy(root_dir, year_root.join(self.s.clone() + &duplicate_value).with_extension("jpg"))?;
        Ok(())
    }
}

fn create_dir(path: &Path) -> io::Result<()> {
    match fs::create_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => Ok(()),
            _ => Err(e),
        },
    }
}

fn main() -> io::Result<()> {
    let matches = clap_app!(png_fix =>
                            (version: "0.0.1")
                            (author: "Roald Strangstadstuen")
                            (about: "Standarizes jpeg cluster into nicer standard")
                            (@arg input: +required "Sets the input dir to use")
                            (@arg target: +required "Sets the output dir to use")
    )
    .get_matches();
    let target_root = Path::new(matches.value_of("target").unwrap());
    create_dir(target_root)?;
    let mut key_counter = HashMap::new();
    let copied = WalkDir::new(matches.value_of("input").unwrap())
        .same_file_system(true)
        .follow_links(false)
        .into_iter()
        .map(|v| v.unwrap_or_else(|e| panic!("Error happend {:?}", e)))
        .filter(|n| n.file_type().is_file())
        .filter(|n| {
            n.path()
                .extension()
                .filter(|n| {
                    n.to_str()
                        .map(|c| c.eq_ignore_ascii_case("jpg"))
                        .filter(|&n| n)
                        .is_some()
                })
                .is_some()
        })
        .map(|f| {
            let dt = PngDateTime::get_date_time(f.path());
            dt.copy_file_to_dest(f.path(), target_root, &mut key_counter)
        })
        .count();
    println!("Number of pictures copied: {}", copied);
    Ok(())
}
