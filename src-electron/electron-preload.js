const { contextBridge, ipcRenderer } = require("electron");
const fs = require("fs");
const path = require("path");
const MarkdownIt = require("markdown-it");

const mdParser = new MarkdownIt();

contextBridge.exposeInMainWorld("electronAPI", {
  saveDiary(date, content) {
    const diaryDir = getDiaryDir();
    if (!fs.existsSync(diaryDir)) {
      fs.mkdirSync(diaryDir, { recursive: true });
    }
    const filePath = path.join(diaryDir, `${date}.txt`);
    fs.writeFileSync(filePath, content, "utf-8");
  },
  loadDiary(date) {
    const filePath = path.join(getDiaryDir(), `${date}.txt`);
    if (fs.existsSync(filePath)) {
      return fs.readFileSync(filePath, "utf-8");
    }
    return null;
  },
  getAllDiaries() {
    return getAllDiariesLocal();
  },
  saveImage: (fileData) => ipcRenderer.invoke('save-image', fileData),
});

/**
 * 导出为 JSON
 */
ipcRenderer.on("request-export-json", async () => {
  try {
    const diaries = getAllDiariesLocal();
    const filePath = await ipcRenderer.invoke("show-save-dialog");
    if (!filePath) {
      // 用户取消保存
      return await showDialog("info", "Canceled", "JSON export canceled by user.");
    }

    fs.writeFileSync(filePath, JSON.stringify(diaries, null, 2), "utf-8");
    await showDialog("info", "Success", `Diaries have been exported to JSON:\n${filePath}`);
  } catch (err) {
    await showDialog("error", "Export Error", `Failed to export diaries.\n${err.message}`);
  }
});

/**
 * 导出为 PDF
 */
ipcRenderer.on("request-export-pdf", async () => {
  try {
    const diaries = getAllDiariesLocal();
    if (!diaries.length) {
      return await showDialog("info", "No Diaries", "No diaries to export!");
    }

    // 将所有日记合并成 HTML
    let fullHTML = `
      <html>
      <head>
        <meta charset="UTF-8" />
        <style>
          body {
            font-family: "Microsoft YaHei", Arial, sans-serif;
            margin: 20px;
          }
          h2 {
            margin: 0.5em 0;
          }
          hr {
            margin: 1em 0;
          }
        </style>
      </head>
      <body>
    `;

    diaries.forEach((d, i) => {
      fullHTML += `
        <h2>${d.date}</h2>
        ${mdParser.render(d.content)}
        ${i < diaries.length - 1 ? "<hr />" : ""}
      `;
    });

    fullHTML += "</body></html>";

    const pdfPath = await ipcRenderer.invoke("show-save-dialog-pdf");
    if (!pdfPath) {
      // 用户取消保存
      return await showDialog("info", "Canceled", "PDF export canceled by user.");
    }

    // 让主进程执行打印 PDF
    await ipcRenderer.invoke("export-to-pdf", { html: fullHTML, savePath: pdfPath });
    await showDialog("info", "Success", `Diaries have been exported to PDF:\n${pdfPath}`);
  } catch (err) {
    await showDialog("error", "Export PDF Error", `Failed to export PDF.\n${err.message}`);
  }
});

ipcRenderer.on("request-export-images", async () => {
  try {
    const diaryDir = getDiaryDir();
    const imagesDir = path.join(diaryDir, 'images');

    // 检查图片目录是否存在
    if (!fs.existsSync(imagesDir)) {
      return await showDialog('info', 'Export Images', 'No images directory found.');
    }

    // 读取所有文件
    const files = fs.readdirSync(imagesDir);
    if (files.length === 0) {
      return await showDialog('info', 'Export Images', 'The images directory is empty.');
    }

    // 选择目标目录
    const targetDir = await ipcRenderer.invoke('show-directory-dialog', {
      title: 'Select Export Destination Folder',
      properties: ['openDirectory', 'createDirectory']
    });
    if (!targetDir) return;

    // 确保目标目录存在
    if (!fs.existsSync(targetDir)) {
      fs.mkdirSync(targetDir, { recursive: true });
    }

    // 复制所有文件
    let copiedCount = 0;
    files.forEach(file => {
      const srcPath = path.join(imagesDir, file);
      const destPath = path.join(targetDir, file);
      
      if (fs.statSync(srcPath).isFile()) {
        fs.copyFileSync(srcPath, destPath);
        copiedCount++;
      }
    });

    await showDialog('info', 'Success', `Exported ${copiedCount} images to:\n${targetDir}`);
  } catch (err) {
    await showDialog('error', 'Export Error', `Failed to export images: ${err.message}`);
  }
});

/**
 * 从 JSON 加载日记
 */
ipcRenderer.on("request-load-json", async () => {
  try {
    const jsonFilePath = await ipcRenderer.invoke("show-open-dialog-json");
    if (!jsonFilePath) {
      return await showDialog("info", "Canceled", "Load diaries canceled by user.");
    }

    const fileContent = fs.readFileSync(jsonFilePath, "utf-8");
    const diaries = JSON.parse(fileContent);
    if (!Array.isArray(diaries)) {
      throw new Error("JSON file format incorrect, expecting an array of diaries.");
    }

    const diaryDir = getDiaryDir();
    if (!fs.existsSync(diaryDir)) {
      fs.mkdirSync(diaryDir, { recursive: true });
    }

    diaries.forEach(({ date, content }) => {
      const filePath = path.join(diaryDir, `${date}.txt`);
      fs.writeFileSync(filePath, content, "utf-8");
    });

    await showDialog("info", "Success", `Successfully loaded ${diaries.length} diaries from JSON.`);
  } catch (err) {
    await showDialog("error", "Load Error", `Failed to load diaries.\n${err.message}`);
  }
});

// 添加新的菜单项处理
ipcRenderer.on("request-load-images", async () => {
  try {
    // 1. 选择源目录
    const sourceDir = await ipcRenderer.invoke('show-directory-dialog', {
      title: 'Select Image Source Folder'
    });
    if (!sourceDir) return;

    // 2. 验证源目录
    if (!fs.existsSync(sourceDir)) {
      return await showDialog('error', 'Import Error', 'Selected directory does not exist');
    }

    // 3. 获取目标目录
    const diaryDir = getDiaryDir();
    const targetDir = path.join(diaryDir, 'images');
    if (!fs.existsSync(targetDir)) {
      fs.mkdirSync(targetDir, { recursive: true });
    }

    // 4. 遍历并复制图片
    const allowedExts = ['.png', '.jpg', '.jpeg', '.gif', '.webp'];
    const files = fs.readdirSync(sourceDir);
    let importedCount = 0;
    let skippedCount = 0;

    for (const file of files) {
      const ext = path.extname(file).toLowerCase();
      if (!allowedExts.includes(ext)) {
        skippedCount++;
        continue;
      }

      const sourcePath = path.join(sourceDir, file);
      const stats = fs.statSync(sourcePath);
      if (!stats.isFile()) {
        skippedCount++;
        continue;
      }

      // 生成唯一文件名
      const baseName = path.basename(file, ext);
      let destFile = `${baseName}${ext}`;
      let counter = 1;

      while (fs.existsSync(path.join(targetDir, destFile))) {
        destFile = `${baseName}_${Date.now()}${ext}`;
        counter++;
      }

      fs.copyFileSync(sourcePath, path.join(targetDir, destFile));
      importedCount++;
    }

    // 5. 显示结果
    let message = `Successfully imported ${importedCount} images`;
    if (skippedCount > 0) {
      message += `\nSkipped ${skippedCount} non-image files`;
    }
    await showDialog('info', 'Import Complete', message);
    
    // 6. 刷新应用中的图片显示（根据你的应用逻辑可能需要添加）
    // window.location.reload(); 或其他更新逻辑

  } catch (err) {
    await showDialog('error', 'Import Error', `Failed to import images: ${err.message}`);
  }
});


/**
 * 显示当前的日记存储路径
 */
ipcRenderer.on("request-show-info", async () => {
  try {
    const diaryDir = getDiaryDir();
    await showDialog("info", "Diary Path", `Current diary storage path:\n${diaryDir}`);
  } catch (err) {
    await showDialog("error", "Error", `Failed to get diary path.\n${err.message}`);
  }
});

/**
 * 修改日记存储路径
 */
ipcRenderer.on("request-change-info", async () => {
  try {
    const newDir = await ipcRenderer.invoke("show-directory-dialog");
    if (!newDir) {
      // 用户取消选择
      return await showDialog("info", "Canceled", "Change storage path canceled by user.");
    }

    // 更新 config.json
    const config = readConfig();
    config.diaryDir = newDir;
    writeConfig(config);

    await showDialog("info", "Success", `Diary path changed to:\n${newDir}`);
  } catch (err) {
    await showDialog("error", "Error", `Failed to change diary path.\n${err.message}`);
  }
});

// ========== 辅助函数 ========== //

/**
 * 显示系统对话框
 */
async function showDialog(type, title, message) {
  return ipcRenderer.invoke("show-dialog", { type, title, message });
}

/**
 * 获取所有日记
 */
function getAllDiariesLocal() {
  const diaryDir = getDiaryDir();
  if (!fs.existsSync(diaryDir)) return [];
  const files = fs.readdirSync(diaryDir).filter((f) => f.endsWith(".txt") || f.endsWith(".md"));
  return files.map((file) => {
    const date = file.replace(".txt", "").replace(".md", "");
    const content = fs.readFileSync(path.join(diaryDir, file), "utf-8");
    return { date, content };
  });
}

/**
 * 获取/写入 config.json
 */
function getConfigFilePath() {
  const userDataPath = ipcRenderer.sendSync("get-user-data-path");
  return path.join(userDataPath, "config.json");
}

function readConfig() {
  const configFile = getConfigFilePath();
  if (!fs.existsSync(configFile)) return {};
  try {
    return JSON.parse(fs.readFileSync(configFile, "utf-8"));
  } catch (err) {
    console.warn("Failed to parse config.json", err);
    return {};
  }
}

function writeConfig(configObj) {
  fs.writeFileSync(getConfigFilePath(), JSON.stringify(configObj, null, 2), "utf-8");
}

/**
 * 获取当前日记目录
 * 如果 config.json 里有 diaryDir 并且存在，就用之，否则用默认的 [userData]/diary
 */
function getDiaryDir() {
  const config = readConfig();
  if (config.diaryDir && fs.existsSync(config.diaryDir)) {
    return config.diaryDir;
  }
  const defaultDir = path.join(ipcRenderer.sendSync("get-user-data-path"), "diary");
  return defaultDir;
}
