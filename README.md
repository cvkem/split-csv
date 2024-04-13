# Split_csv
This programm can be used to split large CSV-file (even if too large to fit into memory) into a set of smaller files. 
The splitting of files is based on a line-count, or a column can be selected to provide a grouping-label, where each group is emitted to a separate file.

The available commands are:
* count: count the total number of lines in a file (including the headline)
* group_by: group all rows based on a label taken from a specific column (column 1 by default) and write each set to a separate file
* head: Show the first 'num_header_lines'  of the file (default 'num_header_lines' = 3)
* split: Split the file in a series of files, where each file contains at most 'num_lines' data-rows

The help of the command, as shown by `split_csv.exe --help` shows:
```
Usage: split_csv.exe [OPTIONS] --file <FILE> <COMMAND>

Arguments:
  <COMMAND>  commmand that needs to be performed ('count', 'head', 'group_by', or 'split')

Options:
  -f, --file <FILE>                        name of the input-file
  -l, --lines <LINES>                      Maximum number of data-rows per file [default: 1000000]
  -g, --group-by-column <GROUP_BY_COLUMN>  Grouping values by the column [default: 1]
  -n, --num-head-lines <NUM_HEAD_LINES>    Number of headlines shown by 'head' command [default: 3]
  -s, --separator <SEPARATOR>              Separator used between columns [default: ,]
  -h, --help                               Print help
  -V, --version                            Print version
  ```

Some examples of running the tool is
```
# show the first 3 lines of a csv-file (or other text-file)
split_csv.exe head -f test.csv

# show the first 10 lines
split_csv.exe head --file test.csv --num-head-lines 10

# show grouped by the label taken from the first column
split_csv.exe group_by  --file test.csv  

# group by the label taken from the second column, while using ';' as separator symbol between columns
split_csv.exe group_by  -f test.csv --group_by_column 2 --separator ; 

# split file in a series of files each containing at most 1,000,000 data-lines
split_csv.exe split  --file test.csv  

# split file in a series of files each containing at most 500 data-lines
split_csv.exe split  --file test.csv  --lines 500

```