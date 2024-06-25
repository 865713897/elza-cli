const fs = require('fs').promises;
const path = require('path');
const { exec } = require('child_process');

const npmDir = './npm'; // 定义npm包的目录

async function publishPackage(dirPath) {
  try {
    await execPromise('npm publish --access public', { cwd: dirPath });
    console.log(`Published ${dirPath}`);
  } catch (error) {
    console.error(`Failed to publish ${dirPath}: ${error.message}`);
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
    console.error(`Error reading directory ${dirPath}: ${error.message}`);
  }
}

findAndPublishPackages(npmDir).catch(error => {
  console.error(`Failed to process ${npmDir}: ${error.message}`);
});