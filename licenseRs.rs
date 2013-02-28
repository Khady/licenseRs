extern mod std;
use std::getopts::*;
use std::time::*;
//use core::container::Map;
use core::hashmap::linear::LinearMap;
use core::io::ReaderUtil;

fn print_usage(program: &str, _opts: &[std::getopts::Opt]) {
    io::println(fmt!("Usage: %s [options]", program));
    io::println("-o\t\tOutput");
    io::println("-h --help\tUsage");
}

// fn make_absolute(p: &Path) -> Path

fn get_license(args: Option<~str>) -> ~str {
    match args {
        Some(x) => x.to_lower(),
        None => ~"bsd3"
    }
}

fn get_proj(args: Option<~str>) -> ~str {
    match args {
        Some(x) => x,
        None => {
            match os::getcwd().filename() {
                Some(dir) => dir,
                None => ~"project"
            }
        }
    }
}

fn get_org(args: Option<~str>) -> ~str {
    match args {
        Some(x) => x,
            None => {
                match os::getenv("USER") {
                    Some(dir) => dir,
                    None => ~"organization"
                }
            }
        }
}

fn get_year(args: Option<~str>) -> ~str {
    match args {
        Some(x) => x,
        None => {
            let time = now();
            time.strftime("%Y")
        }
    }
}

fn load_file_template(file: ~str) -> ~str {
    io::println(file);
    let open_file = &io::file_reader(&Path(file));
    let test_reader = result::get(open_file);
    test_reader.each_line(fn@ (line: &str) -> bool {
        io::println(line);
        true
    });
   // check if file exist. If true, replace with a regexp. Else raise an error
    ~"error"
}

fn main() {
    let args = os::args();

    let program = copy args[0];

    let opts = ~[
        optflag(~"h"),
        optflag(~"help"),
        optopt(~"year"),
        optopt(~"proj"),
        optopt(~"org"),
        optopt(~"license")
    ];
    let matches = match getopts(vec::tail(args), opts) {
        result::Ok(m) => { m }
        result::Err(f) => { fail!(fail_str(f)) }
    };
    if opt_present(&matches, "h") || opt_present(&matches, "help") {
        print_usage(program, opts);
        return;
    }
    let year = opt_maybe_str(&matches, ~"year");
    let proj = opt_maybe_str(&matches, ~"proj");
    let org = opt_maybe_str(&matches, ~"org");
    let license = opt_maybe_str(&matches, ~"license");

    // io::println(get_year(year));
    // io::println(get_proj(proj));
    // io::println(get_org(org));
    // io::println(get_license(license));

    let mut context: LinearMap<~str, ~str> = LinearMap::new::<~str, ~str>();
    context.insert(~"year", get_year(year));
    context.insert(~"proj", get_proj(proj));
    context.insert(~"org", get_org(org));
    context.insert(~"license", get_license(license));

    let template = load_file_template(~"template-" + *context.get(&~"license") + ~".txt");
    // io::println(template);
    // let content = generate_license(template);
}