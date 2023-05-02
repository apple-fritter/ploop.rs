# ploop.rs
More robust and reliable than the original Bash script, with better memory management, improved error handling, and more accurate timing information.
For more information on the original project, please visit the [ploop.sh](https://github.com/apple-fritter/ploop.sh) GitHub page.

The original ploop was written in Bash and used the `read` command to read each line of a `TSV` file and process it one at a time. This Rust program, on the other hand, reads the entire file into memory as a `Vector` type, which allows for better memory management and faster processing.

In addition, the Rust program uses Rust's built-in `timer` functionality to record the start and end times of the program's execution, rather than using the `date` command in a Bash script.

## Functions and features:

Command-Line Argument Parsing: The program uses the clap crate to define and parse command-line arguments. It expects the following arguments:
```
'program_name': Name of the external program to be called.
'tsv_file': Path to the TSV file to process.
'num_columns': Number of columns in the TSV file.
'columns_to_process' (optional): Indices of the columns to process.
```
Input Validation: The program performs input validation on the command-line arguments. It ensures that the column indices provided in the columns_to_process argument are within the valid range (between 1 and num_columns). If an invalid column index is detected, an error message is displayed.

TSV File Processing: The program opens the specified TSV file and reads its contents line by line. It skips lines starting with '#'. Each non-skipped line is split into fields using the tab character ('\t') as the delimiter, resulting in a vector of field values.

Column Processing: Based on the provided columns_to_process argument, the program selects the relevant columns from each line. If columns_to_process is not provided, it processes all columns. The selected columns are joined together using a space character as the separator.

Calling External Program: The program calls the external program specified by the program_name argument, passing the processed columns as an argument. This code snippet is currently commented out in the provided implementation, so you would need to add the necessary code to make the actual external program call.

Error Handling: The program handles various potential errors. It reports any parsing errors that occur during the conversion of command-line arguments to the appropriate types. It also checks for errors when opening the TSV file and reading its contents. If any errors occur, they are propagated up and displayed to the user.R

## Notable changes
* The use of Rust's `standard error` handling mechanisms. In the original script, errors would simply be printed to the console and the program would continue running. The Rust program, however, prints errors to standard error and exits with a non-zero exit code, which is a common convention in Unix-based systems.

* When parsing the `columns_to_process` argument, there will be a validation check for each column index. If the column index is less than or equal to zero or greater than `num_columns`, an error is returned. The error message includes the specific invalid column index.

* The columns_to_process variable is now assigned `None` if any of the column indices are invalid. This way, it can handle the case of invalid column indices separately later in the code. When an invalid column index is provided, an error will be raised and propagated up the call stack. You can handle this error as needed, such as displaying an error message to the user or logging the error.

## Usage
### Processing All Columns
To process all columns in a TSV file named data.tsv using the external program myprogram, run the following command:

```bash
$ ./ploop myprogram data.tsv 5
```
In this example, the program expects data.tsv to have 5 columns. It will read each line of the file, skip lines starting with '#', and pass the entire row to myprogram for processing.

### Processing Specific Columns
To process specific columns in a TSV file named `data.tsv`, you can provide the column indices as a space-separated list in the columns_to_process argument. For example, to process columns 1, 3, and 4, run the following command:

```bash
$ ./ploop myprogram data.tsv 5 1 3 4
```
In this case, the program will extract the values from columns 1, 3, and 4 for each line in the TSV file and pass them to myprogram for processing.

### Error Handling
If an invalid column index is provided in the columns_to_process argument, the program will display an error message and exit with a non-zero exit code. For example, if you provide an out-of-range column index like 6 for a TSV file with only 5 columns, you will see an error message like the following:

```bash
$ ./ploop myprogram data.tsv 5 1 3 6
Error: Invalid column index: 6. Column index must be between 1 and 5.
```

Make sure to provide valid column indices within the range of the number of columns in your TSV file.

## Requirements
There are two external crates that are used in the code and must be included as dependencies in the `Cargo.toml` file in, order for the program to compile:

```
chrono: A crate for working with dates and times.
clap: A crate for parsing command-line arguments.
```

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

## License

These files released under the [MIT License](LICENSE).
