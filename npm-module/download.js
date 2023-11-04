const path = require('path');
const fs = require('fs-extra');
const chalk = require('chalk');
const logger = require('debug');
const download = require('download');
const cliProgress = require('cli-progress');
const {dyLibFullName: LIB_FULL_NAME, defaultTag: DEFAULT_TAG, targetTriple} = require('./utils');
//
const CACHE_DIR = path.join(tmpDir(), 'npm-scaffold-wizard', DEFAULT_TAG);
//
exports.CACHE_DIR = CACHE_DIR;
exports.DEFAULT_TAG = DEFAULT_TAG;
Reflect.defineProperty(exports, 'downloadUrl', {
    enumerable: true,
    configurable: false,
    get(){
        if (process.arch === 'x64') {
            if (process.platform === 'win32') {
                return `https://github.com/stuartZhang/scaffold-wizard/releases/download/${DEFAULT_TAG}/scaffold-wizard.setup-lib.zip`;
            }
            if (process.platform === 'darwin') {
                return `https://github.com/stuartZhang/scaffold-wizard/releases/download/${DEFAULT_TAG}/scaffold-wizard.setup-lib.${targetTriple}.zip`;
            }
        }
        throw new Error(`没有与'${process.platform}_${process.arch}'操作系统相配的图形界面程序包`);
    }
});
exports.download = async function(){
    const log = logger('scaffold-wizard:download');
    log('DOWNLOAD_URL=', exports.downloadUrl);
    log('CACHE_DIR=', CACHE_DIR);
    if (await checkCacheDir()) {
        console.log(' > 图形界面程序包已缓存');
        return;
    }
    console.log(' > 下载与解压缩图形界面程序包');
    await fs.remove(CACHE_DIR);
    await fs.ensureDir(CACHE_DIR);
    const waitUtil = download(exports.downloadUrl, CACHE_DIR, {
        extract: true,
        retries: 4,
        timeout: 1000 * 120
    }).on('downloadProgress', event => {
        progress.update(Math.floor(event.percent * 100), {
            status: event.percent >= 1 ? '解压缩' : '下载',
            received: `${Math.floor(event.transferred / 1024 / 1024)}MB`,
            totalSize: `${Math.floor(event.total / 1024 / 1024)}MB`
        });
    });
    const progress = new cliProgress.SingleBar({
        format: ` > ${chalk.greenBright('{bar}')} {percentage}% | ${chalk.bold.cyan('{status}')} | {received} / {totalSize}`
    }, cliProgress.Presets.shades_classic);
    progress.start(110, 0, {
        status: '未动',
        received: 'N/A',
        totalSize: 'N/A'
    });
    await waitUtil.then(data => {
        progress.setTotal(100);
        progress.update({status: '完成'});
        progress.stop();
        return data;
    }, err => {
        progress.update({status: '失败'});
        progress.stop();
        return Promise.reject(err);
    });
};
async function checkCacheDir(){
    if (await fs.exists(CACHE_DIR) && (await fs.stat(CACHE_DIR)).isDirectory()) {
        const filePaths = [path.join(CACHE_DIR, 'bin', LIB_FULL_NAME)];
        if (process.platform === 'win32') {
            filePaths.push(path.join(CACHE_DIR, 'bin/zlib1.dll'));
        }
        return Promise.all(filePaths.map(dllPath => fs.exists(dllPath).then(exist => {
            if (exist) {
                return fs.stat(dllPath).then(stats => stats.isFile());
            }
            return exist;
        }))).then(results => results.every(result => result));
    }
    return false;
}
function tmpDir(){
    if (process.env.TMPDIR) {
        return process.env.TMPDIR;
    }
    const os = require('os');
    return os.tmpdir();
}
