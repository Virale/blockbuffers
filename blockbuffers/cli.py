"""Generate code from serialized flatbuffers schema in bfbs format.

Usage:
  blockc [(--hash-table=<table>... --hash-mod=<mod>)] [-o <dir>] <bfbs>

Options:
  <bfbs>                    bfbs file which is generated using `flatc -b --schema <fbs>`
  -h --help                 Show this screen.
  -t --hash-table=<table>   Generate code to compute hashes for the specified tables.
  -m --hash-mod=<mod>       The name of the module which will contain the code to computing hashes.
  -o <dir>                  Output directory.
"""
from docopt import docopt


def parse_arguments(argv=None):
    return docopt(__doc__, argv)


def main():
    print(parse_arguments())
