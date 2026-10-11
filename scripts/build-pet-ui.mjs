#!/usr/bin/env node
// Build platform resources from the single source icon; keep generated copies out of Git.
import { mkdtempSync, mkdirSync, copyFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const native = join(root, 'apps/pet_desktop');
function run(command, args, cwd, stdio='inherit') {
  const result=spawnSync(command,args,{cwd,stdio,env:{...process.env,NO_COLOR:'true'}});
  if(result.error)throw result.error;
  if(result.status!==0)throw new Error(`${command} exited ${result.status}`);
}
const scratch=mkdtempSync(join(tmpdir(),'copilot-icons-'));
try {
  run('cargo',['tauri','icon',join(native,'icons/icon.png'),'--output',scratch],native,['ignore','ignore','inherit']);
  const generated=join(native,'generated-icons');mkdirSync(generated,{recursive:true});
  for(const name of ['icon.ico','icon.icns'])copyFileSync(join(scratch,name),join(generated,name));
} finally {rmSync(scratch,{recursive:true,force:true});}
run('trunk',['build','--release=true','--no-color'],join(root,'apps/pet_ui'));
