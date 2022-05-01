#!/usr/bin/env python3

import argparse
import glob
import os
import subprocess
import sys

def date():
    env = { 'TZ': 'America/Los_Angeles' }
    return subprocess.check_output(['date'], env=env).decode().strip()

def head_revision():
    return subprocess.check_output(['git', 'rev-parse', 'HEAD']).decode().strip()

def sh(script):
    subprocess.check_call(['bash', '-c', 'set -ex\n' + script])

def cd(path):
    os.chdir(path)

def replace(path, needle, replacement):
    with open(path) as f:
        contents = f.read()
        contents = contents.replace(needle, replacement)

    with open(path, 'w') as f:
        f.write(contents)

def delete_contents(path):
    for p in glob.glob(path + '/*'):
        sh('rm -r {}'.format(p))

def install(source, dest_dir):
    for p in glob.glob(source):
        if os.path.isdir(p):
            subprocess.check_call(['cp', '-r', '-v', p, dest_dir])
        else:
            subprocess.check_call(['install', p, dest_dir])

def ensure_rust_updated():
    sh('rustup update nightly')
    sh('rustup default nightly')

def build():
    cd('web')
    sh('wasm-pack build --target web --release')
    cd('www')
    sh('npm install')
    cd('..')

def deploy(deploy_path):
    delete_contents(deploy_path)

    install('pkg/*', deploy_path)
    install('www/*.html', deploy_path)
    install('www/*.js', deploy_path)
    install('www/src', deploy_path)

    index_html = os.path.join(deploy_path, 'index.html')
    replace(index_html, '$REVISION', head_revision())
    replace(index_html, '$DATE', date())

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("deploy_path")
    opts = parser.parse_args()

    ensure_rust_updated()
    build()
    deploy(opts.deploy_path)

if __name__ == "__main__":
    main()
