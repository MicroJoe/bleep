use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

const BUILTINS_DIR: &str = "builtins";

fn parse_dir(
    map: &mut phf_codegen::Map<String>,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    parse_dir_rec(map, path, path)
}

fn parse_dir_rec(
    map: &mut phf_codegen::Map<String>,
    basepath: &Path,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for f in fs::read_dir(path)? {
        let f = f?;

        let ft = f.file_type()?;
        if ft.is_dir() {
            parse_dir_rec(map, basepath, f.path().as_path())?;
            continue;
        } else if !ft.is_file() && !ft.is_symlink() {
            println!("cargo:warning=file {:?} was not recognized", &f);
            continue;
        }

        let path = f.path();
        let abspath = env::current_dir()?.join(path);

        let key = f
            .path()
            .file_stem()
            .ok_or("Could not get stem")?
            .to_str()
            .ok_or("could not parse stem")?
            .into();
        let value = format!(r##"include_bytes!(r#"{}"#)"##, abspath.as_path().display());

        map.entry(key, &value);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir_path = Path::new(&out_dir);

    let dest_path = out_dir_path.join("builtins.rs");
    let mut file = File::create(&dest_path)?;

    let mut map: phf_codegen::Map<String> = phf_codegen::Map::new();

    parse_dir(&mut map, Path::new(BUILTINS_DIR))?;

    writeln!(
        &mut file,
        "pub static BUILTINS: phf::Map<&'static str, &'static [u8]> = \n{};\n",
        map.build()
    )
    .unwrap();

    Ok(())
}
