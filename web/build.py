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

def replace(path, needle, replacement):
    with open(path) as f:
        contents = f.read()
        contents = contents.replace(needle, replacement)

    with open(path, 'w') as f:
        f.write(contents)

def put(path, contents):
    with open(path, 'w') as f:
        f.write(contents)
    print(f"+ cat <contents> > {path}")

def delete_contents(path):
    for p in glob.glob(path + '/*'):
        sh('rm -r {}'.format(p))

def install(source, dest_dir):
    for p in glob.glob(source):
        if os.path.isdir(p):
            sh(f'cp -r -v {p} {dest_dir}')
        else:
            sh(f'install {p} {dest_dir}')

def build(optimize):
    target_dir = 'target/wasm32-unknown-unknown/release'
    sh('cargo build --package come_boy_web --target wasm32-unknown-unknown --release --features aggressive-inline')
    sh(f'wasm-bindgen --target web {target_dir}/come_boy_web.wasm --out-dir {target_dir}')
    if optimize:
        sh(f'wasm-opt {target_dir}/come_boy_web_bg.wasm -o {target_dir}/come_boy_web_bg.opt.wasm -O3')
        sh(f'mv {target_dir}/come_boy_web_bg.opt.wasm {target_dir}/come_boy_web_bg.wasm')

MAIN_SRC = '''\
import * as wasm from "./come_boy_web.js";
import init from "./come_boy_web.js";

init();
'''

def deploy(deploy_path):
    delete_contents(deploy_path)

    install('target/wasm32-unknown-unknown/release/come_boy_web_bg.wasm', deploy_path)
    install('target/wasm32-unknown-unknown/release/come_boy_web.js', deploy_path)
    install('web/www/index.html', deploy_path)

    index_html = os.path.join(deploy_path, 'index.html')
    replace(index_html, '$REVISION', head_revision())
    replace(index_html, '$DATE', date())

    revision = os.path.join(deploy_path, 'revision')
    put(revision, head_revision())

    main_js = os.path.join(deploy_path, 'main.js')
    put(main_js, MAIN_SRC)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--optimize", action='store_true')
    parser.add_argument("deploy_path")
    opts = parser.parse_args()

    build(opts.optimize)
    deploy(opts.deploy_path)

if __name__ == "__main__":
    main()
