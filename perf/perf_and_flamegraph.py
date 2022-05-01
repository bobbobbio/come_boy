#!/usr/bin/env python3

import argparse
import os
import shutil
import subprocess
import sys
import tempfile

def sh(script):
    subprocess.check_call(['bash', '-c', 'set -ex\n' + script])

def cd(path):
    os.chdir(path)

def build():
    sh('cargo build --release --bin come_boy')

def save_state():
    pass

def perf_record(tmp_dir, rom_path, ticks):
    cmd = [
        'target/release/come_boy',
        rom_path,
        '--renderer', 'null',
        '--unlock-cpu',
        '--run-until', str(ticks)
    ]

    perf_output = os.path.join(tmp_dir, 'perf.data')
    cmd_str = ' '.join(cmd)
    sh(f'perf record --call-graph dwarf --output {perf_output} -- ' + cmd_str)

    return perf_output

FLAME_URL = 'https://raw.githubusercontent.com/brendangregg/FlameGraph/master'

def download_flame_graph_tools(dest):
    files = ['stackcollapse-perf.pl', 'flamegraph.pl']

    for f in files:
        local = os.path.join(dest, f)
        if not os.path.exists(local):
            sh(f'wget {FLAME_URL}/{f} -O {local}')
            sh(f'chmod +x {local}')

def render_flamegraph(tmp_dir, flametools, perf_data):
    collapsed_stacks = os.path.join(tmp_dir, 'perf.folded-data')
    sh(f'perf script --input {perf_data} |'
       f' {flametools}/stackcollapse-perf.pl > {collapsed_stacks}')
    svg_path = os.path.join(tmp_dir, 'perf.svg')
    sh(f'{flametools}/flamegraph.pl {collapsed_stacks} > {svg_path}')
    return svg_path

def proc_path(p):
    return os.path.join('/proc/sys/kernel', p)

def proc_value(path):
    with open(path) as f:
        return int(f.read())

def check_kernel_perf_permissions():
    perf_event = proc_value(proc_path('perf_event_paranoid'))
    kptr_restrict = proc_value(proc_path('kptr_restrict'))
    if perf_event > 0 or kptr_restrict != 0:
        set_cmd = ' && '.join((f'echo 0 > {f}' for f in (
            proc_path('perf_event_paranoid'),
            proc_path('kptr_restrict'),
        )))
        sys.stderr.write(
            'Permission Denied\n' +
            f'run `sudo bash -c "{set_cmd}"`\n'
        )
        return 1
    return 0

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("rom_path")
    parser.add_argument("svg_path")
    parser.add_argument("--ticks", type=int, required=True)
    opts = parser.parse_args()

    if check_kernel_perf_permissions():
        return 1

    flametools = 'target/.flamegraph'
    sh(f'mkdir -p {flametools}')
    download_flame_graph_tools(flametools)

    tmp_dir = tempfile.mkdtemp()

    build()
    perf_data = perf_record(tmp_dir, opts.rom_path, opts.ticks)

    svg_path = render_flamegraph(tmp_dir, flametools, perf_data)
    sh(f'mv {svg_path} {opts.svg_path}')

    shutil.rmtree(tmp_dir)

if __name__ == "__main__":
    main()
