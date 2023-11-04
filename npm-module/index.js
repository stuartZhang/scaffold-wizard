const path = require('path');
const logger = require('debug');
const downloader = require('./download');
//
const {CACHE_DIR, download} = downloader;
exports.inquire = questions => download().then(injectZlib1).then(() => {
    if (process.platform === 'darwin') {
        return new Promise((resolve, reject) => {
            let isKilled = false;
            const child = require('child_process').fork(path.join(__dirname, 'engine.js'), {
                env: {
                    ...process.env,
                    DYLD_LIBRARY_PATH: `${CACHE_DIR}/lib:${process.env.DYLD_LIBRARY_PATH}`
                }
            });
            child.on('close', code => {
                isKilled = true;
                if (code === 0) {
                    resolve();
                } else {
                    reject(new Error('子进程执行失败'));
                }
            });
            child.on('error', err => {
                if (!isKilled) {
                    child.kill();
                }
                reject(err);
            });
            child.on('message', message => {
                if (!isKilled) {
                    child.kill();
                }
                if (message.type === 'success') {
                    resolve(message.data);
                } else {
                    reject(message.data || message);
                }
            });
            child.send({
                action: 'questions',
                data: questions
            });
        });
    }
    return require('./engine').inquire(questions);
});
if (process.platform === 'darwin') {
    exports.inquireAsync = exports.inquire;
} else {
    exports.inquireAsync = questions => download().then(injectZlib1).then(() => require('./engine').inquireAsync(questions));
}
function injectZlib1(){
    return new Promise((resolve, reject) => {
        const log = logger('scaffold-wizard:inject-dll');
        log('platform=', process.platform, 'cpu-arch=', process.arch);
        if (process.platform === 'win32' && process.arch === 'x64') {
            const zlib1 = path.join(CACHE_DIR, 'bin/zlib1.dll');
            log('zlib1=', zlib1, 'process-id=', process.pid);
            const injector = require('dll-inject');
            const isNodeRunning = injector.isProcessRunningPID(process.pid);
            if (isNodeRunning) {
                const success = injector.injectPID(process.pid, zlib1);
                if (success !== 0) {
                    reject(new Error(`${zlib1} 注入失败：${success}, ${process.pid}`));
                    return;
                }
            }
            log('注入', zlib1, '给进程', process.pid);
            setTimeout(resolve, 0);
        } else {
            log('不需要注入 zlib1');
            resolve();
        }
    });
}
