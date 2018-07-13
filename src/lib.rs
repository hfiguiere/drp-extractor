

#[macro_use]
extern crate try_opt;
extern crate xml;
extern crate zip;

mod project;
mod mediapool;
mod sequence;

use std::fs::File;
use std::path::Path;

use project::Project;
use mediapool::MediaPool;
use sequence::Sequence;

pub struct ProjectArchive {
    project: Project,
    mediapool: MediaPool,
    sequence: Sequence,
}

pub fn read_project_archive<T: AsRef<Path>>(path: T) -> Option<ProjectArchive> {
    let file = try_opt!(File::open(path).ok());

    // open archive
    let mut zip = try_opt!(zip::ZipArchive::new(file).ok());

    // extract XML files
    if let Ok(project_file) = zip.by_name("project.xml") {
        // read project file

    }

    None
}
