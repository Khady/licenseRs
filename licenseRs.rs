#[link(name = "licenseRs", vers = "0.2", author = "Louis Roché")]

extern mod std;
use std::getopts::*;
use std::time::now;
use core::container::Map;
use core::hashmap::linear::LinearMap;
use core::io::ReaderUtil;

fn print_usage(program: &str, _opts: &[std::getopts::Opt]) {
    io::println(fmt!("Usage: %s [options]", program));
    io::println("-h --help\tUsage");
    io::println("--proj\t\tProject name, defaults to name of current directory");
    io::println("--year\t\tCopyright year");
    io::println("--org\t\tOrganization, defaults to $USER");
    io::println("--license\tThe license to generate, one of: agpl3, apache, bsd2, \
                 bsd3, cddl, cc0, epl, gpl2, gpl3, lgpl, mit, mpl");
    io::println("--template\tPath to license template file");
    io::println("--vars\t\tList template variables for specified license");
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

fn replace_keys(origin_template: ~[u8], context: LinearMap<~str, ~str>) {
    let mut template = str::from_bytes(origin_template);
    let keys : [~str * 3] = [~"year", ~"project", ~"organization"];
    for keys.each |k| {
        template = str::replace(template, ~"{{ " + *k + ~" }}", *context.get(k));
    }
    io::println(template);
}

fn get_template_vars(template: ~str) -> ~[~str] {
    let vars_tmp = str::split_str_nonempty(template, ~"{{");
    let mut vars: ~[~str] = ~[];
    for vars_tmp.each |&var| {
        let end = str::find_str(var, "}}");
        match end {
            Some(pos) => { vars.push(var.substr(0, pos).trim()) },
            None => { }
        }
    }
    vars
}

fn display_template_vars(origin_template: ~[u8]) {
    let mut template = str::from_bytes(origin_template);
    let vars = get_template_vars(template);
    for vars.each |var| {
        io::println(*var);
    }
}

fn main() {
    let args = os::args();

    let program = copy args[0];

    let opts = ~[
        optflag(~"h"),
        optflag(~"help"),
        optflag(~"vars"),
        optopt(~"year"),
        optopt(~"proj"),
        optopt(~"org"),
        optopt(~"template"),
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
    let template_path = opt_maybe_str(&matches, ~"template_path");

    let mut context: LinearMap<~str, ~str> = LinearMap::new();
    context.insert(~"year", get_year(year));
    context.insert(~"project", get_proj(proj));
    context.insert(~"organization", get_org(org));
    context.insert(~"license", get_license(license));

    let template = load_file_template(~"template-" + *context.get(&~"license") + ~".txt");

    if opt_present(&matches, "vars") {
        display_template_vars(template);
    } else {
        replace_keys(template, context);
    }
}