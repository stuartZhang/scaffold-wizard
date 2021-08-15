const os = require('os');
const path = require('path');
const fs = require('fs-extra');
const chalk = require('chalk');
const logger = require('debug');
const download = require('download');
const cliProgress = require('cli-progress');
//
const DEFAULT_TAG = 'basic2';
const CACHE_DIR = path.join(os.tmpdir(), 'npm-scaffold-wizard', DEFAULT_TAG);
//
exports.CACHE_DIR = CACHE_DIR;
exports.DEFAULT_TAG = DEFAULT_TAG;
Reflect.defineProperty(exports, 'downloadUrl', {
    enumerable: true,
    configurable: false,
    get(){
        if (os.platform() === 'win32' && os.arch() === 'x64') {
            return `https://github.com/stuartZhang/scaffold-wizard/releases/download/${DEFAULT_TAG}/scaffold-wizard.setup-lib.zip`;
        }
        throw new Error(`没有与'${os.platform()}_${os.arch()}'操作系统相配的图形界面程序包`);
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
        return Promise.all([
            path.join(CACHE_DIR, 'bin/scaffold_wizard.dll'),
            path.join(CACHE_DIR, 'bin/zlib1.dll')
        ].map(dllPath => fs.exists(dllPath).then(exist => {
            if (exist) {
                return fs.stat(dllPath).then(stats => stats.isFile());
            }
            return exist;
        }))).then(results => results.every(result => result));
    }
    return false;
}
