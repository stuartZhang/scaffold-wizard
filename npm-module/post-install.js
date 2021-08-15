const {CACHE_DIR, download} = require('./download');

download().then(() => {
    console.log(' > 保存图形界面程序包至', CACHE_DIR);
}, err => {
    console.error(' > 图形界面程序包下载失败', err);
});
