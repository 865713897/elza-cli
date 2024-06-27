const fs = require('fs').promises;
const path = require('path');
const { logger, style } = require('./logger');

// 解析命令行参数，指定版本号增量类型
const versionType = process.argv[2] || 'patch';

async function walk(dir) {
  try {
    const files = await fs.readdir(dir);
    await Promise.all(
      files.map(async (file) => {
        const filePath = path.join(dir, file);
        const stat = await fs.stat(filePath);
        if (stat.isDirectory()) {
          await walk(filePath);
        } else if (file === 'package.json') {
          await updateVersion(filePath);
        }
      })
    );
  } catch (err) {
    logger.error(`无法读取文件夹 ${dir}`);
  }
}

// 更新Cargo.toml中的版本号
async function updateCargoToml() {
  const filePath = 'Cargo.toml';
  try {
    let data = await fs.readFile(filePath, 'utf-8');
    // 使用正则表达式匹配 version 字段
    const versionMatch = data.match(/version = "([^"]+)"/);
    if (versionMatch && versionMatch[1]) {
      const oldVersion = versionMatch[1];
      const newVersion = getLatestVersion(oldVersion, versionType);
      data = data.replace(versionMatch[0], `version = "${newVersion}"`);
      logger.event(
        `${filePath} ${style('v' + oldVersion).red()} -> ${style(
          'v' + newVersion
        ).green()}`
      );
      await fs.writeFile(filePath, data, 'utf8');
    }
  } catch (error) {
    logger.error(`${filePath} 更新版本出错`);
  }
}

// 更新版本号
async function updateVersion(filePath) {
  try {
    const data = await fs.readFile(filePath, 'utf-8');
    const json = JSON.parse(data);
    const oldVersion = json.version;
    json.version = getLatestVersion(oldVersion, versionType);
    logger.event(
      `${filePath} ${style('v' + oldVersion).red()} -> ${style(
        'v' + json.version
      ).green()}`
    );
    if (json.optionalDependencies) {
      for (const dep in json.optionalDependencies) {
        json.optionalDependencies[dep] = getLatestVersion(
          json.optionalDependencies[dep],
          versionType
        );
      }
    }
    await fs.writeFile(filePath, JSON.stringify(json, null, 2) + '\n', 'utf8');
  } catch (err) {
    logger.error(`${filePath} 更新版本出错`);
  }
}

// 获取最新版本号
function getLatestVersion(version = '', versionType) {
  const parts = version.split('.');
  if (parts.length !== 3) {
    logger.error(`无效的版本号 v${version}`);
    return version;
  }
  let [major, minor, patch] = parts.map((part) => parseInt(part, 10));
  if (isNaN(major) || isNaN(minor) || isNaN(patch)) {
    logger.error(`无效的版本号 v${version}`);
    return version;
  }
  switch (versionType) {
    case 'major':
      major += 1;
      minor = 0;
      patch = 0;
      break;
    case 'minor':
      minor += 1;
      patch = 0;
      break;
    case 'patch':
    default:
      patch += 1;
      break;
  }
  return `${major}.${minor}.${patch}`;
}

const npmDir = './npm'; // 设置npm目录的相对路径
logger.info('开始更新版本号...');
updateCargoToml();
walk(npmDir)
  .then(() => {
    logger.ready('版本号更新完成');
  })
  .catch((err) => console.error('Error during directory walk:', err));
