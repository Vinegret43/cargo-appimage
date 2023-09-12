use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let here_dir = std::path::PathBuf::from(std::env::current_exe()?);
    let orig_parent = here_dir
        .parent()
        .with_context(|| format!("{} has no parent directory", &here_dir.display()))?;
    let parent = orig_parent
        .parent()
        .with_context(|| format!("{} has no parent directory", &orig_parent.display()))?;
    std::env::set_current_dir(&parent)?;
    std::env::set_var(
        "LD_LIBRARY_PATH",
        format!("{}/usr/lib/:{}/usr/lib/i386-linux-gnu/:{}/usr/lib/x86_64-linux-gnu/:{}/usr/lib32/:{}/usr/lib64/:{}/lib/:{}/lib/i386-linux-gnu/:{}/lib/x86_64-linux-gnu/:{}/lib32/:{}/lib64/{}", parent.display(), parent.display(), parent.display(), parent.display(), parent.display(), parent.display(), parent.display(), parent.display(), parent.display(), parent.display(), if let Ok(ldlibpath) = std::env::var("LD_LIBRARY_PATH") { ":".to_string() + &ldlibpath } else { String::new() }),
    );
    std::env::set_var(
        "XDG_DATA_DIRS",
        format!(
            "XDG_DATA_DIRS={}:{}",
            parent.join("usr/share").display(),
            std::env::var("XDG_DATA_DIRS").unwrap_or(String::new())
        ),
    );

    let err = exec::execvp(orig_parent.join("usr/bin/bin"), std::env::args());
    eprintln!("Error: {}", err);
    Ok(())
}

// .env(
//            "LD_LIBRARY_PATH",
//            if let Ok(env) = std::env::var("LD_LIBRARY_PATH") {
//                args.map(|i| i + ":").collect::<String>() + &env
//            } else {
//                args.map(|i| i + ":").collect::<String>()
//            },
//        )
