const fs = require('fs').promises;
const path = require('path');
const { exec } = require('child_process');
const { logger } = require('./logger');

const npmDir = './npm'; // 定义npm包的目录

async function publishPackage(dirPath) {
  try {
    logger.event(`${dirPath} -> npm publish --access public`);
    await execPromise('npm publish --access public', { cwd: dirPath });
    logger.ready(`发布 ${dirPath} 完成`);
  } catch (error) {
    logger.error(`发布 ${dirPath} 失败: ${error.message}`);
  }
}

function execPromise(command, options) {
  return new Promise((resolve, reject) => {
    exec(command, options, (error, stdout, stderr) => {
      if (error) {
        reject(new Error(stderr));
      } else {
        resolve(stdout);
      }
    });
  });
}

async function findAndPublishPackages(dirPath) {
  try {
    const entries = await fs.readdir(dirPath, { withFileTypes: true });
    for (const entry of entries) {
      const fullPath = path.join(dirPath, entry.name);
      if (entry.isDirectory()) {
        const packageJsonPath = path.join(fullPath, 'package.json');
        try {
          await fs.access(packageJsonPath);
          await publishPackage(fullPath); // 发现package.json，发布包
        } catch {
          await findAndPublishPackages(fullPath); // 继续递归搜索子目录
        }
      }
    }
  } catch (error) {
    logger.error(`读取 ${dirPath} 失败: ${error.message}`);
  }
}

logger.info('开始发布npm包...');
findAndPublishPackages(npmDir).catch((error) => {
  logger.error(`处理 ${npmDir} :`, error.message);
});
