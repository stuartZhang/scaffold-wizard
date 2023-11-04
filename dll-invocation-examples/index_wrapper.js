#!/usr/bin/env node
const childProcess = require('child_process');
const path = require('path');
const index = childProcess.spawn(process.argv[0], [path.join(__dirname, 'index.js'), ...process.argv.slice(2)], {
    env: {
        DYLD_LIBRARY_PATH: `../target/setup-lib/lib:${process.env.DYLD_LIBRARY_PATH}`
    }
});
process.stdin.pipe(index.stdin);
index.stdout.pipe(process.stdout);
index.stderr.pipe(process.stderr);
index.on('close', code => process.exit(code));
