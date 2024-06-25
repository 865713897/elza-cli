const fs = require('fs');
const path = require('path');
const zlib = require('zlib');
const https = require('https');

// 平台-二进制文件对照表
const BINARY_DISTRIBUTION_PACKAGES = {
  'darwin-arm64': '@elza-cli/darwin-arm64',
  'win32-x64': '@elza-cli/win32-x64',
};

// 安装版本
const BINARY_DISTRIBUTION_VERSION = require('./package.json').version;

// 处理windows平台文件结尾
const binaryName = process.platform === 'win32' ? 'elza-cli.exe' : 'elza-cli';

// 确定当前平台包名
const platformBinaryName =
  BINARY_DISTRIBUTION_PACKAGES[`${process.platform}-${process.arch}`];

// 计算备用二进制文件路径
const fallbackBinaryPath = path.join(__dirname, binaryName);

// 创建http请求promise
function makeRequest(url) {
  return new Promise((resolve, reject) => {
    https
      .get(url, (response) => {
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
          makeRequest(response.headers.location).then(resolve).catch(reject);
        } else {
          reject(
            new Error(
              `HTTP request failed with status code ${response.statusCode}`
            )
          );
        }
      })
      .on('error', (error) => {
        reject(error);
      });
  });
}

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
async function downloadBinary() {
  try {
    const packageName = platformBinaryName.replace('@elza-cli/', '');
    const tarballDownloadBuffer = await makeRequest(
      `https://registry.npmjs.org/${platformBinaryName}/-/${packageName}-${BINARY_DISTRIBUTION_VERSION}.tgz`
    );

    const tarballBuffer = zlib.unzipSync(tarballDownloadBuffer);

    const binaryBuffer = extractFileFromTarball(
      tarballBuffer,
      `package/bin/${binaryName}`
    );

    fs.writeFileSync(fallbackBinaryPath, binaryBuffer, { mode: 0o755 });

    console.log('二进制文件下载并解压成功');
  } catch (error) {
    console.error('二进制文件下载失败:', error.message);
    process.exit(1);
  }
}

// 检查是否已经安装过
function isPlatformSpecificPackageInstalled() {
  try {
    require.resolve(`${platformBinaryName}/bin/${binaryName}`);
    return true;
  } catch (err) {
    return false;
  }
}

// 如果不支持，抛出错误
if (!platformBinaryName) {
  throw new Error(`The platform "${process.platform}" is not supported.`);
}

// 如果已经安装过，直接使用
if (!isPlatformSpecificPackageInstalled()) {
  console.log('未找到平台特定软件包，将手动下载二进制文件');
  downloadBinary()
    .then(() => {
      console.log('二进制文件下载并解压成功');
    })
    .catch((error) => {
      console.error('二进制文件下载失败:', error.message);
    });
} else {
  console.log('平台特定的软件包已安装。将回退到手动下载二进制文件');
  downloadBinary()
    .then(() => {
      console.log('二进制文件下载并解压成功');
    })
    .catch((error) => {
      console.error('二进制文件下载失败:', error.message);
    });
}
