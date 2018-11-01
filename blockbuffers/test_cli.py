from unittest import TestCase
from blockbuffers import cli
from docopt import DocoptExit


class TestCli(TestCase):
    def test_parse_arguments(self):
        with self.assertRaises(DocoptExit):
            cli.parse_arguments([])

        args = cli.parse_arguments(['test.bfbs'])
        self.assertEqual('test.bfbs', args['<bfbs>'])

        with self.assertRaises(DocoptExit):
            cli.parse_arguments(['--hash-table=Bbs.Transaction', 'test.bfbs'])
        with self.assertRaises(DocoptExit):
            cli.parse_arguments(['--hash-mod=v1', 'test.bfbs'])

        args = cli.parse_arguments(['--hash-table=Bbs.Transaction', '--hash-mod=v1', 'test.bfbs'])
        self.assertEqual(['Bbs.Transaction'], args['--hash-table'])
        self.assertEqual('v1', args['--hash-mod'])
        self.assertEqual('test.bfbs', args['<bfbs>'])

        args = cli.parse_arguments(['--hash-table=Bbs.Transaction', '-t', 'Bbs.CellOutput', '--hash-mod=v1', 'test.bfbs'])
        self.assertEqual(['Bbs.Transaction', 'Bbs.CellOutput'], args['--hash-table'])
        self.assertEqual('v1', args['--hash-mod'])
        self.assertEqual('test.bfbs', args['<bfbs>'])

        args = cli.parse_arguments(['-o', 'out', 'test.bfbs'])
        self.assertEqual('out', args['-o'])
        self.assertEqual('test.bfbs', args['<bfbs>'])
