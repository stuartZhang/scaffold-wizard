#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const util = require('util');
const archiver = require_('archiver');
const {exec} = require('child_process');
const {binFullName, dyLibFullName} = require('./npm-module/utils');

const lStat = util.promisify(fs.lstat);
const mkdir = util.promisify(fs.mkdir);
const rmDir = util.promisify(fs.rmdir);
const access = util.promisify(fs.access);
const unlink = util.promisify(fs.unlink);
const readDir = util.promisify(fs.readdir);
const copyFile = util.promisify(fs.copyFile);

const isRelease = ~process.argv.indexOf('--release');
const assetsDir = path.resolve(__dirname, 'assets');
const boilerplate = path.resolve(__dirname, path.join('.boilerplate', process.platform));
const targetDir = path.resolve(__dirname, 'target');
const targetTriple = `${process.platform}-${process.arch}`;

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
    // await collectDyLibDeps();
    await buildBin();
    await buildLib();
})();
async function buildLib(){
    const dllFile = path.join(__dirname, 'target', isRelease ? 'release' : 'debug', dyLibFullName);
    const zipFile = path.join(targetDir, `scaffold-wizard.setup-lib.${targetTriple}.zip`);
    const setupDir = path.join(targetDir, 'setup-lib');
    const setupDllFile = path.join(setupDir, 'bin', dyLibFullName);
    await build(setupDir, zipFile, dllFile, setupDllFile);
}
async function buildBin(){
    const exeFile = path.join(__dirname, 'target', isRelease ? 'release' : 'debug', binFullName);
    const zipFile = path.join(targetDir, `scaffold-wizard.setup-bin.${targetTriple}.zip`);
    const setupDir = path.join(targetDir, 'setup-bin');
    const setupExeFile = path.join(setupDir, 'bin', binFullName);
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
    if (await fileExists(boilerplate)) {
        console.info('复制依赖 dll、资源文件、配置文件');
        await copyDir(boilerplate, setupDir);
    }
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
    if (await fileExists(dir)) {
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
function fileExists(filePath){
    return access(filePath, fs.constants.F_OK).then(() => true, () => false);
}
function require_(name){
    try {
        return require(name);
    } catch (err) {
        console.error('请【全局】安装', name);
        return null;
    }
}
async function collectDyLibDeps(pkgName = 'gtk+3', handledPkgNames = []){
    if (process.platform !== 'darwin') {
        return;
    }
    const filePaths = await execute(`brew ls -v ${pkgName}`);
    const pairs = filePaths.split(/\n/).map(filePath => filePath.trim()).filter(filePath => {
        return !!filePath && !filePath.startsWith(`/usr/local/Cellar/${filePath}/`);
    }).filter(filePath => {
        const middleFragment = filePath.split(/\//).splice(6, 2);
        if (['bin', 'lib'].includes(middleFragment[0])) {
            return true;
        }
        if (middleFragment[0] === 'share') {
            if (['man', 'locale'].includes(middleFragment[1])) {
                return false;
            }
            return true;
        }
        return false;
    }).map(filePath => {
        return [filePath, path.join(__dirname, '.boilerplate/darwin', filePath.split(/\//).slice(6).join('/'))];
    });
    for (const [source, dest] of pairs) {
       await execute(`mkdir -p ${path.dirname(dest)}`);
       try {
            await execute(`cp -R -L -f ${source} ${dest}`);
       } catch {
            console.error(`sudo cp ${source} ${dest}`);
       }
    }
    // 寻找全部依赖项。
    let pkgNames = await execute(`brew deps --installed ${pkgName}`);
    pkgNames = pkgNames.split(/\n/).map(pkgName => pkgName.trim()).filter(pkgName => !!pkgName);
    for (const pkgName of pkgNames) {
        if (handledPkgNames.includes(pkgName)) {
            continue;
        }
        handledPkgNames.push(pkgName);
        console.log(pkgName);
        await collectDyLibDeps(pkgName, handledPkgNames);
    }
}
function execute(command){
    return new Promise((resolve, reject) => exec(command, {windowsHide: true}, (err, stdOut, stdErr) => {
        if (err) {
            reject(err);
        } else if (stdErr) {
            resolve(stdOut);
        } else {
            resolve(stdOut);
        }
    }));
}