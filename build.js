#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const util = require('util');
const archiver = require_('archiver');
const {exec} = require('child_process');

const lStat = util.promisify(fs.lstat);
const mkdir = util.promisify(fs.mkdir);
const rmDir = util.promisify(fs.rmdir);
const access = util.promisify(fs.access);
const unlink = util.promisify(fs.unlink);
const readDir = util.promisify(fs.readdir);
const copyFile = util.promisify(fs.copyFile);

const isRelease = ~process.argv.indexOf('--release');
const assetsDir = path.resolve(__dirname, 'assets');
const boilerplate = path.resolve(__dirname, '.boilerplate');
const targetDir = path.resolve(__dirname, 'target');

(async () => {
    const command = `cargo build ${process.argv.slice(2).join(' ')}`;
    console.info('编译 rs 程序', command);
    await new Promise((resolve, reject) => exec(command, {
        windowsHide: true
    }, (err, stdOut, stdErr) => {
        if (err) {
            reject(err);
        } else if (stdErr) {
            console.error(stdErr);
            resolve(stdOut);
        } else {
            console.info(stdOut);
            resolve(stdOut);
        }
    }));
    await buildBin();
    await buildLib();
})();
async function buildLib(){
    const prefix = process.platform === 'darwin' ? 'lib' : '';
    const extName = process.platform === 'darwin' ? '.dylib' : '.dll';
    const dllFile = path.join(__dirname, 'target', isRelease ? 'release' : 'debug', `${prefix}scaffold_wizard${extName}`);
    const zipFile = path.join(targetDir, 'scaffold-wizard.setup-lib.zip');
    const setupDir = path.join(targetDir, 'setup-lib');
    const setupDllFile = path.join(setupDir, 'bin', `${prefix}scaffold_wizard${extName}`);
    await build(setupDir, zipFile, dllFile, setupDllFile);
}
async function buildBin(){
    const extName = process.platform === 'win32' ? '.exe' : '';
    const exeFile = path.join(__dirname, 'target', isRelease ? 'release' : 'debug', `scaffold-wizard${extName}`);
    const zipFile = path.join(targetDir, 'scaffold-wizard.setup-bin.zip');
    const setupDir = path.join(targetDir, 'setup-bin');
    const setupExeFile = path.join(setupDir, 'bin', `scaffold-wizard${extName}`);
    await build(setupDir, zipFile, exeFile, setupExeFile);
}
async function build(setupDir, zipFile, exeFile, setupExeFile){
    const setupAssetsDir = path.join(setupDir, 'assets');
    console.info('删除旧文件');
    if (await access(zipFile, fs.constants.F_OK).then(() => true, () => false)) {
        await unlink(zipFile);
    }
    if (await access(setupDir, fs.constants.F_OK).then(() => true, () => false)) {
        await removeDir(setupDir);
    }
    console.info('创建', setupDir, '目录');
    await mkdir(setupDir);
    console.info('复制依赖 dll、资源文件、配置文件');
    await copyDir(boilerplate, setupDir);
    await copyFile(exeFile, setupExeFile);
    await mkdir(setupAssetsDir);
    await copyDir(assetsDir, setupAssetsDir);
    await zip(setupDir, zipFile);
    console.info('结束');
}
async function zip(setupDir, zipFile){
    if (archiver) {
        console.info('生成压缩包');
        const dirs = await readDir(setupDir);
        const archive = archiver('zip', {
            zlib: {
                level: 9
            }
        });
        for (const dirName of dirs) {
            archive.directory(`${path.join(setupDir, dirName)}/`, dirName, {
                name: dirName
            });
        }
        await new Promise((resolve, reject) => {
            const output = fs.createWriteStream(zipFile);
            output.on('close', resolve);
            archive.on('warning', reject);
            archive.on('error', reject);
            archive.pipe(output);
            archive.finalize();
        });
    } else {
        console.warn('未能生成 zip 文件，请安装 npm i -g archiver');
    }
}
async function copyDir(source, target){
    if ((await lStat(source)).isDirectory()) {
        const items = await readDir(source);
        await Promise.all(items.map(async item => {
            const sourcePath = path.join(source, item);
            const targetPath = path.join(target, item);
            if ((await lStat(sourcePath)).isDirectory()) {
                await mkdir(targetPath);
                await copyDir(sourcePath, targetPath);
            } else {
                copyFile(sourcePath, targetPath);
            }
        }));
    }
}
async function removeDir(dir){
    if (await access(dir, fs.constants.F_OK).then(() => true, () => false)) {
        const items = await readDir(dir);
        if (items.length > 0) {
            await Promise.all(items.map(async item => {
                const delPath = path.join(dir, item);
                if ((await lStat(delPath)).isDirectory()) {
                    await removeDir(delPath);
                } else {
                    await unlink(delPath);
                }
            }));
            await removeDir(dir);
        } else {
            await rmDir(dir)
        }
    }
}
function require_(name){
    try {
        return require(name);
    } catch (err) {
        console.error('请【全局】安装', name);
        return null;
    }
}
