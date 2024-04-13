use clap::Parser;

mod head;
use head::head_lines;

mod count_lines;
use count_lines::count_lines;

mod output_file;

mod split_lines;
use split_lines::split_lines;

mod group_by;
use group_by::group_by;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// commmand that needs to be performed ('count', 'head', 'group_by', or 'split')
    command: String,

    /// name of the input-file
    #[arg(short, long)]
    file: String,

    /// Maximum number of data-rows per file
    #[arg(short, long, default_value_t = 1_000_000)]
    lines: u64,

    /// Grouping values by the column
    #[arg(short, long, default_value_t = 1)]
    group_by_column: usize,

    /// Number of headlines shown by 'head' command.
    #[arg(short, long, default_value_t = 3)]
    num_head_lines: usize,
    
    /// Separator used between columns
    #[arg(short, long, default_value_t = ',')]
    separator: char,
}

fn main() {
    let args = Args::parse();

    match &args.command.to_lowercase()[..] {
        "count" => count_lines(args.file),
        "head" => head_lines(args.file, args.num_head_lines),
        "group_by" => group_by(args.file, args.separator, args.group_by_column).expect("Failed to write grouped"),
        "split" => split_lines(args.file, args.lines).expect("Failed to split file"),
        cmd => panic!("Unknown command: {cmd}\nValid options are: 'count', 'head', 'group_by', or 'split'.")
    }
}


