#!/usr/bin/env python3
import os,sys
compiler=sys.argv[1];args=sys.argv[2:]
if '--crate-name' in args and args[args.index('--crate-name')+1]=='tauri_macros':
    args += ['-C','link-arg=-fuse-ld=bfd']
os.execv(compiler,[compiler,*args])
