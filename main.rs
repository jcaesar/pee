use std::collections::VecDeque;

fn main() {
    let mut argv = std::env::args_os().collect::<VecDeque<_>>();
    let pee = &argv.pop_front().expect("argv length is 0");
    let help = || -> ! {
            let pee = pee.to_str().unwrap_or("pee".into());
            print!(concat!("pee - put something into a file: like tee, without the stdout.\n",
            "\n",
            "Usage: {} [-ah-] {{FILE}} {{CONTENT}}...\n",
            " -a         Append instead of overwrite\n",
            " --         Overwrite instead of append\n",
            "            default, only necessary if FILE starts with a -\n",
            " -h         Print this help\n",
            " FILE       Write to this path\n",
            " CONTENT    Write all remaining arguments - will read stdin if empty\n")
            , pee);
            std::process::exit(0);
    };
    let append;
    let file;
    let front = argv.pop_front().unwrap_or_else(|| help());

    match front.to_str() {
        Some("-h") => help(),
        Some("-a") => { file = None; append = true; },
        Some("--") => { file = None; append = false; },
        _ => {
            append = false;
            let tsl = front.to_string_lossy();
            if &tsl[0..1] == "-" {
                eprintln!("Unknown argument {}, try -h.", tsl);
                std::process::exit(-1);
            } else {
                file = Some(front)
            }
        }
    };
    let file = file.unwrap_or_else(|| argv.pop_front().unwrap_or_else(|| help()));

    let mut content;
    if argv.is_empty() {
        content = vec!();
        std::io::Read::read_to_end(&mut std::io::stdin(), &mut content).ok();
    } else {
        #[cfg(any(unix, target_os = "wasi"))]
        {
            #[cfg(unix)]
            use std::os::unix::ffi::OsStringExt;
            #[cfg(target_os = "wasi")]
            use std::os::wasi::ffi::OsStringExt;
            let argv = argv.into_iter().map(|s| s.into_vec()).collect::<Vec<_>>();
            content = argv.join(&32);
            content.push(10);
        }
        #[cfg(not(any(unix, target_os = "wasi")))]
        {}
    }
    let file = &mut std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .open(&file)
        .expect(&format!("Could not open {:?} for writing", file));
    std::io::Write::write_all(file, &content).expect("Couldn't write to FILE");
}
