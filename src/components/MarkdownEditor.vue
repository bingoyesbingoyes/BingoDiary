<template>
    <div class="markdown-editor" :class="{ 'dark-theme': theme === 'dark' }">
        <!-- Source mode: plain textarea -->
        <textarea
            v-if="isSourceMode"
            ref="textareaRef"
            :value="modelValue"
            @input="handleInput"
            @paste="handlePaste"
            class="markdown-textarea"
            placeholder="Write your diary here..."
        ></textarea>

        <!-- Render mode: TipTap with preview -->
        <div v-else class="markdown-preview" ref="previewRef" @click="handlePreviewClick" v-html="renderedHtml"></div>

        <!-- Top-right toolbar for mobile (image upload) -->
        <div v-if="isMobile && isSourceMode" class="top-toolbar">
            <button @click="uploadImage" class="toolbar-btn" title="Upload Image">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <rect x="3" y="3" width="18" height="18" rx="2"/>
                    <circle cx="8.5" cy="8.5" r="1.5"/>
                    <path d="M21 15l-5-5L5 21"/>
                </svg>
            </button>
        </div>

        <!-- Bottom-right toggle button -->
        <button
            v-if="!hideToggle"
            @click="toggleMode"
            class="mode-toggle-btn"
            :title="isSourceMode ? 'Preview (⌘P)' : 'Edit (⌘P)'"
        >
            {{ isSourceMode ? '◉' : '✎' }}
        </button>

        <!-- Image Lightbox -->
        <Teleport to="body">
            <div v-if="lightboxVisible" class="image-lightbox" @click="closeLightbox">
                <div class="lightbox-content" @click.stop>
                    <img :src="lightboxSrc" alt="Preview" />
                    <div class="lightbox-toolbar">
                        <button @click="saveImage" class="lightbox-btn">
                            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                                <polyline points="7 10 12 15 17 10"/>
                                <line x1="12" y1="15" x2="12" y2="3"/>
                            </svg>
                        </button>
                        <button @click="closeLightbox" class="lightbox-btn">
                            <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="18" y1="6" x2="6" y2="18"/>
                                <line x1="6" y1="6" x2="18" y2="18"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </Teleport>
    </div>
</template>

<script setup>
import { ref, computed, nextTick, watch, onMounted, onUnmounted } from 'vue';
import { marked } from 'marked';
import katex from 'katex';
import 'katex/dist/katex.min.css';
import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';
import { platform } from '@tauri-apps/plugin-os';

// Platform detection
const isMobile = ref(false);

const detectPlatform = async () => {
    try {
        const p = await platform();
        isMobile.value = p === 'android' || p === 'ios';
    } catch (e) {
        isMobile.value = false;
    }
};

// Expose preview element ref for PDF export
const previewRef = ref(null);
const textareaRef = ref(null);

const props = defineProps({
    modelValue: {
        type: String,
        default: ''
    },
    theme: {
        type: String,
        default: 'light'
    },
    hideToggle: {
        type: Boolean,
        default: false
    },
    diaryDir: {
        type: String,
        default: ''
    }
});

const emit = defineEmits(['update:modelValue']);

const isSourceMode = ref(true);

// Lightbox state
const lightboxVisible = ref(false);
const lightboxSrc = ref('');
const lightboxOriginalPath = ref('');

// Undo/Redo history
const history = ref([]);
const historyIndex = ref(-1);
const isUndoRedo = ref(false);
const maxHistory = 100;

// Save state to history
const saveToHistory = (text) => {
    if (isUndoRedo.value) return;

    // Remove any redo states
    if (historyIndex.value < history.value.length - 1) {
        history.value = history.value.slice(0, historyIndex.value + 1);
    }

    // Don't save if same as last state
    if (history.value.length > 0 && history.value[history.value.length - 1] === text) {
        return;
    }

    history.value.push(text);
    if (history.value.length > maxHistory) {
        history.value.shift();
    }
    historyIndex.value = history.value.length - 1;
};

// Undo action
const undo = () => {
    if (historyIndex.value > 0) {
        isUndoRedo.value = true;
        historyIndex.value--;
        emit('update:modelValue', history.value[historyIndex.value]);
        nextTick(() => {
            isUndoRedo.value = false;
        });
    }
};

// Redo action
const redo = () => {
    if (historyIndex.value < history.value.length - 1) {
        isUndoRedo.value = true;
        historyIndex.value++;
        emit('update:modelValue', history.value[historyIndex.value]);
        nextTick(() => {
            isUndoRedo.value = false;
        });
    }
};

const handleInput = (event) => {
    const newValue = event.target.value;
    emit('update:modelValue', newValue);
    saveToHistory(newValue);
};

const toggleMode = () => {
    isSourceMode.value = !isSourceMode.value;
};

// Wrap selected text with markdown syntax
const wrapSelection = (before, after) => {
    const textarea = textareaRef.value;
    if (!textarea || !isSourceMode.value) return;

    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const text = props.modelValue;
    const selected = text.substring(start, end);

    const newText = text.substring(0, start) + before + selected + after + text.substring(end);
    emit('update:modelValue', newText);

    // Restore cursor position
    nextTick(() => {
        textarea.focus();
        if (selected) {
            textarea.setSelectionRange(start + before.length, end + before.length);
        } else {
            textarea.setSelectionRange(start + before.length, start + before.length);
        }
    });
};

// Insert link markdown
const insertLink = () => {
    const textarea = textareaRef.value;
    if (!textarea || !isSourceMode.value) return;

    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const text = props.modelValue;
    const selected = text.substring(start, end);

    const linkText = selected || 'link text';
    const newText = text.substring(0, start) + `[${linkText}](url)` + text.substring(end);
    emit('update:modelValue', newText);

    nextTick(() => {
        textarea.focus();
        // Select "url" for easy replacement
        const urlStart = start + linkText.length + 3;
        textarea.setSelectionRange(urlStart, urlStart + 3);
    });
};

// Handle keyboard shortcuts
const handleKeydown = (e) => {
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    const cmdKey = isMac ? e.metaKey : e.ctrlKey;

    // Cmd+Z - Undo
    if (cmdKey && e.key === 'z') {
        e.preventDefault();
        undo();
        return;
    }

    // Cmd+P - Toggle preview mode
    if (cmdKey && e.key === 'p' && !e.shiftKey) {
        e.preventDefault();
        toggleMode();
        return;
    }

    // Cmd+B - Bold
    if (cmdKey && e.key === 'b') {
        e.preventDefault();
        wrapSelection('**', '**');
        return;
    }

    // Cmd+I - Italic
    if (cmdKey && e.key === 'i') {
        e.preventDefault();
        wrapSelection('*', '*');
        return;
    }

    // Cmd+K - Insert link
    if (cmdKey && e.key === 'k') {
        e.preventDefault();
        insertLink();
        return;
    }
};

// Initialize history with initial value
const initHistory = () => {
    history.value = [props.modelValue];
    historyIndex.value = 0;
};

// Watch for external changes (like switching dates)
watch(() => props.modelValue, (newVal, oldVal) => {
    // If this is an external change (not from undo/redo or input)
    if (!isUndoRedo.value && history.value[historyIndex.value] !== newVal) {
        // Check if it's a completely new document (date switch)
        const isNewDoc = !oldVal || (newVal.split('\n')[0] !== oldVal.split('\n')[0]);
        if (isNewDoc) {
            initHistory();
        }
    }
});

// Force source mode when hideToggle is true (split mode)
watch(() => props.hideToggle, (hidden) => {
    if (hidden) {
        isSourceMode.value = true;
    }
});

// Setup keyboard listeners
onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
    initHistory();
    detectPlatform();
});

onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown);
});

// Expose methods for parent component
defineExpose({
    toggleMode,
    isSourceMode,
    textareaRef
});

// Handle paste event for images
const handlePaste = async (event) => {
    const items = event.clipboardData?.items;
    if (!items) return;

    for (const item of items) {
        if (item.type.startsWith('image/')) {
            event.preventDefault();

            const file = item.getAsFile();
            if (!file) continue;

            try {
                // Read file as array buffer
                const arrayBuffer = await file.arrayBuffer();
                const uint8Array = new Uint8Array(arrayBuffer);

                // Generate filename with extension
                const ext = file.type.split('/')[1] || 'png';
                const filename = `paste-${Date.now()}.${ext}`;

                // Save image via Tauri backend
                const savedPath = await invoke('save_image', {
                    filename: filename,
                    data: Array.from(uint8Array)
                });

                // Extract just the filename from the saved path for cross-device compatibility
                const imageFilename = savedPath.split('/').pop().split('\\').pop();
                console.log('[Image Paste] savedPath:', savedPath, 'filename:', imageFilename);

                // Insert markdown image syntax at cursor position
                const textarea = textareaRef.value;
                const cursorPos = textarea?.selectionStart || props.modelValue.length;
                const textBefore = props.modelValue.substring(0, cursorPos);
                const textAfter = props.modelValue.substring(cursorPos);

                // Use relative path for cross-device sync compatibility
                const imageMarkdown = `![image](images/${imageFilename})\n`;
                const newText = textBefore + imageMarkdown + textAfter;

                emit('update:modelValue', newText);

                // Move cursor after the inserted image
                setTimeout(() => {
                    if (textarea) {
                        const newPos = cursorPos + imageMarkdown.length;
                        textarea.selectionStart = newPos;
                        textarea.selectionEnd = newPos;
                        textarea.focus();
                    }
                }, 0);

            } catch (error) {
                console.error('Failed to save pasted image:', error);
                alert('Failed to save image: ' + error);
            }

            break; // Only handle the first image
        }
    }
};

// Upload image from file picker (for mobile)
const uploadImage = async () => {
    try {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const { readFile } = await import('@tauri-apps/plugin-fs');

        const filePath = await open({
            title: 'Select an image',
            filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp'] }]
        });

        if (!filePath) return;

        // Read the file
        const data = await readFile(filePath);

        // Extract extension from path
        let ext = 'png';
        const pathStr = String(filePath);
        if (pathStr.includes('.')) {
            const parts = pathStr.split('.');
            const lastPart = parts[parts.length - 1].toLowerCase();
            if (['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp'].includes(lastPart)) {
                ext = lastPart;
            }
        }

        // Generate filename
        const filename = `upload-${Date.now()}.${ext}`;

        // Save image via Tauri backend
        const savedPath = await invoke('save_image', {
            filename: filename,
            data: Array.from(data)
        });

        // Extract just the filename from the saved path for cross-device compatibility
        const imageFilename = savedPath.split('/').pop().split('\\').pop();

        // Insert markdown image syntax at cursor position
        const textarea = textareaRef.value;
        const cursorPos = textarea?.selectionStart || props.modelValue.length;
        const textBefore = props.modelValue.substring(0, cursorPos);
        const textAfter = props.modelValue.substring(cursorPos);

        // Use relative path for cross-device sync compatibility
        const imageMarkdown = `![image](images/${imageFilename})\n`;
        const newText = textBefore + imageMarkdown + textAfter;

        emit('update:modelValue', newText);
        saveToHistory(newText);

        // Move cursor after the inserted image
        nextTick(() => {
            if (textarea) {
                const newPos = cursorPos + imageMarkdown.length;
                textarea.selectionStart = newPos;
                textarea.selectionEnd = newPos;
                textarea.focus();
            }
        });

    } catch (error) {
        console.error('Failed to upload image:', error);
        alert('Failed to upload image: ' + error);
    }
};

// Handle click on preview to open lightbox for images
const handlePreviewClick = (event) => {
    const target = event.target;
    if (target.tagName === 'IMG') {
        lightboxSrc.value = target.src;
        // Store original path for saving (extract from asset URL or use src directly)
        lightboxOriginalPath.value = target.src;
        lightboxVisible.value = true;
    }
};

// Close lightbox
const closeLightbox = () => {
    lightboxVisible.value = false;
    lightboxSrc.value = '';
    lightboxOriginalPath.value = '';
};

// Save image to device
const saveImage = async () => {
    try {
        const { save } = await import('@tauri-apps/plugin-dialog');

        // Fetch image data
        const response = await fetch(lightboxSrc.value);
        const blob = await response.blob();
        const arrayBuffer = await blob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);

        // Determine file extension
        let ext = 'png';
        const mimeType = blob.type;
        if (mimeType.includes('jpeg') || mimeType.includes('jpg')) ext = 'jpg';
        else if (mimeType.includes('gif')) ext = 'gif';
        else if (mimeType.includes('webp')) ext = 'webp';

        // Open save dialog
        const filePath = await save({
            title: 'Save Image',
            defaultPath: `image-${Date.now()}.${ext}`,
            filters: [{ name: 'Images', extensions: [ext] }]
        });

        if (filePath) {
            const { writeFile } = await import('@tauri-apps/plugin-fs');
            await writeFile(filePath, uint8Array);
            closeLightbox();
        }
    } catch (error) {
        console.error('Failed to save image:', error);
        alert('Failed to save image: ' + error);
    }
};

// Configure marked for better rendering
marked.setOptions({
    breaks: true,
    gfm: true,
});

const renderedHtml = computed(() => {
    if (!props.modelValue) return '';

    // Process LaTeX math formulas before markdown rendering
    let content = props.modelValue;

    // Store math expressions with placeholders
    const mathExpressions = [];
    let mathIndex = 0;

    // Replace display math $$ ... $$
    content = content.replace(/\$\$([\s\S]+?)\$\$/g, (_, formula) => {
        try {
            const rendered = katex.renderToString(formula.trim(), {
                displayMode: true,
                throwOnError: false
            });
            const placeholder = `<span id="math-placeholder-${mathIndex}" style="display:none;"></span>`;
            mathExpressions.push({ id: mathIndex, html: rendered, isBlock: true });
            mathIndex++;
            return placeholder;
        } catch (e) {
            return `<div class="math-error">Math Error: ${e.message}</div>`;
        }
    });

    // Replace inline math $ ... $
    content = content.replace(/\$([^\$\n]+?)\$/g, (_, formula) => {
        try {
            const rendered = katex.renderToString(formula.trim(), {
                displayMode: false,
                throwOnError: false
            });
            const placeholder = `<span id="math-placeholder-${mathIndex}" style="display:none;"></span>`;
            mathExpressions.push({ id: mathIndex, html: rendered, isBlock: false });
            mathIndex++;
            return placeholder;
        } catch (e) {
            return `<span class="math-error">Math Error: ${e.message}</span>`;
        }
    });

    // Convert image paths to Tauri asset URLs for preview
    // 1. Handle relative paths (images/xxx.png) - new format for cross-device sync
    if (props.diaryDir) {
        content = content.replace(/!\[([^\]]*)\]\((images\/[^)]+)\)/g, (match, alt, relativePath) => {
            const fullPath = `${props.diaryDir}/${relativePath}`;
            const assetUrl = convertFileSrc(fullPath);
            console.log('[Image Preview] relativePath:', relativePath, 'fullPath:', fullPath, 'assetUrl:', assetUrl);
            return `![${alt}](${assetUrl})`;
        });
    } else {
        console.log('[Image Preview] diaryDir is empty, cannot convert relative paths');
    }

    // 2. Handle absolute paths starting with / (legacy format, for backward compatibility)
    content = content.replace(/!\[([^\]]*)\]\((\/.+?)\)/g, (match, alt, path) => {
        const assetUrl = convertFileSrc(path);
        return `![${alt}](${assetUrl})`;
    });

    // Render markdown
    let html = marked(content);

    // Restore math expressions by replacing placeholders
    mathExpressions.forEach(({ id, html: mathHtml, isBlock }) => {
        const placeholderRegex = new RegExp(`<span id="math-placeholder-${id}"[^>]*></span>`, 'g');
        if (isBlock) {
            html = html.replace(placeholderRegex, `<div class="math-block">${mathHtml}</div>`);
        } else {
            html = html.replace(placeholderRegex, mathHtml);
        }
    });

    return html;
});
</script>

<style scoped lang="scss">
.markdown-editor {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: transparent;
    border-radius: 10px;
    position: relative;
    overflow: hidden;

    &.dark-theme {
        background: transparent;
    }
}

.markdown-textarea {
    width: 100%;
    height: 100%;
    padding: 2em 4em;
    border: none;
    outline: none;
    resize: none;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, "Menlo", "Monaco", monospace;
    font-size: 16px;
    line-height: 1.6;
    background: transparent;
    color: #2e3440;
    cursor: text;
    pointer-events: auto;
    z-index: 1;
    user-select: text;
    -webkit-user-select: text;
    -moz-user-select: text;
    -ms-user-select: text;

    &::placeholder {
        color: #999;
        opacity: 0.6;
    }

    .dark-theme & {
        color: #d8dee9;
    }
}

.markdown-preview {
    width: 100%;
    height: 100%;
    padding: 2em 4em;
    overflow-y: auto;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 1.6;
    color: #2e3440;
    user-select: text;
    -webkit-user-select: text;
    -moz-user-select: text;
    -ms-user-select: text;
    cursor: text;
    pointer-events: auto;
    z-index: 1;

    :deep(h1) {
        font-size: 2em;
        margin: 0.67em 0;
        font-weight: 600;
        border-bottom: 2px solid #eee;
        padding-bottom: 0.3em;
    }

    :deep(h2) {
        font-size: 1.5em;
        margin: 0.75em 0;
        font-weight: 600;
        border-bottom: 1px solid #eee;
        padding-bottom: 0.3em;
    }

    :deep(h3) {
        font-size: 1.25em;
        margin: 1em 0;
        font-weight: 600;
    }

    :deep(p) { margin: 1em 0; }
    :deep(strong) { font-weight: 600; }
    :deep(em) { font-style: italic; }

    :deep(code) {
        background: rgba(135, 131, 120, 0.15);
        color: #eb5757;
        border-radius: 3px;
        padding: 0.2em 0.4em;
        font-family: "SF Mono", Monaco, "Menlo", monospace;
        font-size: 85%;
    }

    :deep(pre) {
        background: #f7f7f7;
        border-radius: 8px;
        padding: 1em;
        overflow-x: auto;
        margin: 1em 0;
    }

    :deep(pre code) {
        background: transparent;
        color: inherit;
        padding: 0;
    }

    :deep(blockquote) {
        border-left: 4px solid #ddd;
        padding-left: 1em;
        margin: 1em 0;
        color: #6c6c6c;
    }

    :deep(ul) {
        padding-left: 2em;
        margin: 1em 0;
        list-style-type: disc;
        list-style-position: outside;
    }

    :deep(ul ul) {
        list-style-type: circle;
        margin: 0.5em 0;
    }

    :deep(ul ul ul) {
        list-style-type: square;
    }

    :deep(ol) {
        padding-left: 2em;
        margin: 1em 0;
        list-style-type: decimal;
        list-style-position: outside;
    }

    :deep(li) {
        margin: 0.25em 0;
        display: list-item;
    }

    :deep(a) {
        color: #1976d2;
        text-decoration: none;

        &:hover {
            text-decoration: underline;
        }
    }

    :deep(hr) {
        border: none;
        border-top: 2px solid #eee;
        margin: 2em 0;
    }

    :deep(img) {
        max-width: 100%;
        height: auto;
        border-radius: 8px;
        margin: 1em 0;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        cursor: zoom-in;
        transition: transform 0.2s;

        &:active {
            transform: scale(0.98);
        }
    }

    :deep(.katex-display) { margin: 1em 0; }
    :deep(.math-block) {
        margin: 1em 0;
        text-align: center;
    }
    :deep(.math-error) {
        color: #d32f2f;
        background: rgba(211, 47, 47, 0.1);
        padding: 0.2em 0.5em;
        border-radius: 3px;
        font-size: 0.9em;
    }

    .dark-theme & {
        color: #d8dee9;

        :deep(pre) { background: #3b4252; }
        :deep(blockquote) {
            border-left-color: #4c566a;
            color: #a0a0a0;
        }
    }
}

/* Top-right toolbar for mobile */
.top-toolbar {
    position: absolute;
    top: 1em;
    right: 16px;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 8px;
    z-index: 10;
}

/* Bottom-right preview toggle */
.mode-toggle-btn {
    position: absolute;
    bottom: 1.5em;
    right: 16px;
    width: 36px;
    height: 36px;
    padding: 0;
    background: rgba(0, 0, 0, 0.06);
    border: none;
    border-radius: 10px;
    cursor: pointer;
    font-size: 20px;
    color: #666;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    z-index: 10;
    pointer-events: auto;

    &:hover {
        background: rgba(0, 0, 0, 0.12);
        color: #333;
    }

    .dark-theme & {
        background: rgba(255, 255, 255, 0.08);
        color: #a0a0a0;

        &:hover {
            background: rgba(255, 255, 255, 0.15);
            color: #d8dee9;
        }
    }
}

/* Scrollbar styling - subtle glass effect */
.markdown-textarea,
.markdown-preview {
    &::-webkit-scrollbar {
        width: 6px;
    }

    &::-webkit-scrollbar-track {
        background: transparent;
        margin: 8px 0;
    }

    &::-webkit-scrollbar-thumb {
        background: rgba(0, 0, 0, 0.08);
        border-radius: 12px;

        &:hover {
            background: rgba(0, 0, 0, 0.15);
        }
    }

    .dark-theme &::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.08);

        &:hover {
            background: rgba(255, 255, 255, 0.15);
        }
    }
}

/* Toolbar button (image upload etc.) */
.toolbar-btn {
    width: 36px;
    height: 36px;
    padding: 0;
    background: rgba(0, 0, 0, 0.06);
    border: none;
    border-radius: 10px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #666;
    transition: all 0.2s;
    pointer-events: auto;

    &:hover, &:active {
        background: rgba(0, 0, 0, 0.12);
        color: #333;
    }

    .dark-theme & {
        background: rgba(255, 255, 255, 0.08);
        color: #a0a0a0;

        &:hover, &:active {
            background: rgba(255, 255, 255, 0.15);
            color: #d8dee9;
        }
    }
}
</style>

<!-- Global styles for lightbox (not scoped) -->
<style lang="scss">
.image-lightbox {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.9);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 99999;
    padding: 20px;
    box-sizing: border-box;
}

.lightbox-content {
    position: relative;
    max-width: 90vw;
    max-height: 90vh;
    display: flex;
    flex-direction: column;
    align-items: center;

    img {
        max-width: 100%;
        max-height: calc(90vh - 60px);
        object-fit: contain;
        border-radius: 8px;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
    }
}

.lightbox-toolbar {
    display: flex;
    gap: 16px;
    margin-top: 16px;
}

.lightbox-btn {
    width: 48px;
    height: 48px;
    border: none;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.15);
    color: white;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;

    &:hover, &:active {
        background: rgba(255, 255, 255, 0.25);
        transform: scale(1.1);
    }
}
</style>
