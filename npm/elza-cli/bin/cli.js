#!/usr/bin/env node

const path = require('path');
const childProcess = require('child_process');

// 平台-二进制文件对照表
const optionalDependencies = require('./package.json').optionalDependencies;
const BINARY_DISTRIBUTION_PACKAGES = Object.keys(optionalDependencies).reduce(
  (pre, dep) => {
    const key = dep.replace('@elza-cli/', '');
    pre[key] = dep;
    return pre;
  },
  {}
);

// Windows平台的二进制文件以.exe结尾，因此需要特殊处理
const binaryName = process.platform === 'win32' ? 'elza-cli.exe' : 'elza-cli';

// 确定此平台的软件包名称
const platformSpecificPackageName =
  BINARY_DISTRIBUTION_PACKAGES[`${process.platform}-${process.arch}`];

function getBinaryPath() {
  try {
    // 尝试解析特定平台的二进制文件路径
    return require.resolve(`${platformSpecificPackageName}/bin/${binaryName}`);
  } catch (e) {
    // 如果未安装相关的 optional dependency，返回备用的二进制文件路径
    return path.join(__dirname, '..', binaryName);
  }
}

// 使用child_process模块执行二进制文件并传递命令行参数
try {
  const binaryPath = getBinaryPath();
  childProcess.execFileSync(binaryPath, process.argv.slice(2), {
    stdio: 'inherit',
  });
} catch (error) {
  process.exit(1);
}