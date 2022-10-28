#!/usr/bin/env python3

import argparse
import os
import shutil
import subprocess
import sys
import tempfile

def sh(script):
    subprocess.check_call(['bash', '-c', 'set -ex\n' + script])

def build():
    sh('cargo build --release --bin come_boy')

def run_and_time(tmp_dir, rom_path, ticks):
    cmd = [
        'target/release/come_boy',
        rom_path,
        '--renderer', 'null',
        '--unlock-cpu',
        '--perf-stats',
        '--run-until', str(ticks)
    ]
    time_output = os.path.join(tmp_dir, 'time.stderr')
    sh('/usr/bin/time ' + ' '.join(cmd) + f' 2> {time_output}')

    with open(time_output) as f:
        first_column = f.read().split(' ')[0]
        return float(first_column[:-len('user')])

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("rom_path")
    parser.add_argument("--ticks", type=int, required=True)
    parser.add_argument("--runs", type=int, required=True)
    opts = parser.parse_args()

    tmp_dir = tempfile.mkdtemp()

    build()

    cpu_time_total = 0
    for _ in range(opts.runs):
        cpu_time = run_and_time(tmp_dir, opts.rom_path, opts.ticks)
        print(f'one run used {cpu_time}s CPU time')
        cpu_time_total += cpu_time

    cpu_time_average = round(cpu_time_total / float(opts.runs), 2)
    print(f'average CPU time spent {cpu_time_average}s')

    shutil.rmtree(tmp_dir)

if __name__ == "__main__":
    main()
