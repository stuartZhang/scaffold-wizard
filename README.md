# scaffold-wizard

这是一款加持了【图形用户界面】的[npm - inquirer](https://www.npmjs.com/package/inquirer)（名曰：**问卷**）。即，根据【问卷】配置文件，以人-机交互的形式，收集终端用户的【回答结果】。这里提到的【问卷配置】与【回答结果】都是`*.json`格式的字符串（或文件）。

【问卷】既能够作为`.exe`文件被双击运行，也支持作为`.dll`文件被链接和调用-间接运行。

* 前者的输入与输出都是`.json`文件。
* 后者对外开放了两个`C ABI`以备调用。
  * 同步接口：`char* inquire(char* questions, char* bin_dir, char* log4rs_file)`
    * 【问卷配置】以`json`字符串的形式从第一个形参`questions`传入。
    * 【回答结果】以`json`字符串的形式从函数返回值传出。
  * `nodejs`风格的异步接口：`void inquireAsync(char* questions, char* bin_dir, char* log4rs_file, void (*callback)(char* error, char* answers))`
    * 【问卷配置】以`json`字符串的形式从第一个形参`questions`传入。
    * 【回答结果】通过最后一个【回调函数】的第二个实参输入形参，以`json`字符串的形式异步地传出。

在函数调用期间，会有`gnome`图形界面被弹出和提示用户输入问题答案。

## 制作这款工具的动机

我最近花了两个月的业余时间制作【问卷】这款工具的直接冲动来源于：将公司【前端-脚手架安装向导】从·命令行交互·升级为·图形界面互动·的构想。其路线图大约包括：

* 首先，让整个人-机交互过程更具有表现力；
* 其次，最好能将【安装向导】改造成为一个“原生`GUI`平台”，从而在未来添加更多辅助功能。
* 最终，成为公司技术工具链中重要的一环 --- 目标远大，征程漫长。

后来，我越做这款工具，越是觉得它的·通用性·还是比较高的。其使用场景不应仅只局限于【脚手架-安装过程】的现场配置收集。相反，任何含有【意见咨询】类功能的使用场景都可以考虑使用这款（或这类）工具。而，工具链的后续处理环节，再根据被收集的反馈结果，做定制化的“裁剪”。比如，“裁剪”脚手架内置的工程原型，使其更符合项目要求。

于是，我将这款工具从“脚手架-安装向导”更名为“问卷”。同时，它“下一步”再“下一步”的使用风格真心地相像于传统的`windows`应用软件的【安装向导】。【情怀】--- 在我认知体系中的任何软件安装都应该是“下一步”再“下一步”...最后“完成”；并且，其步骤越多，越有仪式感。

另一方面，在【`rust`桌面应用】方向投入更多业余精力也符合我个人对掌握`rust`技术栈的成长规划。即，

* 从`rust + wasm`入门。作为入门，这个“接入端”算是门槛比较低的了。
* 在`rust`桌面编程领域进阶。毕竟，`wasm`是一个严重受限的技术平台，许多`rust`高级语言特性，还有`rust`生态一多半的`crate`都没有用武之地。这严重地制约了我对`rust`技术栈的想像力与领悟层次。而转向`rust Iron`则很不明智。因为，
  * 就诸多后端解决方案而言，`rust`相对于`go`并没有绝对优势，生存空间极为狭小。同时，`rust`还得受着来自`java`, `ruby`, `php`, `python`的冲击。
  * 愣头青地和既得利益【团体】正面抢生存空间不利于团队的团结，我的领导也不会对我满意的。
  * 我掌握新技术的初衷是**提高个人岗位竞争力，不是找挨虐的**。
* 最后我的愿景是：在`IoT`嵌入式设备上“开花结果”。这对`rust`技术栈本身来说真不是问题。它已经一次又一次地证明其实力。愿景的实现主要还是看我对`rust`的掌握能够达到什么水平。

综上所述，实践`rust`的务实路径：`wasm` -> `Native GUI App` -> `IoT`嵌入式编程。使用`rust`做一些`GC`类语言想做，而做不好的事。

即便作为是一名懒惰的程序员，我也得掌握两个计算机语言

* `GC`类精通一门（一般说是“高级计算机语言”）
* 非`GC`类掌握一门（通常认为是“系统计算机语言”）

前者中佼佼者䊨在：“铺得面广`+`无处不在”，解决“温饱”问题；后者中“剩者”的立足点是：“足够地快`+`内存安全”，解决“小康”问题。**我要是能达到这个目标，那可真是：“中年危机远离我”。**

## 技术

简单地讲，`rust`写业务逻辑 + `gtk`组件库画界面。

### 依赖说明

* [clap](https://crates.io/crates/clap)
  * 解析命令行参数`input-file`，`output-file`，`log4rs-file`
  * 用法还算是高级，给`clap`写`yaml`配置文件，而不是在代码里攒【解析树】。
* [eval](https://crates.io/crates/eval)
  * 在运行时，根据上下文，求值【问卷配置】中`when`表达式。“给表达式求值”的功能真像`javascript`里的`eval`函数，但没那么强大。**我也绝不想在这个小工具里集成一个`JavascriptCore`引擎。实在太重了**
  * `when`表达式的求值结果决定了一个【问题】是否出现在图形界面的交互流程内。
* [log](https://crates.io/crates/log)与[log4rs](https://crates.io/crates/log4rs)
  * 日志记录
  * 大家对`log4**`家族里的其他成员一定很熟悉。比如，`log4j`与`log4js`。
* [quick-xml](https://crates.io/crates/quick-xml)
  * 解析`SGML`格式的`Glade`布局文件。将布局文件内，对外部资源（主要是图片）的相对引用地址都改成运行时计算得出的绝对路径。这样，无论你以何种方式启动`.exe`文件，被引用的外部文件都能够被正确地找到。
* [serde_json](https://crates.io/crates/serde_json)
  * 解析与输出`JSON`格式的【问卷配置】输入内容与【回答结果】输出内容。
* [gdk-pixbuf](https://crates.io/crates/gdk-pixbuf), [gio](https://crates.io/crates/gio), [glib](https://crates.io/crates/glib), [gtk](https://crates.io/crates/gtk)
  * 这些都是`Gnome`.`gtk3`的`rust binding`。其功能可类似于`C`里的【头文件】。

> 毕竟，【问卷】功能单一，所以用到的第三方依赖项不多。此外，
>
> * 在类`Linux`操作系统上，需要`Gnome`的`GtK`版本`>= 3.24`。
> * 在`windows`操作系统上，绿色安装包需要自带`gtk`动态链接库与资源文件的“家什儿”。

### 开发环境搭建

不熟悉`rust + gtk + win32`技术栈的小伙伴儿请移步我的另一篇技术分享：[为 Rust 原生 gui 编程，搭建 win32 开发环境](https://rustcc.cn/article?id=30291979-61e0-422d-9084-37d7d9eea2a1)。

#### `rustup`工具链版本

鉴于之前使用`rust + wasm`完成【网络加密通讯】功能的踩坑经验，我这次显示地将`package`绑定了适用的`rustup`版本`nightly-2021-03-25-x86_64-pc-windows-gnu`。若你的本地`rustup`安装版本与之不匹配，请根据编译的报错信息，`rustup install ***`正确的`rustup toolchain`版本。就开发环境而言，对非`windows`用户不友好了，实在对不住。

### 构建

#### `cargo build`或`cargo build --release`

输出两个关键结果

* `bin`的`target\debug\scaffold-wizard.exe` --- 可执行文件
* `lib`的`target\debug\scaffold_wizard.dll` --- `C`动态链接库`cdylib`。
  * 注意：不是默认的`rust`动态链接库`dylib`。在编译期间，它幼稚地试图将所有被链接到`DLL`文件都静态编译入一个结果`DLL`文件内。这“理想主义”作法直接造成了单个`DLL`导出`public ABI`数量超出上限的编译错误。

`scaffold-wizard`是`cargo new --bin`与`cargo new --lib`的混合体。

#### `cargo test`

执行针对`cdylib`的单元测试。还没有添加【集成测试】与【基准测试】。

#### `cargo run`

* 编译`rust`源码，和输出`target\debug\scaffold-wizard.exe`
* 在`msys2`包管理器的环境下，运行`target\debug\scaffold-wizard.exe`。

#### `node build.js`或`node build.js --release`

这里执行`js`程序有点突兀。但，它是被用来攒“绿色安装包”的。安装包的目录结构如下

```shell
.
├─ bin    # 若 windows 发行包，此目录需要包括 41 个 dll/exe 文件。若 Linux 发行包，仅 1 个 exe 文件。
|  ├─ ...
│  ├─ scaffold-wizard.exe # 仅出现在 target/setup-bin 目录下
|  ├─ ...
│  └─ scaffold_wizard.dll # 仅出现在 target/setup-lib 目录下
├─ lib    # 仅 windows 发行包需要此目录
│  └─ gdk-pixbuf-2.0
├─ share  # 仅 windows 发行包需要此目录
│  ├─ glib-2.0
│  └─ icons
├─ assets
│  ├─ prompt-manifest.json # 【问卷配置】样板文件
│  ├─ log4rs.json          # 日志配置文件
│  └─ images               # 自定义组件的图片
└─ logs   # 运行时滚动日志输出目录。
```

如上所述，要攒这么复杂的目录结构，使用`javascript`编写构建程序绝对是省时省力的明智选择。

```shell
npm i -g archiver
node build.js
```

上面的命令执行之后，其会在`target`目录下，创建两个子文件夹和两个`zip`文件

* `setup-bin`和`scaffold-wizard.setup-bin.zip` --- 独立执行程序和其绿色安装包
* `setup-lib`和`scaffold-wizard.setup-lib.zip` --- 动态链接库和其绿色安装包

双击运行“绿色安装包”内的`bin/scaffold-wizard.exe`。便可，在`msys2`包管理器环境之外，运行应用程序。同理，“绿色安装包”内的`scaffold_wizard.dll`也能够脱离`msys2`地被链接调用。但要稍稍再复杂一些。

#### `build.rs`

每当执行`cargo`指令时，这个构建程序也都会被执行。在`target`目录下，它会创建若干指向`msys2`的符号链接。所以，强调：环境变量`MSYS2_HOME`需要被配置，编译才能被正常地执行。

* 环境变量`MSYS2_HOME`保存了`msys2`的安装目录地址。

## 输入/输出说明

### 可执行文件的命令行参数

```log
前端脚手架安装向导 1.0
张浩予 <stuartpage@qq.com>
以【问卷】的形式，收集开发者对前端工程原型的“裁剪”条件信息

USAGE:
    scaffold-wizard.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input-file <INPUT_FILE>   【问卷配置】 json 文件（包括：题面，选项，默认值）。
                                    缺省此参数会弹出【文件选择对话框】要求你临时选择一个 json 文件。
    -l, --log4rs-file <LOG_FILE>    JSON 格式的 log4rs 配置文件。忽略此参数，程序会试从
                                    （1）.exe 文件所在的同级目录
                                    （2）程序被执行的工作目录
                                    寻找 ../assets/log4rs.json 文件。若两处都没有配置文件，
                                    程序日志功将不会被开启。
    -o, --output-file <OUTPUT_FILE> 【问卷】的答案清单 json 文件。默认输出文件是 answers.json。
                                    并且，输出文件会被放置于与输入文件（--input-file）相同的目录里。
```

### 【问卷配置】`json`文件

它全方位地抄袭了[Inquirer 的 Question 部分](https://github.com/SBoudrias/Inquirer.js#question)。但是，【回调函数钩子】那块，我是实在抄袭不来，原因包括：

* 第一，我自己不会做定制而精简的“脚本程序”词法分析与执行器。
* 第二，集成`JavascriptCore`引擎又太重了。

所以，现在阶段，我暂停点开这个方向的“科技树”。

另一方面，作为对缺失【回调函数钩子】的补偿，我在如下几处添加了新配置属性：

1. 给`"type": "input"`类型（即，文本输入框）添加了`"subType": "port"`子类。其专门收集【数字类型】，取值范围在`1000 ~ 99999`的端口号。样板配置如下：

    ```json
    {
        "appPort": { // 这个问题唯一标识字符串。相当于主键 ID。
            "when": "subprojects.app", // 条件表达式，当前问题是出现在交互流程中（true），还是被跳过（false）。
            "type": "input", // 文本输入框
            "subType": "port", // 之端口数字输入框
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
                "when": "compWhichEnd == 'pcBrowser'", // 此选项是否出现的`when`表达式
                "name": "Element UI", // 【显示用】完整名
                "short": "Element", // 【显示用】简称名 - 暂时尚未使用
                "value": "elementUI" // 【程序引用】此选项的唯一标识字符串
            }, { // 题面 - 单选项3
                "when": "compWhichEnd == 'mobileBrowser'", // 此选项是否出现的`when`表达式
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

### 【回答结果】`json`文件

首先，它会被输出至和输入文件相同的文件夹内。

其次，它全方位地抄袭了[Inquirer 的 Answers 部分](https://github.com/SBoudrias/Inquirer.js#answers)。

最后，补充说明：

* `"type": "checkbox"`类型题面对应的答案类型是`Map<String, boolean>`

### 调用·动态链接库

* 直接贴`nodejs`代码
* 在程序注释里，解释每个参数与返回值的用途

> 注意：
>
> * 在链接与调用`DLL`时，请保持`target\setup-lib`文件夹内的目录结构。
> * 在`windows`操作系统上，因为`C:\Windows\System32`目录下的`zlib1.dll`与`Gnome.GTK3`依赖的`zlib1.dll`名字冲突了。所以，为了让【问卷】`DLL`能够正常地运行，需要（无论是手动、还是程序自动）复制`.boilerplate\bin\zlib1.dll`到`node`安装目录的根目录（即，`node.exe`所在的文件夹）。

#### 同步接口调用

```javascript
const fs = require('fs');
const ffi = require('ffi');
const ref = require('ref');
const path = require('path');
const util = require('util');
// 准备【问卷配置】`json`文件
const homeDir = path.resolve('target/setup-lib');
const questionsFile = path.join(homeDir, 'assets/prompt-manifest.json');
const readFile = util.promisify(fs.readFile);
readFile(questionsFile, {encoding: 'utf8'}).then(questions => {
    // 加载 DLL
    const dllFile = path.join(homeDir, 'bin/scaffold_wizard.dll');
    const dllDir = path.dirname(dllFile);
    const scaffoldWizard = ffi.Library(dllFile, {
        inquire: ['string', ['string', 'string', 'string']],
    });
    // 调用 DLL
    // inquire(...) 一共有三个输入参数
    // (1) JSON 格式字符串，包括了【问卷配置】
    // (2) 被加载 DLL 文件所在的目录。以此，来寻找 assets\images 目录。
    // (3) log4rs 的配置文件路径。传一个空指针，表示关闭日志功能。
    // 输出返回值是 JSON 格式字符串，包括了【回答结果】
    const answers = scaffoldWizard.inquire(questions, dllDir, ref.NULL_POINTER);
    console.info('被收集的答案包括', answers);
});
```

#### 异步接口调用

```javascript
const fs = require('fs');
const ffi = require('ffi');
const ref = require('ref');
const path = require('path');
const util = require('util');
// 准备【问卷配置】`json`文件
const homeDir = path.resolve('target/setup-lib');
const questionsFile = path.join(homeDir, 'assets/prompt-manifest.json');
const readFile = util.promisify(fs.readFile);
readFile(questionsFile, {encoding: 'utf8'}).then(questions => {
    // 加载 DLL
    const dllFile = path.join(homeDir, 'bin/scaffold_wizard.dll');
    const dllDir = path.dirname(dllFile);
    const scaffoldWizard = ffi.Library(dllFile, {
        inquireAsync: ['void', ['string', 'string', 'string', 'pointer']]
    });
    // 调用 DLL
    // inquire(...) 一共有三个输入参数
    // (1) JSON 格式字符串，包括了【问卷配置】
    // (2) 被加载 DLL 文件所在的目录。以此，来寻找 assets\images 目录。
    // (3) log4rs 的配置文件路径。传一个空指针，表示关闭日志功能。
    // 输出返回值是 JSON 格式字符串，包括了【回答结果】
    scaffoldWizard.inquireAsync(questions, dllDir, ref.NULL_POINTER, ffi.Callback('void', ['string', 'string'],
        finishedBuilder((err, answers) => {
            if (err) {
                console.error('失败原因', err);
            } else {
                console.info('被收集的答案包括', answers);
            }
        })
    ));
});
function finishedBuilder(callback){
    let timerId;
    const holding = () => { // 锁住 event loop，不立即结束程序。
        timerId = setTimeout(holding, 1000 * 60 * 60);
    };
    holding();
    return (err, answers) => {
        clearTimeout(timerId); // 解锁
        return callback(err, answers);
    };
}
```

### `N-API`封装

即将到来。

* 正在阅读`N-API`相关文档（主要是`Rust Binding`的内容）。应该不难。
* 但是，`N-API`也有不足，其对`node 10`之前的版本不兼容。

### `Neon`封装

即将到来。

## 执行演示

运行这款工具分发包的最简单方式就是：

1. 双击`target\setup-bin\bin\scaffold-wizard.exe`
2. 直接弹出【文件选择对话框】，默认打开`target\setup-bin\assets`文件夹，要求你选择一个【问卷配置】`json`文件。
3. 选择`prompt-manifest.json`文件，点击【打开】按钮。

   ![image](https://user-images.githubusercontent.com/13935927/119863250-f3488280-bf4b-11eb-9697-58edf1127b6e.png)

4. 开始回答问题。

   ![image](https://user-images.githubusercontent.com/13935927/119863392-14a96e80-bf4c-11eb-986e-ed05ec7b6b62.png)

5. 期间，不能退出。

   ![image](https://user-images.githubusercontent.com/13935927/119863511-37d41e00-bf4c-11eb-97e8-37b5dd763983.png)

6. 完成所有问题之后，点击【完成】按钮。
7. 程序退出。
8. 【回答结果】`json`文件被输出到和输入文件相同的目录下，文件名为`answers.json`。

> 我已经在`windows 10x64`与`windows 7x64`亲自验证过了。

## 后继阶段的工作计划

1. 完成`N-API`封装，让它更容易地与`nodejs`集成。`node-ffi`的集成方式还是太繁琐了。能够直接支持操作系统也有限。比如说，【中标麒麟】的国产操作系统就没有被明确地表示支持。
2. 完成`Neon`封装
3. 向`ubuntu`, `MacOS`操作系统交叉编译
4. 就`DLL`或`C node module`【安装向导】组件这个业务场景，实现更高级的业务功能。即，
   1. 接收【调用端】传入的回调函数。
   2. 每完成一步【问题-收集】就调用回调函数向【调用端】通报进度，和暂停【交互流程】
   3. 【调用端】异步地执行一些工作，再借助回调函数的返回值通知【安装向导】继续【交互流程】
   4. 直到整个安装过程结束。
5. 将此工程内一些通用的部分添加到【前端-脚手架】内的【`rust`工程原型】里。比如，
   1. `build.js`脚本与`.boilerplate`目录，生成【绿色安装包】
   2. `build.rs`为`cargo run`准备符号链接
6. 考虑到`WebkitGTK`不兼容于`windows`操作系统。后续不可避免向`QT`组件库技术转向。

## 希望路过“大神”帮我看看

我这`cargo build --release`编译出来的`dll`与`exe`都有点儿大（大约`20MB`）。这似乎有些不正常。路过的【神仙哥哥】与【神仙妹妹】是否可以帮我看看，我这是代码或编译配置，哪里有问题呀？
