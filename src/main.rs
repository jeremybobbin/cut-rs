use std::env;
use std::collections::HashSet;
use std::io::{
    self,
    BufReader,
    BufRead,
};

struct Options {
    delim: char,
    bounds: Vec<usize>
}

fn main() {

    let delim = ':';
    let opts = parse_args().unwrap();
    cut(opts.delim, opts.bounds); 
}

fn parse_args<'a>() -> Result<Options, &'a str> {
    let mut possible_args: HashSet<String> = vec!["-d", "-f"]
        .into_iter()
        .map(str::to_string)
        .collect();

    let mut preceding: Option<String> = None;
    let mut bounds = Vec::new();
    let mut delim = ':';
    for arg in env::args().skip(1) {
        if let Some(ref p) = preceding {
            match &p[..] {
                "-d" => {
                    if arg.len() > 1 {
                        return Err("Delimiter must be one character");
                    }
                    delim = arg.chars()
                        .next()
                        .ok_or("Delimiter requires a parameter")?;
                    preceding = None;
                },
                "-f" => {
                    parse_bounds(arg, &mut bounds);
                    preceding = None;
                },
                _ => {}
            }
        } else {
            if possible_args.remove(&arg) {
                preceding = Some(arg);
            } else {
                return Err("Tried to do something")
            }
        }
    }

    Ok(Options {
        delim,
        bounds
    })
}
fn parse_bounds<'a>(arg: String, res: &mut Vec<usize>) -> Result<(), &'a str>{
    for sel in arg.split(',') {
        if sel.contains('-') {
            let mut range = sel.split('-');
            let beg: usize = range.next()
                .ok_or("Need to specify numbers on either side of '-'.")?
                .parse()
                .map_err(|_| "could not parse...")?;
            let end: usize = range.next()
                .ok_or("Range requires both numbers for now.")?
                .parse()
                .map_err(|_| "could not parse...")?;

            if beg > end || beg < 1 {
                return Err("Range was out of bounds...");
            }
            let mut nums: Vec<usize> = (beg - 1..end - 1).collect();

            res.append(&mut nums)

        } else {
            let num: usize = sel.parse()
                .map_err(|_| "could not parse...")?;

            if num < 1 {
                return Err("Number must be greater than 1.")
            }
            res.push(num - 1);
        }
    }
    Ok(())
}

fn cut(delim: char, columns: Vec<usize>) -> io::Result<()>
{
    let stdin = BufReader::new(io::stdin());
    let lines = stdin
        .lines()
        .filter_map(Result::ok);

    for line in lines {

        let fields: Vec<String>= line.split(delim)
            .enumerate()
            .filter(|(i, f)| columns.contains(i))
            .map(|(i, f)| f.to_string())
            .collect();

        println!("{}", fields.join(&delim.to_string()));
    }

    Ok(())
}
