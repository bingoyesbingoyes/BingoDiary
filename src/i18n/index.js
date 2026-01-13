// Lightweight i18n system for BingoDiary
// Supports English (en) and Chinese (zh)

const translations = {
  en: {
    // App
    'app.title': 'Bingo Diary',

    // Common
    'common.save': 'Save',
    'common.cancel': 'Cancel',
    'common.delete': 'Delete',
    'common.edit': 'Edit',
    'common.add': 'Add',
    'common.close': 'Close',
    'common.done': 'Done',
    'common.today': 'Today',
    'common.yesterday': 'Yesterday',

    // Settings Modal
    'settings.title': 'Settings',
    'settings.tabs.general': 'General',
    'settings.tabs.sync': 'Sync',
    'settings.tabs.shortcuts': 'Shortcuts',

    // General Settings
    'settings.language': 'Language',
    'settings.storageLocation': 'Storage Location',
    'settings.background': 'Background',
    'settings.bg.presets': 'Presets',
    'settings.bg.customColor': 'Custom Color',
    'settings.bg.image': 'Image',
    'settings.selectImage': 'Select Image',
    'settings.fitMode': 'Fit Mode:',
    'settings.fitMode.cover': 'Cover',
    'settings.fitMode.contain': 'Contain',
    'settings.fitMode.stretch': 'Stretch',
    'settings.fitMode.original': 'Original',
    'settings.opacity': 'Opacity:',
    'settings.panelTransparency': 'Panel Transparency:',
    'settings.window': 'Window',
    'settings.width': 'Width',
    'settings.height': 'Height',
    'settings.apply': 'Apply',
    'settings.appPassword': 'App Password',
    'settings.enablePassword': 'Enable startup password',
    'settings.newPassword': 'New password',
    'settings.confirmPassword': 'Confirm password',
    'settings.setPassword': 'Set Password',
    'settings.save': 'Save',

    // Preset themes
    'presets.blue': 'Blue',
    'presets.green': 'Green',
    'presets.purple': 'Purple',
    'presets.orange': 'Orange',
    'presets.gray': 'Gray',
    'presets.teal': 'Teal',

    // Shortcuts
    'shortcuts.navigation': 'Navigation',
    'shortcuts.goToToday': 'Go to Today',
    'shortcuts.focusSearch': 'Focus Search',
    'shortcuts.openSettings': 'Open Settings',
    'shortcuts.editor': 'Editor',
    'shortcuts.save': 'Save',
    'shortcuts.undo': 'Undo',
    'shortcuts.bold': 'Bold',
    'shortcuts.italic': 'Italic',
    'shortcuts.insertLink': 'Insert Link',
    'shortcuts.view': 'View',
    'shortcuts.togglePreview': 'Toggle Preview',
    'shortcuts.splitMode': 'Split Mode',
    'shortcuts.exitClear': 'Exit / Clear',

    // Titlebar & Menu
    'titlebar.close': 'Close',
    'titlebar.minimize': 'Minimize',
    'titlebar.maximize': 'Maximize',
    'titlebar.sync': 'Sync',
    'titlebar.menu': 'Menu',
    'titlebar.settings': 'Settings',
    'menu.export': 'Export',
    'menu.json': 'JSON',
    'menu.pdf': 'PDF',
    'menu.markdown': 'Markdown',
    'menu.import': 'Import',
    'menu.fromJson': 'From JSON',

    // Drawer & Navigation
    'drawer.calendar': 'Calendar',
    'drawer.tags': 'Tags',
    'drawer.schedule': 'Schedule',
    'nav.calendar': 'Calendar',
    'nav.tags': 'Tags',
    'nav.settings': 'Settings',

    // Search
    'search.placeholder': 'Search...',
    'search.noDiariesYet': 'No diaries yet, write one!',
    'search.noMatchingDiaries': 'No matching diaries found',

    // Tags
    'tags.entriesCount': '{count} entries',
    'tags.clear': 'Clear',
    'tags.selectTag': 'Select a tag',
    'tags.noTags': 'No tags',
    'tags.addTags': 'Add tags',
    'tags.noTagsHint': 'No tags available. Create some in tag manager.',
    'tagManager.title': 'Manage Tags',
    'tagManager.manage': 'Manage',
    'tagManager.statistics': 'Statistics',
    'tagManager.addNewTag': 'Add New Tag',
    'tagManager.tagName': 'Tag name',
    'tagManager.existingTags': 'Existing Tags',
    'tagManager.noTagsYet': 'No tags yet',
    'tagManager.deleteConfirm': 'Delete tag "{name}"?',
    'tagManager.deleteWarning': 'This will remove the tag from all diary entries.',
    'tagManager.noStats': 'No tags to show statistics for.',
    'tagManager.diary': 'diary',
    'tagManager.last': 'Last:',
    'tagBar.all': 'All',
    'tagBar.manageTags': 'Manage Tags',

    // Schedule & Events
    'schedule.selectedDate': 'Selected Date',
    'schedule.add': 'Add',
    'schedule.noSchedules': 'No schedules for this day',
    'event.editEvent': 'Edit Event',
    'event.newEvent': 'New Event',
    'event.alarm': 'Alarm',
    'event.minBefore': 'min before',
    'event.notification': 'Notification',
    'event.sound': 'Sound',
    'event.both': 'Both',
    'event.newCategory': 'New category',
    'event.event': 'Event',

    // Sync
    'sync.googleCloudSetup': 'Google Cloud Setup',
    'sync.setupInfo': 'To enable sync, you need to create a Google Cloud project and get credentials.',
    'sync.step1': 'Go to',
    'sync.step2': 'Create a new project or select existing',
    'sync.step3': 'Enable "Google Drive API"',
    'sync.step4': 'Go to "APIs & Services" -> "Credentials"',
    'sync.step5': 'Create "OAuth 2.0 Client ID" (Desktop app)',
    'sync.step6': 'Copy Client ID and Client Secret below',
    'sync.clientId': 'Client ID',
    'sync.clientSecretHint': 'Client Secret (required for desktop, optional for mobile)',
    'sync.connectionSuccess': 'Successfully connected to Google Account!',
    'sync.googleAccount': 'Google Account',
    'sync.connected': 'Connected',
    'sync.disconnect': 'Disconnect',
    'sync.connectHint': 'Connect your Google account to sync diary data across devices.',
    'sync.connecting': 'Connecting...',
    'sync.connectGoogleAccount': 'Connect Google Account',
    'sync.fallbackHint': 'Cannot auto-open browser. Please click the link below or copy to browser:',
    'sync.clickToLogin': 'Click here to login Google',
    'sync.copyLink': 'Copy Link',
    'sync.codeHint': 'After logging in, browser will show "cannot access".<br>Copy the full URL from address bar and paste below:',
    'sync.pasteCodePlaceholder': 'Paste link or auth code',
    'sync.verifying': 'Verifying...',
    'sync.submit': 'Submit',
    'sync.manualSync': 'Sync',
    'sync.lastSync': 'Last sync',
    'sync.neverSynced': 'Never synced',
    'sync.syncing': 'Syncing...',
    'sync.syncNow': 'Sync Now',
    'sync.uploaded': 'Uploaded',
    'sync.downloaded': 'Downloaded',
    'sync.files': 'files',
    'sync.upToDate': 'Everything is up to date!',
    'sync.stageConnecting': 'Connecting',
    'sync.stageDiaryEntries': 'Diary Entries',
    'sync.stageTags': 'Tags',
    'sync.stageImages': 'Images',
    'sync.advancedSettings': 'Advanced Settings',
    'sync.currentClientId': 'Current Client ID',
    'sync.changeClientId': 'Change Client ID',
    'sync.linkCopied': 'Link copied to clipboard',
    'sync.authFailed': 'Authentication failed',
    'sync.userCancelled': 'Authorization cancelled by user',
    'sync.networkError': 'Network error, please check your connection',
    'sync.deepLinkWaiting': 'Waiting for authorization...',
    'sync.deepLinkTimeout': 'Authorization timeout, please try again or enter code manually',
    'sync.forceUpload': 'Force Upload',
    'sync.forceUploadHint': 'Upload all local files to cloud, overwriting remote versions',
    'sync.forceUploadConfirm': 'This will overwrite all cloud data with local data. Are you sure?',

    // Time
    'time.justNow': 'Just now',
    'time.minutesAgo': '{count} minutes ago',
    'time.hoursAgo': '{count} hours ago',

    // Lock Screen
    'lock.title': 'Bingo Diary',
    'lock.enterPassword': 'Enter password',
    'lock.unlock': 'Unlock',

    // Editor
    'editor.placeholder': 'Write your diary here...',
    'editor.uploadImage': 'Upload Image',
    'editor.preview': 'Preview',
    'editor.edit': 'Edit',

    // Theme
    'theme.darkMode': 'Dark Mode',
    'theme.lightMode': 'Light Mode',

    // Export
    'export.exporting': 'Exporting {type}...',
    'export.rangeExport': 'Range Export',
    'export.thisWeek': 'This Week',
    'export.thisMonth': 'This Month',
    'export.startDate': 'Start Date',
    'export.endDate': 'End Date',
    'export.format': 'Format',
    'export.export': 'Export',
    'export.entriesInRange': '{count} entries in range',
    'export.noEntriesInRange': 'No entries in selected range',

    // Alerts
    'alert.settingsSaved': 'Settings saved!',
    'alert.storageChanged': 'Storage location changed. Please restart the app.',
    'alert.passwordRequired': 'Please enter a password',
    'alert.passwordMismatch': 'Passwords do not match',
    'alert.passwordSet': 'Password set successfully!',
    'alert.incorrectPassword': 'Incorrect password',
    'alert.pdfExportSuccess': 'PDF exported successfully!',
    'alert.markdownExportSuccess': 'Markdown exported successfully!',
    'alert.markdownExportImages': 'Markdown exported successfully!\n{count} images copied to ./images/',
    'alert.linkCopied': 'Link copied to clipboard',
    'alert.authFailed': 'Authentication failed: {error}',
    'alert.exportJsonFailed': 'Failed to export JSON: {error}',
    'alert.exportPdfFailed': 'Failed to export PDF: {error}',
    'alert.exportMarkdownFailed': 'Failed to export Markdown: {error}',
  },

  zh: {
    // App
    'app.title': '冰果日记',

    // Common
    'common.save': '保存',
    'common.cancel': '取消',
    'common.delete': '删除',
    'common.edit': '编辑',
    'common.add': '添加',
    'common.close': '关闭',
    'common.done': '完成',
    'common.today': '今天',
    'common.yesterday': '昨天',

    // Settings Modal
    'settings.title': '设置',
    'settings.tabs.general': '通用',
    'settings.tabs.sync': '同步',
    'settings.tabs.shortcuts': '快捷键',

    // General Settings
    'settings.language': '语言',
    'settings.storageLocation': '存储位置',
    'settings.background': '背景',
    'settings.bg.presets': '预设',
    'settings.bg.customColor': '自定义颜色',
    'settings.bg.image': '图片',
    'settings.selectImage': '选择图片',
    'settings.fitMode': '适应方式：',
    'settings.fitMode.cover': '覆盖',
    'settings.fitMode.contain': '包含',
    'settings.fitMode.stretch': '拉伸',
    'settings.fitMode.original': '原始',
    'settings.opacity': '不透明度：',
    'settings.panelTransparency': '面板透明度：',
    'settings.window': '窗口',
    'settings.width': '宽度',
    'settings.height': '高度',
    'settings.apply': '应用',
    'settings.appPassword': '应用密码',
    'settings.enablePassword': '启用启动密码',
    'settings.newPassword': '新密码',
    'settings.confirmPassword': '确认密码',
    'settings.setPassword': '设置密码',
    'settings.save': '保存',

    // Preset themes
    'presets.blue': '蓝色',
    'presets.green': '绿色',
    'presets.purple': '紫色',
    'presets.orange': '橙色',
    'presets.gray': '灰色',
    'presets.teal': '青色',

    // Shortcuts
    'shortcuts.navigation': '导航',
    'shortcuts.goToToday': '跳转到今天',
    'shortcuts.focusSearch': '聚焦搜索',
    'shortcuts.openSettings': '打开设置',
    'shortcuts.editor': '编辑器',
    'shortcuts.save': '保存',
    'shortcuts.undo': '撤销',
    'shortcuts.bold': '粗体',
    'shortcuts.italic': '斜体',
    'shortcuts.insertLink': '插入链接',
    'shortcuts.view': '视图',
    'shortcuts.togglePreview': '切换预览',
    'shortcuts.splitMode': '分屏模式',
    'shortcuts.exitClear': '退出 / 清除',

    // Titlebar & Menu
    'titlebar.close': '关闭',
    'titlebar.minimize': '最小化',
    'titlebar.maximize': '最大化',
    'titlebar.sync': '同步',
    'titlebar.menu': '菜单',
    'titlebar.settings': '设置',
    'menu.export': '导出',
    'menu.json': 'JSON',
    'menu.pdf': 'PDF',
    'menu.markdown': 'Markdown',
    'menu.import': '导入',
    'menu.fromJson': '从 JSON 导入',

    // Drawer & Navigation
    'drawer.calendar': '日历',
    'drawer.tags': '标签',
    'drawer.schedule': '日程',
    'nav.calendar': '日历',
    'nav.tags': '标签',
    'nav.settings': '设置',

    // Search
    'search.placeholder': '搜索...',
    'search.noDiariesYet': '还没有日记，写一篇吧！',
    'search.noMatchingDiaries': '没有找到匹配的日记',

    // Tags
    'tags.entriesCount': '{count} 篇日记',
    'tags.clear': '清除',
    'tags.selectTag': '选择标签',
    'tags.noTags': '无标签',
    'tags.addTags': '添加标签',
    'tags.noTagsHint': '没有可用标签。请在标签管理中创建。',
    'tagManager.title': '管理标签',
    'tagManager.manage': '管理',
    'tagManager.statistics': '统计',
    'tagManager.addNewTag': '添加新标签',
    'tagManager.tagName': '标签名称',
    'tagManager.existingTags': '已有标签',
    'tagManager.noTagsYet': '还没有标签',
    'tagManager.deleteConfirm': '删除标签 "{name}"？',
    'tagManager.deleteWarning': '这将从所有日记中移除该标签。',
    'tagManager.noStats': '没有标签可供统计。',
    'tagManager.diary': '篇日记',
    'tagManager.last': '最近：',
    'tagBar.all': '全部',
    'tagBar.manageTags': '管理标签',

    // Schedule & Events
    'schedule.selectedDate': '选中日期',
    'schedule.add': '添加',
    'schedule.noSchedules': '今天没有日程',
    'event.editEvent': '编辑事件',
    'event.newEvent': '新建事件',
    'event.alarm': '提醒',
    'event.minBefore': '分钟前',
    'event.notification': '通知',
    'event.sound': '声音',
    'event.both': '两者',
    'event.newCategory': '新分类',
    'event.event': '事件',

    // Sync
    'sync.googleCloudSetup': 'Google Cloud 设置',
    'sync.setupInfo': '要启用同步，需要创建 Google Cloud 项目并获取凭据。',
    'sync.step1': '前往',
    'sync.step2': '创建新项目或选择现有项目',
    'sync.step3': '启用 "Google Drive API"',
    'sync.step4': '前往 "APIs & Services" -> "Credentials"',
    'sync.step5': '创建 "OAuth 2.0 Client ID"（桌面应用）',
    'sync.step6': '复制 Client ID 和 Client Secret 到下方',
    'sync.clientId': 'Client ID',
    'sync.clientSecretHint': 'Client Secret（桌面端必填，移动端可选）',
    'sync.connectionSuccess': '已成功连接到 Google 账户！',
    'sync.googleAccount': 'Google 账户',
    'sync.connected': '已连接',
    'sync.disconnect': '断开连接',
    'sync.connectHint': '连接 Google 账户以在多设备间同步日记数据。',
    'sync.connecting': '连接中...',
    'sync.connectGoogleAccount': '连接 Google 账户',
    'sync.fallbackHint': '无法自动打开浏览器，请点击下方链接或复制到浏览器中打开：',
    'sync.clickToLogin': '点击这里登录 Google',
    'sync.copyLink': '复制链接',
    'sync.codeHint': '登录 Google 后，浏览器会显示"无法访问此网站"。<br>请复制地址栏中的完整链接，粘贴到下方：',
    'sync.pasteCodePlaceholder': '粘贴链接或授权码',
    'sync.verifying': '验证中...',
    'sync.submit': '提交',
    'sync.manualSync': '同步',
    'sync.lastSync': '上次同步',
    'sync.neverSynced': '从未同步',
    'sync.syncing': '同步中...',
    'sync.syncNow': '立即同步',
    'sync.uploaded': '已上传',
    'sync.downloaded': '已下载',
    'sync.files': '个文件',
    'sync.upToDate': '一切都是最新的！',
    'sync.stageConnecting': '连接中',
    'sync.stageDiaryEntries': '日记条目',
    'sync.stageTags': '标签',
    'sync.stageImages': '图片',
    'sync.advancedSettings': '高级设置',
    'sync.currentClientId': '当前 Client ID',
    'sync.changeClientId': '更改 Client ID',
    'sync.linkCopied': '链接已复制到剪贴板',
    'sync.authFailed': '认证失败',
    'sync.forceUpload': '强制上传',
    'sync.forceUploadHint': '将所有本地文件上传到云端，覆盖云端版本',
    'sync.forceUploadConfirm': '这将用本地数据覆盖所有云端数据。确定要继续吗？',

    // Time
    'time.justNow': '刚刚',
    'time.minutesAgo': '{count} 分钟前',
    'time.hoursAgo': '{count} 小时前',

    // Lock Screen
    'lock.title': '冰果日记',
    'lock.enterPassword': '输入密码',
    'lock.unlock': '解锁',

    // Editor
    'editor.placeholder': '在这里写日记...',
    'editor.uploadImage': '上传图片',
    'editor.preview': '预览',
    'editor.edit': '编辑',

    // Theme
    'theme.darkMode': '深色模式',
    'theme.lightMode': '浅色模式',

    // Export
    'export.exporting': '正在导出 {type}...',
    'export.rangeExport': '按范围导出',
    'export.thisWeek': '本周',
    'export.thisMonth': '本月',
    'export.startDate': '开始日期',
    'export.endDate': '结束日期',
    'export.format': '格式',
    'export.export': '导出',
    'export.entriesInRange': '范围内有 {count} 篇日记',
    'export.noEntriesInRange': '所选范围内没有日记',

    // Alerts
    'alert.settingsSaved': '设置已保存！',
    'alert.storageChanged': '存储位置已更改。请重启应用。',
    'alert.passwordRequired': '请输入密码',
    'alert.passwordMismatch': '两次密码不一致',
    'alert.passwordSet': '密码设置成功！',
    'alert.incorrectPassword': '密码错误',
    'alert.pdfExportSuccess': 'PDF 导出成功！',
    'alert.markdownExportSuccess': 'Markdown 导出成功！',
    'alert.markdownExportImages': 'Markdown 导出成功！\n{count} 张图片已复制到 ./images/',
    'alert.linkCopied': '链接已复制到剪贴板',
    'alert.authFailed': '认证失败：{error}',
    'alert.exportJsonFailed': 'JSON 导出失败：{error}',
    'alert.exportPdfFailed': 'PDF 导出失败：{error}',
    'alert.exportMarkdownFailed': 'Markdown 导出失败：{error}',
  }
};

let currentLang = 'en';

/**
 * Set the current language
 * @param {string} lang - Language code ('en' or 'zh')
 */
export function setLanguage(lang) {
  if (translations[lang]) {
    currentLang = lang;
    localStorage.setItem('app-language', lang);
  }
}

/**
 * Get the current language
 * @returns {string} Current language code
 */
export function getLanguage() {
  return currentLang;
}

/**
 * Translate a key to the current language
 * @param {string} key - Translation key
 * @param {object} params - Optional parameters for interpolation
 * @returns {string} Translated string
 */
export function t(key, params = {}) {
  let text = translations[currentLang]?.[key] || translations['en']?.[key] || key;

  // Handle parameter interpolation: {count}, {name}, etc.
  Object.keys(params).forEach(param => {
    text = text.replace(new RegExp(`\\{${param}\\}`, 'g'), params[param]);
  });

  return text;
}

/**
 * Initialize language from saved preference or config
 * @param {string} savedLang - Saved language preference
 */
export function initLanguage(savedLang) {
  const lang = savedLang || localStorage.getItem('app-language') || 'en';
  if (translations[lang]) {
    currentLang = lang;
  }
}

/**
 * Get all available languages
 * @returns {Array} Array of {code, name} objects
 */
export function getAvailableLanguages() {
  return [
    { code: 'en', name: 'English' },
    { code: 'zh', name: '中文' }
  ];
}

export default { t, setLanguage, getLanguage, initLanguage, getAvailableLanguages };
