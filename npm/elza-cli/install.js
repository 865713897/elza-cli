const fs = require('fs');
const path = require('path');
const zlib = require('zlib');
const { execSync } = require('child_process');
const https = require('https');
const { logger } = require('./logger');

console.log('install.js script started'); // Debug log

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

// 安装版本
const CURRENT_VERSION = require('./package.json').version;

// 处理windows平台文件结尾
const binaryName = process.platform === 'win32' ? 'elza-cli.exe' : 'elza-cli';

// 确定当前平台包名
const platformBinaryName =
  BINARY_DISTRIBUTION_PACKAGES[`${process.platform}-${process.arch}`];

// 计算备用二进制文件路径
const fallbackBinaryPath = path.join(__dirname, binaryName);

// 获取全局安装路径
function getGlobalNodeModulesPath() {
  const globalNodeModulesPath = execSync('npm root -g').toString().trim();
  return globalNodeModulesPath;
}

// 计算全局安装路径下的二进制文件路径
const globalBinaryPath = path.join(getGlobalNodeModulesPath(), binaryName);

// 创建http请求promise
function makeRequest(url, retries = 3, timeout = 5000) {
  return new Promise((resolve, reject) => {
    let attempts = 0;
    function request() {
      attempts++;
      let handled = false; // 标记是否已经处理过
      const req = https.get(url, (response) => {
        if (response.statusCode >= 200 && response.statusCode < 300) {
          const chunks = [];
          response.on('data', (chunk) => chunks.push(chunk));
          response.on('end', () => {
            resolve(Buffer.concat(chunks));
          });
        } else if (
          response.statusCode >= 300 &&
          response.statusCode < 400 &&
          response.headers.location
        ) {
          // 重定向
          makeRequest(response.headers.location, retries, timeout)
            .then(resolve)
            .catch(reject);
        } else {
          handleRetry();
        }
      });
      req.on('error', () => {
        if (!handled) {
          handled = true;
          handleRetry();
        }
      });
      req.setTimeout(timeout, () => {
        if (!handled) {
          handled = true;
          req.abort();
          handleRetry();
        }
      });
    }

    function handleRetry() {
      if (attempts < retries) {
        logger.warn('获取版本失败, 重试中...');
        setTimeout(request, 1000);
      } else {
        reject();
      }
    }

    request();
  });
}

// 获取用户npm源
function getUserNpmRegistry() {
  return execSync('npm config get registry').toString().trim();
}

const npmRegistry = getUserNpmRegistry() || 'https://registry.npmjs.org/';

// 从tarball中提取文件
function extractFileFromTarball(tarballBuffer, filepath) {
  let offset = 0;
  while (offset < tarballBuffer.length) {
    const header = tarballBuffer.subarray(offset, offset + 512);
    offset += 512;

    const fileName = header.toString('utf-8', 0, 100).replace(/\0.*/g, '');
    const fileSize = parseInt(
      header.toString('utf-8', 124, 136).replace(/\0.*/g, ''),
      8
    );

    if (fileName === filepath) {
      return tarballBuffer.subarray(offset, offset + fileSize);
    }

    offset = (offset + fileSize + 511) & ~511;
  }

  throw new Error(`File ${filepath} not found in tarball`);
}

// 下载二进制文件
async function downloadBinary(version) {
  try {
    const packageName = platformBinaryName.replace('@elza-cli/', '');
    const tarballDownloadBuffer = await makeRequest(
      `${npmRegistry}${platformBinaryName}/-/${packageName}-${version}.tgz`
    );
    logger.info('二进制文件下载完成');
    logger.event('开始解压二进制文件');
    const tarballBuffer = zlib.unzipSync(tarballDownloadBuffer);
    logger.info('二进制文件解压完成');
    const binaryBuffer = extractFileFromTarball(
      tarballBuffer,
      `package/bin/${binaryName}`
    );
    fs.writeFileSync(fallbackBinaryPath, binaryBuffer, { mode: 0o755 });
    logger.ready('已完成下载');
    process.exit(0);
  } catch (_) {
    logger.error('二进制文件下载失败');
    process.exit(1);
  }
}

// 获取最新版本
async function getLatestVersion(packageName) {
  const url = `${npmRegistry}${packageName}`;
  try {
    const data = await makeRequest(url);
    const packageInfo = JSON.parse(data.toString('utf8'));
    return packageInfo['dist-tags'].latest;
  } catch (_) {
    logger.error(`获取最新版本失败, 请检查网络或更改npm镜像源`);
    process.exit(1);
  }
}

// 检查并下载新版本
async function checkAndUpdate() {
  const latestVersion = await getLatestVersion(platformBinaryName);
  if (latestVersion !== CURRENT_VERSION) {
    logger.info_version(`发现新版本 v${latestVersion}`);
    await downloadBinary(latestVersion);
  } else {
    logger.ready('已是最新版本');
  }
}

// 检查是否已经安装过
function isPlatformSpecificPackageInstalled() {
  return fs.existsSync(globalBinaryPath);
}

// 如果不支持，抛出错误
if (!platformBinaryName) {
  throw new Error(`The platform "${process.platform}" is not supported.`);
}

// 如果已经安装过，直接使用
if (!isPlatformSpecificPackageInstalled()) {
  logger.info('平台未检测到软件包');
  logger.event('开始下载二进制文件...');
  downloadBinary(CURRENT_VERSION);
} else {
  logger.info('平台已检测到软件包');
  logger.event('开始检查更新...');
  checkAndUpdate();
}
