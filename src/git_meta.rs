use git2::{ErrorCode, Repository};
use std::{fs, path::Path};

fn name_from_unborn_branch(repo: &Repository, e: git2::Error) -> Option<String> {
    if e.code() == ErrorCode::UnbornBranch {
        // HEAD should only be an unborn branch if the repository is fresh,
        // in that case read directly from `.git/HEAD`
        let mut head_path = repo.path().to_path_buf();
        head_path.push("HEAD");

        // get first line, then last path segment
        fs::read_to_string(&head_path)
            .ok()?
            .lines()
            .next()?
            .trim()
            .split('/')
            .last()
            .map(std::borrow::ToOwned::to_owned)
    } else {
        None
    }
}

pub fn get_current_branch(path: &Path) -> Result<Option<String>, git2::Error> {
    let repo = Repository::discover(path);
    if repo.is_err() {
        return Ok(None);
    }
    let repo = repo.unwrap();
    let head = match repo.head() {
        Ok(reference) => reference,
        Err(e) => return Ok(name_from_unborn_branch(&repo, e)),
    };

    let shorthand = head.shorthand();

    Ok(shorthand.map(std::string::ToString::to_string))
}
