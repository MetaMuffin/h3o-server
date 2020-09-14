

use crate::server::router::PathMatcher::MatchFn;
use crate::server::router::PathMatcher::MatchRegex;
use crate::server::router::PathMatcher::MatchString;
use crate::server::router::HandleResult::Handled;
use crate::server::router::HandleResult::NoMatch;
use crate::server::router::HandleResult::Next;
use regex::Regex;
use crate::server::response::Response;
use crate::server::request::Request;

type RouteHandlerFn = fn(&Request,&mut Response) -> HandleResult;

pub enum HandleResult {
    Handled,
    Next,
    NoMatch,
}

pub enum PathMatcher {
    MatchString(String),
    MatchRegex(Regex),
    MatchFn(fn(&str) -> Option<usize>)
}

pub struct Route {
    child_routes: Vec<Route>,
    handler: RouteHandlerFn,
    path: PathMatcher
}


impl Route {
    pub fn new(routes: Vec<Route>,index: RouteHandlerFn) -> Route {
        Route {
            child_routes: routes,
            handler: index,
            path: MatchFn(|_s| { Some(1) })
        }
    }


    pub fn respond(&self, path: &str, req: &Request, res: &mut Response) -> HandleResult {
        let match_at = match &self.path {
            MatchString(s) => if path.starts_with(s) { Some(s.len()) } else { None },
            MatchRegex(_r) => Some(0),
            MatchFn(f) => (f)(path)
        };
        match match_at {
            Some(mid) => {
                let (_handled, following) = path.split_at(mid);
                if following.len() < 1 { 
                    return (self.handler)(req,res);
                } else {
                    for sub_route in &self.child_routes {
                        match sub_route.respond(following, &req, res) {
                            Handled => { return Handled },
                            NoMatch => { return NoMatch },
                            Next => { continue; },
                        }
                    }
                    HandleResult::NoMatch
                }
            },
            None => HandleResult::Next,
        }
    }
}
