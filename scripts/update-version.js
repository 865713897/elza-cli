const fs = require('fs').promises;
const path = require('path');

// 解析命令行参数，指定版本号增量类型
const versionType = process.argv[2] || 'patch';

async function walk(dir) {
  try {
    const files = await fs.readdir(dir);
    for (const file of files) {
      const filePath = path.join(dir, file);
      const stat = await fs.stat(filePath);
      if (stat.isDirectory()) {
        await walk(filePath);
      } else if (file === 'package.json') {
        await updateVersion(filePath);
      }
    }
  } catch (err) {
    console.error(`Error reading directory ${dir}:`, err);
  }
}

// 更新版本号
async function updateVersion(filePath) {
  try {
    const data = await fs.readFile(filePath, 'utf-8');
    const json = JSON.parse(data);
    json.version = getLatestVersion(json.version, versionType);
    if (json.optionalDependencies) {
      for (const dep in json.optionalDependencies) {
        json.optionalDependencies[dep] = getLatestVersion(json.optionalDependencies[dep], versionType);
      }
    }
    await fs.writeFile(filePath, JSON.stringify(json, null, 2) + '\n', 'utf8');
  } catch (err) {
    console.error(`Error updating version in ${filePath}:`, err);
  }
}

// 获取最新版本号
function getLatestVersion(version = '', versionType) {
  const parts = version.split('.');
  if (parts.length !== 3) {
    console.error(`Invalid version format: ${version}`);
    return version;
  }
  let [major, minor, patch] = parts.map(part => parseInt(part, 10));
  if (isNaN(major) || isNaN(minor) || isNaN(patch)) {
    console.error(`Invalid version format: ${version}`);
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
walk(npmDir).catch(err => console.error('Error during directory walk:', err));