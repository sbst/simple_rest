extern crate iron;
#[macro_use]
extern crate router;

use iron::Handler;
use iron::status;
use iron::IronResult;
use iron::Response;
use iron::Request;
use iron::Iron;

struct IndexRoute {
	name: String
}

impl Handler for IndexRoute {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, self.name.clone())))
    }
}

fn main() {
    let index = IndexRoute {
        name: "index".to_string()
    };

let router = router! { index: get  "/"    => index };
    Iron::new(router).http("localhost:3000").unwrap();
}
