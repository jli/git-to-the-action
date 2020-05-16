use std::{path::{Component, PathBuf}, process::Command};

type Err = Box<dyn std::error::Error>;

fn main() -> Result<(), Err> {
    let paths = git_changed_paths()?;
    if paths.len() == 0 {
        return Ok(())
    }
    // paths.iter().for_each(|f| { println!("git path: {:?}", f) });
    let prefix = common_prefix(paths);
    // let root = git_repo_root()?;
    // println!("root: <{}>", root.join(prefix).display());
    println!("{}", prefix.display());
    Ok(())
}

fn git_changed_paths() -> Result<Vec<PathBuf>, Err> {
    // TODO: using porcelain is more stable, but --short gives relative paths, which saves us from an extra rev-parse --show-toplevel call.
    // let output = Command::new("git").arg("status").arg("--porcelain").output()?;
    let output = Command::new("git").arg("status").arg("--short").output()?;
    if !output.status.success() {
        panic!("Command failed: {:?}", output);
    }
    let paths = String::from_utf8(output.stdout)?
        .lines()
        // TODO: would be better to parse renames "->" and use latter file, or handle NUL parsing and use -z
        .map(|line| { &line[3..] })  // drop status prefix
        .map(|path| { PathBuf::from(path.clone()) })
        .collect();
    Ok(paths)
}

// fn git_repo_root() -> Result<PathBuf, Err> {
//     let output = Command::new("git").arg("rev-parse").arg("--show-toplevel").output()?;
//     if !output.status.success() { panic!("Command executed with failing error code"); }
//     let s = String::from_utf8(output.stdout)?;
//     Ok(PathBuf::from(s.trim_end()))
// }

fn common_prefix(paths: Vec<PathBuf>) -> PathBuf {
    let mut paths_components = vec![];
    for pathbuf in paths {
        let mut path_comps = vec![];
        for comp in pathbuf.components() {
            path_comps.push(comp.as_os_str().to_os_string());
        }
        paths_components.push(path_comps);
    }

    let mut prefix = PathBuf::new();
    // len - 1 to drop filename part
    let fewest_components = paths_components.iter().map(|c| { c.len() - 1 }).min().unwrap();
    // println!("fewest components: {:?}", fewest_components);
    for i in 0..fewest_components {
        let comp_of_first_path = &paths_components[0][i];
        let all_comps_eq =  paths_components.iter().all(|path_comps| &path_comps[i] == comp_of_first_path);
        if all_comps_eq {
            prefix.push(comp_of_first_path.clone());
        } else {
            break;
        }
    }
    if prefix.as_os_str().is_empty() {
        prefix.push(Component::CurDir);
    }
    prefix
}
