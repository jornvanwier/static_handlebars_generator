```
Static Handlebars Generator 0.1
Jorn van Wier <contact@jornvanwier.com>
Renders templates using supplied data for usage in static websites.

USAGE:
    static_handlebars_generator [FLAGS] [OPTIONS] <template_dir> <data_dir> <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Output information for every step.

OPTIONS:
    -o, --output_dir <output_dir>    The directory to place the finished file in. Will output to stdout otherwise.

ARGS:
    <template_dir>    The directory to take templates from.
    <data_dir>        The directory to take data (in json files) from.
    <file>            The file to render, other required files (e.g. partials) will be automatically included.
                      Should be without any extensions. The program will look for a template and a date file with
                      the approriate extensions in the template_dir and data_dir.
```
