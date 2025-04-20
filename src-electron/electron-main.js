import { app, BrowserWindow, Menu, ipcMain, dialog, protocol } from "electron";
import path from "path";
import os from "os";
import fs from "fs";

const platform = process.platform || os.platform();

let mainWindow;

function createWindow() {
    mainWindow = new BrowserWindow({
        icon: path.resolve(__dirname, "icons/icon.png"),
        width: 1400,
        height: 600,
        roundedCorners: true,
        transparent: true,
        // frame: false,
        useContentSize: true,

        webSecurity: false,
        allowRunningInsecureContent: true,
        
        webPreferences: {
            contextIsolation: true,
            nodeIntegration: true,
            preload: path.resolve(
                __dirname,
                process.env.QUASAR_ELECTRON_PRELOAD
            ),
        },
    });

    mainWindow.loadURL(process.env.APP_URL);

    if (process.env.DEBUGGING) {
        // mainWindow.webContents.openDevTools();
    } else {
        mainWindow.webContents.on("devtools-opened", () => {
            mainWindow.webContents.closeDevTools();
        });
    }

    mainWindow.on("closed", () => {
        mainWindow = null;
    });

    // ========== Application Menu ========== //
    const menuTemplate = [
        {
            label: "Export",
            submenu: [
                {
                    label: "JSON", // Export as JSON
                    click: () => {
                        mainWindow.webContents.send("request-export-json");
                    },
                },
                {
                    label: "PDF", // Export as PDF
                    click: () => {
                        mainWindow.webContents.send("request-export-pdf");
                    },
                },
                {
                    label: "Images",
                    click: () => {
                        mainWindow.webContents.send("request-export-images");
                    }
                }
            ],
        },
        {
            label: "Load",
            submenu: [
                {
                    label: "From JSON", // Load From Json
                    click: () => {
                        mainWindow.webContents.send("request-load-json");
                    },
                },
                {
                    label: "From Images", // Load From Images
                    click: () => {
                        mainWindow.webContents.send("request-load-images");
                    },
                },
            ],
        },
        {
            label: "Info",
            submenu: [
                {
                    label: "Show Storage Path",
                    click: () => {
                        // Show current storage location
                        mainWindow.webContents.send("request-show-info");
                    },
                },
                {
                    label: "Change Storage Path",
                    click: () => {
                        // Change storage location
                        mainWindow.webContents.send("request-change-info");
                    },
                },
            ],
        },
    ];

    const menu = Menu.buildFromTemplate(menuTemplate);
    Menu.setApplicationMenu(menu);
}

// 在 app.whenReady() 中添加协议注册
app.whenReady().then(() => {
    // 注册本地文件协议
    protocol.registerFileProtocol('diary', (request, callback) => {
      try {
        // 解码URL路径（示例URL: diary://images/xxx.png）
        const decodedPath = decodeURIComponent(request.url.replace('diary://', ''));
        // 获取当前日记存储目录
        const diaryDir = getCurrentDiaryDir(); // 使用你现有的getCurrentDiaryDir方法
        // 拼接完整路径
        const fullPath = path.join(diaryDir, decodedPath);
        callback(fullPath);
      } catch (err) {
        console.error('协议处理错误:', err);
        callback({ error: -2 }); // 返回错误代码
      }
    });
  
    createWindow(); // 保持原有创建窗口逻辑
});
// app.whenReady().then(createWindow);

app.on("window-all-closed", () => {
    if (platform !== "darwin") {
        app.quit();
    }
});

app.on("activate", () => {
    if (mainWindow === null) {
        createWindow();
    }
});

/**
 * 让用户选择保存 JSON 的对话框
 */
ipcMain.handle("show-save-dialog", async () => {
    const { filePath } = await dialog.showSaveDialog({
        title: "Export diaries as JSON",
        defaultPath: "diaries.json",
        filters: [{ name: "JSON Files", extensions: ["json"] }],
    });
    return filePath || null;
});

/**
 * 让用户选择保存 PDF 的对话框
 */
ipcMain.handle("show-save-dialog-pdf", async () => {
    const { filePath } = await dialog.showSaveDialog({
        title: "Export diaries as PDF",
        defaultPath: "diaries.pdf",
        filters: [{ name: "PDF Files", extensions: ["pdf"] }],
    });
    return filePath || null;
});

/**
 * 让用户选择要导入的 JSON 文件
 */
ipcMain.handle("show-open-dialog-json", async () => {
    const { canceled, filePaths } = await dialog.showOpenDialog({
        title: "Select a JSON file",
        filters: [{ name: "JSON Files", extensions: ["json"] }],
        properties: ["openFile"],
    });
    if (canceled || !filePaths || filePaths.length === 0) {
        return null;
    }
    return filePaths[0];
});

/**
 * 用隐藏窗口把 HTML 转成 PDF 并保存
 */
ipcMain.handle("export-to-pdf", async (event, { html, savePath }) => {
    if (!html || !savePath) {
        throw new Error("export-to-pdf: 缺少 html 或 savePath");
    }
    const pdfWin = new BrowserWindow({ show: false });
    await pdfWin.loadURL(
        "data:text/html;charset=utf-8," + encodeURIComponent(html)
    );
    const pdfData = await pdfWin.webContents.printToPDF({
        landscape: false,
        printBackground: true,
    });
    fs.writeFileSync(savePath, pdfData);
    pdfWin.close();
    return { success: true };
});

/**
 * 让用户选择新日记目录
 */
ipcMain.handle("show-directory-dialog", async () => {
    const { canceled, filePaths } = await dialog.showOpenDialog({
        title: "Select a folder to store diaries",
        properties: ["openDirectory", "createDirectory"],
    });
    if (canceled || !filePaths || filePaths.length === 0) {
        return null;
    }
    return filePaths[0];
});

/**
 * 通用的消息对话框，用于在 Preload 中弹窗显示结果或错误
 */
ipcMain.handle("show-dialog", async (event, { type, title, message }) => {
    // type 可以是 "none" | "info" | "error" | "question" | "warning"
    await dialog.showMessageBox({
        type,
        title,
        message,
    });
});

/**
 * 返回 userData 路径给 Preload
 */
ipcMain.on("get-user-data-path", (event) => {
    event.returnValue = app.getPath("userData");
});

ipcMain.handle('save-image', async (_, fileData) => {
  const diaryDir = getCurrentDiaryDir();
  const imagesDir = path.join(diaryDir, 'images');
  
  // 确保目录存在
  if (!fs.existsSync(imagesDir)) {
    fs.mkdirSync(imagesDir, { recursive: true });
  }

  // 保留原始文件名（安全过滤）
  const cleanName = fileData.originalname.replace(/[^a-zA-Z0-9\-_.]/g, '');
  const fileName = `${Date.now()}-${cleanName}`;
  const filePath = path.join(imagesDir, fileName);

  // 写入文件
  fs.writeFileSync(filePath, Buffer.from(fileData.buffer));

  // 返回协议路径（相对日记目录）
  return `diary://images/${fileName}`; // 示例：diary://images/1234567890-photo.jpg
});

// 添加获取当前日记目录的辅助函数
function getCurrentDiaryDir() {
    const userDataPath = app.getPath('userData');
    const configPath = path.join(userDataPath, 'config.json');

    if (fs.existsSync(configPath)) {
        const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));
        if (config.diaryDir) return config.diaryDir;
    }

    return path.join(userDataPath, 'diary');
}