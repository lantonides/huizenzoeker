extern crate kuchiki;
extern crate reqwest;

use std::io::Read;
//use kuchiki::parse_html;
use kuchiki::traits::*;
use reqwest::get;

static URL: &'static str = "https://www.huizenzoeker.nl/koop/zuid-holland/alphen-aan-den-rijn/?pv=225000&pt=275000&sts=normaal&so=1";

fn main() {
    let mut response = get(URL).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

 //   println!("text {}",body);
    
    let css_selector = ".lijst strong";

    let document = kuchiki::parse_html().one(body);

    for css_match in document.select(css_selector).unwrap() {
        // css_match is a NodeDataRef, but most of the interesting methods are
        // on NodeRef. Let's get the underlying NodeRef.
        let as_node = css_match.as_node();

        // In this example, as_node represents an HTML node like
        //
        //   <p class='foo'>Hello world!</p>"
        //
        // Which is distinct from just 'Hello world!'. To get rid of that <p>
        // tag, we're going to get each element's first child, which will be
        // a "text" node.
        //
        // There are other kinds of nodes, of course. The possibilities are all
        // listed in the `NodeData` enum in this crate.
        let text_node = as_node.first_child().unwrap();

        // Let's get the actual text in this text node. A text node wraps around
        // a RefCell<String>, so we need to call borrow() to get a &str out.
        let text = text_node.as_text().unwrap().borrow();

        // Prints:
        //
        //  "Hello, world!"
        //  "I love HTML"
        println!("{:?}", text);
    }
}