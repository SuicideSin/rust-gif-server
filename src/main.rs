// mostly pulled from https://github.com/ligthyear/clippy-service/blob/master/src/main.rs
extern crate iron;
// extern crate staticf;
extern crate mount;

#[macro_use]
extern crate router;

#[macro_use]
extern crate log;
extern crate env_logger;

// import what we use
use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;

use std::fmt;
use std::fs;
use std::env;
use std::path::{Path, PathBuf};

fn root(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "root handler")))
}

fn show(path: PathBuf) -> IronResult<Response> {
    return Ok(Response::with((status::Ok, format!("exists: {}", path.display()))));
}

fn browse(req: &mut Request) -> IronResult<Response> {
    let root = env::current_dir().unwrap();
    let content = root.join("media");

    let query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("./");
    let querypath = content.join(Path::new(query));

    // ensure within whatever
    if !querypath.starts_with(&content) {
        return Ok(Response::with((
                    status::Forbidden,
                    format!("Requested path {req} outside of root {root}",
                            req=querypath.display(),
                            root=content.display())
                    )));
    }

    // get canonical path - if this is a 404, then 404
    let canonical_path = match querypath.canonicalize() {
        Ok(path) => path,
        Err(_) => return Ok(Response::with((
                    status::NotFound,
                    format!("path {} does not exist", querypath.display())
                    ))),
    };

    if canonical_path.is_dir() {
        // TODO: implement show_directory
        return show(canonical_path);
    } else {
        // TODO: implement show_file
        return show(canonical_path);
    }
}

// path: /media/:name:/thumbnail:?w_max=xxx&h_max=xxx
// should resize the media, them make a cache
// video -> gif thumbnail
// gif ->   gif thumbnail
 //still -> still thumbnail
//fn media_thumbnail(req: &mut Req) -> IronResult<Response> {
     //wat
//}

 //path: /media/:name:/original
 //this is just the static file, as-is
//fn media_original(req: &mut Req) -> IronResult<Response> {
     //wat
//}

 //path: dynamic, like /folders/:folder_name:
 //html view
//fn folder_view(req: &mut Req) -> IronResult<Response> {
     //wat
//}

 //path: dynamic, like /folders/:folder_name/:name:/
 //html view
//fn image_detail_view(req: &mut Req) -> IronResult<Response> {
     //wat
//}

fn main() {
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/", root);

    // gotta do both because the /browse/:query won't catch GET /browse/
    router.get("/browse", browse);
    router.get("/browse/:query", browse);

    Iron::new(router).http("localhost:3333").unwrap();

    let parent = Path::new("./content");
    let child = Path::new("./content/foo/../../../bar");
    assert!(child.starts_with(parent));




    ();
}
