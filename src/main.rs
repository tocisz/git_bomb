use git2::{Error, Repository, Signature};

fn doit() -> Result<(), Error> {
    let repo = Repository::init("tmp")?;
    let mut t = repo.treebuilder(None).unwrap().write()?;
    println!("tree is {}", t.to_string());

    for _ in 0..10 {
        let mut b = repo.treebuilder(None)?;
        for i in 0..10 {
            let _entry = b.insert(format!("d{}", i), t, 0o040000)?;
        }
        t = b.write()?;
        println!("tree is {}", t.to_string());
    }

    let root = repo.find_tree(t)?;

    let signature = Signature::now("Tomasz", "x@y.com")?;
    let commit = repo.commit(None, &signature, &signature, "test", &root, &[])?;
    println!("commit is {}", commit.to_string());

    let c = repo.find_commit(commit)?;

    repo.branch("master", &c, false)?;

    // repo.set_head("refs/heads/master")?;
    // let mut head = repo.head()?;
    // head.set_target(commit, "Commit");

    Result::Ok(())
}

fn main() {
    // std::fs::remove_dir_all("./tmp").unwrap();
    match doit() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
