extern mod std;
use std::getopts::*;
use std::time::*;
//use core::container::Map;
use core::hashmap::linear::LinearMap;
use core::io::ReaderUtil;

fn print_usage(program: &str, _opts: &[std::getopts::Opt]) {
    io::println(fmt!("Usage: %s [options]", program));
    io::println("--proj\t\tProject name");
    io::println("--year\t\tYear");
    io::println("--org\t\tOrganization");
    io::println("-h --help\tUsage");
}

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

fn load_file_template(file: ~str) -> ~[u8] {
    let open_file = &io::file_reader(&Path(file));
    if result::is_err(open_file) {
        fail!(~"Cannot open license file");
    }
    let license = result::get(open_file);
    license.read_whole_stream()
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

    let mut context: LinearMap<~str, ~str> = LinearMap::new::<~str, ~str>();
    context.insert(~"year", get_year(year));
    context.insert(~"project", get_proj(proj));
    context.insert(~"organization", get_org(org));
    context.insert(~"license", get_license(license));

    let template = load_file_template(~"template-" + *context.get(&~"license") + ~".txt");
    let mut template_str = str::from_bytes(template);

    let keys : [~str * 3] = [~"year", ~"project", ~"organization"];

    for keys.each |k| {
        template_str = str::replace(template_str, ~"{{ " + *k + ~" }}", *context.get(k));
    }
    io::println(template_str);
}