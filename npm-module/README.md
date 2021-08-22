# scaffold-wizard

该模块是对[Rust + GNOME.GTK3 图形界面版 inquirer](https://github.com/stuartZhang/scaffold-wizard/)的`FFI`封装，使其能够在常规`js`代码内被轻易使用。

## 平台限制

鉴于`Rust + GNOME.GTK3`图形界面版`inquirer`动态链接库还仅只针对`win32_x64`操作系统提供了预编译二进制程序包，`scaffold-wizard`也在`package.json`中严格限制了安装平台的`os`类型与`cpu`架构分别为`win32`与`x64`。所以，一般的`64`位`windows 7`与`windows 10`操作系统是能够正常地安装与运行此模块的。

> 今后，`Rust + GNOME.GTK3`图形界面版`inquirer`会陆续发布针对更多操作系统平台的预编译二进制程序包的。

可以将该模块作为`optional dependencies`依赖来安装。于是，在遇到非`windows x64`环境时，借助[safe-require](https://www.npmjs.com/package/safe-require)将其降级为命令行[inquirer](https://www.npmjs.com/package/inquirer)。

## 工作原理

在如下两个时间点：

* `npm`模块安装过程中
* 每次该模块的`js api`被调用之前

程序都会尝试：

1. 检查系统临时目录下是否存在`npm-scaffold-wizard/basic2`文件夹。其中，`basic2`是`Rust + GNOME.GTK3`图形界面版`inquirer`的`tag`号。随着`Rust + GNOME.GTK3`图形界面版`inquirer`的升级，此`tag`号以后可能会发生改变。
2. 检查两个`dll`文件（`npm-scaffold-wizard/basic2/bin/scaffold_wizard.dll`与`npm-scaffold-wizard/basic2/bin/zlib1.dll`）是否存在。

若上面两个检查项之一返回失败的结果，那么就从`github`的`release`频道（具体地讲，如`https://github.com/stuartZhang/scaffold-wizard/releases/download/basic2/scaffold-wizard.setup-lib.zip`）下载被预编译的二进制程序包至`npm-scaffold-wizard/basic2`文件夹。

> 若哪天不幸赶上`github`被墙，可以考虑设置`http_proxy`与`https_proxy`环境变量，来“科学上网”下载【预编译的二进制程序】。

另外，该模块是对`Rust + GNOME.GTK3`图形界面版`inquirer`模块的·薄·封装。所有输入参数与返回结果都是经由[node-ffi](https://www.npmjs.com/package/ffi)被透传的（甚至包括·回调函数·），未做加工处理。

## 安装与使用

在`package.json`文件里，添加

```json
{
    "optionalDependencies": {
        "scaffold-wizard": "^1.0.0"
    }
}
```

在`js`业务代码里，

```javascript
const inquirer = safeRequire('inquirer');
const guiInquirer = require('safe-require')('scaffold-wizard');
const config = {...}; // 问卷配置对象
let answers;
if (guiInquirer) { // 在 windows x64 环境，直接弹出 native gui。
    answers = guiInquirer.inquire(config);
} else { // 其它环境，降级至命令行交互。
    answers = inquirer.prompt(Object.values(config));
}
```

## 接口描述

`Rust + GNOME.GTK3`图形界面版`inquirer`经由`FFI`仅对外开放了两个接口

```typescript
import inquirer from 'inquirer';
/**
 * 和原版 inquirer 的微小差别，这里的问题清单不是 Array<inquirer.DistinctQuestion<T>>，
 * 而是一个 Object<string, inquirer.DistinctQuestion<T>>。其中，键是一个问题
 * 的唯一标识符 identifier（等同于【问题配置对象】里的 name 属性）；值就是该问题的配置对象。
 *
 * 另一方面，问题的提问次序与 Questions 配置对象内【键-值】对的词法次序一致。
 * @export
 * @interface Questions
 * @template T
 */
export interface Questions<T> {
    [key: string]: inquirer.DistinctQuestion<T>
}
/**
 * 在经由图形界面收集问卷答案时，阻塞 libuv 的事件循环。
 * @export
 * @param {Questions} questions
 * @returns {Promise<inquirer.Answers>}
 */
export function inquire(questions: Questions): Promise<inquirer.Answers>;
/**
 * 在经由图形界面收集问卷答案时，不阻塞 libuv 的事件循环。所以，node 还能接着响应
 * 来自其它事件源的请求。
 * @export
 * @param {Questions} questions
 * @returns {Promise<inquirer.Answers>}
 */
export function inquireAsync(questions: Questions): Promise<inquirer.Answers>;
```

所以，一个`json`格式问题清单的例子如下。这是一个典型的【`vue`前端-脚手架】图形界面安装向导的问卷配置数据结构。

```json
{
    "subprojects": { // 这个问题唯一标识字符串。相当于主键 ID。
        "type": "checkbox",
        "message": "请选择 工程类型",
        "required": true,
        "choices": [{
            "name": "PC浏览器-管理界面",
            "value": "admin",
            "short": "中后台",
            "checked": false
        }, {
            "name": "本地 H5 插件",
            "value": "app",
            "short": "移动插件",
            "checked": false
        }, {
            "name": "组件/模块/微前端应用",
            "value": "component",
            "short": "组件/模块/微前端",
            "checked": false,
            "mutex": true
        }, {
            "name": "RUST 语言 WEB 字节码 NPM 模块",
            "value": "wasm",
            "short": "RUST + WASM + NPM",
            "checked": false,
            "mutex": true
        }, {
            "name": "RUST 语言原生 GUI 应用",
            "value": "rust_gui",
            "short": "RUST + GTK3 APP",
            "checked": false,
            "mutex": true
        }]
    },
    "adminPort": {
        "when": "subprojects.admin", // 条件表达式，当前问题是出现在交互流程中（true），还是被跳过（false）。
        "type": "input", // 文本输入框
        "subType": "port", // 端口数字输入框
        "message": "请输入【管理端】webpack dev server 监听端口号", // 题面
        "required": true, // 是否必填
        "default": 9000 // 默认值
    },
    "appPort": {
        "when": "subprojects.app",
        "type": "input",
        "subType": "port",
        "message": "请输入 移动端 webpack dev server 监听端口号",
        "required": true,
        "default": 9010
    },
    "componentPort": {
        "when": "subprojects.component",
        "type": "input",
        "subType": "port",
        "message": "请输入 组件/模块开发 webpack dev server 监听端口号",
        "required": true,
        "default": 9015
    },
    "adminUiLib": {
        "when": "subprojects.admin",
        "type": "list",
        "message": "请选择“管理端” UI 组件库",
        "choices": [{
            "name": "不使用UI组件库",
            "value": "none",
            "short": "无"
        }, {
            "name": "iView",
            "value": "iView",
            "short": "iView"
        }, {
            "name": "Element UI",
            "value": "elementUI",
            "short": "Element"
        }]
    },
    "appUiLib": {
        "when": "subprojects.app",
        "type": "list",
        "message": "请选择“移动端” UI 组件库",
        "choices": [{
            "name": "不使用UI组件库",
            "value": "none",
            "short": "无"
        }, {
            "name": "Vant",
            "value": "vant",
            "short": "vant"
        }]
    },
    "compWhichEnd": {
        "when": "subprojects.component",
        "type": "list",
        "message": "请选择“组件的目标”运行环境",
        "default": 0,
        "choices": [{
            "name": "桌面浏览器",
            "value": "pcBrowser",
            "short": "桌面"
        }, {
            "name": "移动 WebView",
            "value": "mobileBrowser",
            "short": "移动"
        }]
    },
    "compUiLib": {
        "when": "subprojects.component",
        "type": "list",
        "message": "请选择 基于哪款【UI 组件库】做二次开发实现组件",
        "choices": [{
            "name": "不使用UI组件库",
            "value": "none",
            "short": "无"
        }, {
            "when": "compWhichEnd == 'pcBrowser'",
            "name": "Element UI",
            "value": "elementUI",
            "short": "Element"
        }, {
            "when": "compWhichEnd == 'mobileBrowser'",
            "name": "Vant",
            "value": "vant",
            "short": "vant"
        }]
    },
    "favicon": {
        "when": "subprojects.admin || subprojects.app",
        "type": "confirm",
        "message": "针对【管理端】与【移动端】，是否自己管理网页的 favicon 图标？",
        "default": false
    },
    "adminMultiEntries": {
        "when": "subprojects.admin",
        "type": "list",
        "message": "【管理端】是几页应用程序？",
        "default": 0,
        "choices": [{
            "name": "单页",
            "value": 1,
            "short": "单"
        }, {
            "name": "双页",
            "value": 2,
            "short": "双"
        }]
    },
    "appMultiEntries": {
        "when": "subprojects.app",
        "type": "list",
        "message": "【移动端】是几页应用程序？",
        "default": 0,
        "choices": [{
            "name": "单页",
            "value": 1,
            "short": "单"
        }, {
            "name": "双页",
            "value": 2,
            "short": "双"
        }]
    },
    "flavor": {
        "when": "subprojects.wasm == false && subprojects.rust_gui == false",
        "type": "confirm",
        "message": "是否支持 Flavor 选择器？",
        "default": false
    },
    "adminAssetsSubDirectory": {
        "when": "subprojects.admin",
        "type": "list",
        "message": "请选择-管理端-资源文件目录名",
        "default": 0,
        "choices": [{
            "name": "bundle/admin",
            "value": "bundle/admin",
            "short": "bundle/admin"
        }, {
            "name": "static",
            "value": "static",
            "short": "static"
        }]
    },
    "appAssetsSubDirectory": {
        "when": "subprojects.app",
        "type": "list",
        "message": "请选择-移动端-资源文件目录名",
        "default": 0,
        "choices": [{
            "name": "bundle/app",
            "value": "bundle/app",
            "short": "bundle/app"
        }, {
            "name": "static",
            "value": "static",
            "short": "static"
        }]
    },
    "appWebWorker": {
        "when": "subprojects.app",
        "type": "confirm",
        "message": "是否预置线程化 web worker 加速？",
        "default": false
    },
    "adminI18n": {
        "when": "subprojects.admin",
        "type": "confirm",
        "message": "是否预置 Vue 国际化？",
        "default": false
    },
    "appI18n": {
        "when": "subprojects.app",
        "type": "confirm",
        "message": "是否预置 Vue 国际化？",
        "default": false
    },
    "imgCompress": {
        "when": "subprojects.wasm == false && subprojects.rust_gui == false",
        "type": "confirm",
        "message": "是否预压缩各类图片？",
        "default": false
    },
    "appPrerender": {
        "when": "subprojects.app",
        "type": "confirm",
        "message": "【是/否】启动首屏预渲染模式？",
        "default": false
    },
    "cssModules": {
        "when": "subprojects.admin || subprojects.app",
        "type": "confirm",
        "message": "【是/否】启用 CSS 模块？",
        "default": false
    },
    "adminType": {
        "when": "subprojects.admin",
        "type": "list",
        "message": "请选择 工程子类型",
        "default": 0,
        "choices": [{
            "name": "单体应用",
            "value": "standalone",
            "short": "常规"
        }, {
            "name": "微前端宿主",
            "value": "microFrontEnd",
            "short": "微前端"
        }]
    },
    "appType": {
        "when": "subprojects.app",
        "type": "list",
        "message": "请选择 工程子类型",
        "default": 0,
        "choices": [{
            "name": "单体应用",
            "value": "standalone",
            "short": "常规"
        }, {
            "name": "微前端宿主",
            "value": "microFrontEnd",
            "short": "微前端"
        }]
    },
    "compType": {
        "when": "subprojects.component",
        "type": "list",
        "message": "请选择 工程子类型",
        "default": 0,
        "choices": [{
            "name": "组件/模块",
            "value": "module",
            "short": "常规"
        }, {
            "name": "微前端应用",
            "value": "microFrontEnd",
            "short": "微前端"
        }]
    },
    "adminPx2remTactic": {
        "when": "subprojects.admin",
        "type": "list",
        "message": "px2rem 自适应布局策略",
        "default": 1,
        "choices": [{
            "name": "放弃（没有设计稿，凭感觉度量）",
            "value": "none",
            "short": "放弃"
        }, {
            "name": "采用且以·设计稿宽度·为基准（对齐设计稿，制作常规网页）",
            "value": "width",
            "short": "常规网页制作"
        }, {
            "name": "采用且以·设计稿对角线长度·为基准（对齐设计稿，制作·限高·iframe）",
            "value": "diagonal",
            "short": "限高iframe制作"
        }]
    },
    "installDepsImmediate": {
        "when": "!subprojects.rust_gui",
        "type": "confirm",
        "message": "【是/否】是否立即给工程原型安装依赖？",
        "default": true
    },
    "name": {
        "type": "string",
        "subType": "packageName",
        "message": "工程名",
        "required": true,
        "default": "project_name"
    },
    "author": {
        "type": "string",
        "message": "作者名",
        "required": true,
        "default": "author_name"
    }
}
```

## 与[inquirer](https://www.npmjs.com/package/inquirer)的细微差别

1. 问卷配置对象是`Object<inquirer.KeyUnion<T>, inquirer.DistinctQuestion<T>>`，而不是`Array<inquirer.DistinctQuestion<T>>`。
   1. 这一点没有顺从于·原著·是为了更容易地与公司现成的【前端-脚手架】安装向导对接。所以，从核心层[Rust + GNOME.GTK3 图形界面版 inquirer](https://github.com/stuartZhang/scaffold-wizard/)就这么处理的数据结构。
2. 单个问题【配置对象】内暂时缺少【回调函数】支持 --- 这类·动态出现的·`FFI`接口，我还不会做。但作为补偿，我在如下几处添加了新配置属性：
   1. 给`"type": "input"`类型（即，文本输入框）添加了`"subType": "port"`子类。其专门收集【数字类型】，取值范围在`1000 ~ 99999`的端口号。样板配置如下：

       ```json
       {
           "appPort": { // 这个问题唯一标识字符串。相当于主键 ID。
               "when": "subprojects.app", // 条件表达式，当前问题是出现在交互流程中（true），还是被跳过（false）。
               "type": "input", // 文本输入框
               "subType": "port", // 端口数字输入框
               "message": "请输入 移动端 webpack dev server 监听端口号", // 题面
               "required": true, // 是否必填
               "default": 9010 // 默认值
           },
       }
       ```

   2. 给`"type": "list"`类型（即，单选题）的每一个单选项添加了`when`（布尔）表达式。从而，根据上下文内容，动态地决定当前单选项是否被显示出来。样板配置如下：

       ```json
       {
           "compUiLib": { // 这个问题唯一标识字符串。相当于主键 ID。
               "when": "subprojects.component", // 条件表达式，当前问题是出现在交互流程中（true），还是被跳过（false）。
               "type": "list", // 单选题
               "message": "请选择 基于哪款【UI 组件库】做二次开发实现组件", // 题面 - 标题
               "choices": [{ // 题面 - 单选项1
                   "name": "不使用UI组件库", // 【显示用】完整名
                   "short": "无", // 【显示用】简称名 - 暂时尚未使用
                   "value": "none" // 【程序引用】此选项的唯一标识字符串
               }, { // 题面 - 单选项2
                   "when": "compWhichEnd == 'pcBrowser'", // 决定此选项是否出现的`when`表达式
                   "name": "Element UI", // 【显示用】完整名
                   "short": "Element", // 【显示用】简称名 - 暂时尚未使用
                   "value": "elementUI" // 【程序引用】此选项的唯一标识字符串
               }, { // 题面 - 单选项3
                   "when": "compWhichEnd == 'mobileBrowser'", // 决定此选项是否出现的`when`表达式
                   "name": "Vant", // 【显示用】完整名
                   "short": "vant", // 【显示用】简称名 - 暂时尚未使用
                   "value": "vant" // 【程序引用】此选项的唯一标识字符串
               }]
           },
       }
       ```

   3. 给`"type": "checkbox"`类型（即，多选题）的每一个多选项添加了`mutex: boolean`属性。`"mutex": true`表示该选项具有排它性。若其被选中，则该选项只能被单选。样板配置如下：

       ```json
       {
           "subprojects": { // 这个问题唯一标识字符串。相当于主键 ID。
               "type": "checkbox", // 多选题
               "message": "请选择 工程类型", // 题面 - 标题
               "required": true, // 是否必填
               "choices": [{ // 题面 - 多选项1
                   "name": "PC浏览器-管理界面", // 【显示用】完整名
                   "short": "中后台", // 【显示用】简称名 - 暂时尚未使用
                   "value": "admin", // 【程序引用】此选项的唯一标识字符串。比如，subprojects.admin
                   "checked": false // 初始选中状态
               }, { // 题面 - 多选项2
                   "name": "本地 H5 插件", // 【显示用】完整名
                   "short": "移动插件", // 【显示用】简称名 - 暂时尚未使用
                   "value": "app", // 【程序引用】此选项的唯一标识字符串。比如，subprojects.app
                   "checked": false // 初始选中状态
               }, { // 题面 - 多选项3
                   "name": "组件/模块/微前端应用", // 【显示用】完整名
                   "short": "组件/模块/微前端", // 【显示用】简称名 - 暂时尚未使用
                   "value": "component", // 【程序引用】此选项的唯一标识字符串。比如，subprojects.component
                   "checked": false, // 初始选中状态
                   "mutex": true // 是否为单选
               }, { // 题面 - 多选项3
                   "name": "RUST 语言 WEB 字节码 NPM 模块", // 【显示用】完整名
                   "short": "RUST + WASM + NPM", // 【显示用】简称名 - 暂时尚未使用
                   "value": "wasm", // 【程序引用】此选项的唯一标识字符串。比如，subprojects.wasm
                   "checked": false, // 初始选中状态
                   "mutex": true // 是否为单选
               }, { // 题面 - 多选项4
                   "name": "RUST 语言原生 GUI 应用", // 【显示用】完整名
                   "short": "RUST + GTK3 APP", // 【显示用】简称名 - 暂时尚未使用
                   "value": "rust_gui", // 【程序引用】此选项的唯一标识字符串。比如，subprojects.rust_gui
                   "checked": false, // 初始选中状态
                   "mutex": true // 是否为单选
               }]
           },
       }
       ```

### 关于核心模块更多内容也请阅读[scaffold-wizard](https://github.com/stuartZhang/scaffold-wizard/#scaffold-wizard)
