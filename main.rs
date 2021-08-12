use std::collections::VecDeque;
use std::ffi::OsString;

fn main() {
    mein(std::env::args_os().collect::<VecDeque<_>>())
}
fn mein(mut argv: VecDeque<OsString>) {
    let pee = &argv.pop_front().expect("argv length is 0");
    let help = || -> ! {
        let pee = pee.to_str().unwrap_or("pee".into());
        print!(
            concat!(
                "pee - put something into a file: like tee, without the stdout.\n",
                "\n",
                "Usage: {} [-ah-] {{FILE}} {{CONTENT}}...\n",
                "  -a         Append instead of overwrite\n",
                "  --         Overwrite instead of append\n",
                "             default, only necessary if FILE starts with a -\n",
                "  -h         Print this help\n",
                "  FILE       Write to this path\n",
                "  CONTENT    Write all remaining arguments - will read stdin if empty\n"
            ),
            pee
        );
        std::process::exit(0);
    };
    let append;
    let file;
    let front = argv.pop_front().unwrap_or_else(|| help());

    match front.to_str() {
        Some("-h") => help(),
        Some("-a") => {
            file = None;
            append = true;
        }
        Some("--") => {
            file = None;
            append = false;
        }
        _ => {
            append = false;
            let tsl = front.to_string_lossy();
            if tsl.chars().next() == Some('-') {
                eprintln!("Unknown argument {}, try -h.", tsl);
                std::process::exit(-1);
            } else {
                file = Some(front)
            }
        }
    };
    let file = file.unwrap_or_else(|| argv.pop_front().unwrap_or_else(|| help()));

    let content = if argv.is_empty() {
        let mut content = vec![];
        std::io::Read::read_to_end(&mut std::io::stdin(), &mut content).ok();
        content
    } else {
        #[cfg(any(unix, target_os = "wasi"))]
        {
            #[cfg(unix)]
            use std::os::unix::ffi::OsStringExt;
            #[cfg(target_os = "wasi")]
            use std::os::wasi::ffi::OsStringExt;
            let argv = argv.into_iter().map(|s| s.into_vec()).collect::<Vec<_>>();
            let mut content = argv.join(&32);
            content.push(10);
            content
        }
        #[cfg(not(any(unix, target_os = "wasi")))]
        {
            let argv = argv
                .into_iter()
                .map(|s| s.into_string())
                .collect::<Result<Vec<_>, _>>()
                .expect("UTF-8 arguments");
            (argv.join(" ") + "\n").into_bytes()
        }
    };
    let file = &mut std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(&file)
        .expect(&format!("Could not open {:?} for writing", file));
    std::io::Write::write_all(file, &content).expect("Couldn't write to FILE");
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;
    use std::fs::read_to_string;
    use tempfile::{tempdir, TempDir};

    fn mein(argv: Vec<&str>) {
        super::mein(argv.into_iter().map(|x| x.into()).collect::<VecDeque<_>>());
    }

    fn tempf(s: &str) -> (TempDir, String) {
        let d = tempdir().unwrap();
        let p = d.path().join(s);
        (d, p.to_str().unwrap().into())
    }

    #[test]
    fn plain() {
        let (_d, f) = tempf("f");
        mein(vec!["selfy", &f, "asdf"]);
        assert_eq!(read_to_string(f).unwrap(), "asdf\n");
    }

    #[test]
    fn over() {
        let (_d, f) = tempf("f");
        mein(vec!["selfy", &f, "asdffff"]);
        mein(vec!["selfy", &f, "asd"]);
        assert_eq!(read_to_string(f).unwrap(), "asd\n");
    }

    #[test]
    fn append() {
        let (_d, f) = tempf("f");
        mein(vec!["selfy", "-a", &f, "asdffff"]);
        mein(vec!["selfy", &f, "asd"]);
        mein(vec!["selfy", "-a", &f, "bsd"]);
        assert_eq!(read_to_string(f).unwrap(), "asd\nbsd\n");
    }

    #[test]
    fn dashname() {
        let (_d, f) = tempf("-f");
        mein(vec!["selfy", "--", &f, "asdf"]);
        assert_eq!(read_to_string(f).unwrap(), "asdf\n");
    }
}
