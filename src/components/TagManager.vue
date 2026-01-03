<template>
    <div class="tag-manager-overlay" @click.self="$emit('close')">
        <div class="tag-manager-modal" :class="{ 'dark-mode': darkTheme }">
            <div class="modal-header">
                <h2>{{ t('tagManager.title') }}</h2>
                <button @click="$emit('close')" class="close-btn">&times;</button>
            </div>

            <!-- Tabs -->
            <div class="modal-tabs">
                <button
                    :class="{ active: activeTab === 'manage' }"
                    @click="activeTab = 'manage'"
                >
                    {{ t('tagManager.manage') }}
                </button>
                <button
                    :class="{ active: activeTab === 'stats' }"
                    @click="activeTab = 'stats'; loadStats()"
                >
                    {{ t('tagManager.statistics') }}
                </button>
            </div>

            <div class="modal-content">
                <!-- Manage Tab -->
                <div v-show="activeTab === 'manage'">
                <!-- Add New Tag -->
                <div class="add-tag-section">
                    <h3>{{ t('tagManager.addNewTag') }}</h3>
                    <div class="add-tag-form">
                        <input
                            type="text"
                            v-model="newTagName"
                            :placeholder="t('tagManager.tagName')"
                            class="tag-input"
                            @keyup.enter="createTag"
                        />
                        <div class="color-picker-wrapper">
                            <input
                                type="color"
                                v-model="newTagColor"
                                class="color-picker"
                            />
                        </div>
                        <button @click="createTag" class="add-btn" :disabled="!newTagName.trim()">
                            {{ t('common.add') }}
                        </button>
                    </div>
                    <p v-if="error" class="error-msg">{{ error }}</p>
                </div>

                <!-- Color Presets -->
                <div class="color-presets">
                    <button
                        v-for="color in presetColors"
                        :key="color"
                        class="preset-color"
                        :style="{ background: color }"
                        :class="{ selected: newTagColor === color }"
                        @click="newTagColor = color"
                    ></button>
                </div>

                <!-- Existing Tags -->
                <div class="tags-list-section" v-if="tags.length > 0">
                    <h3>{{ t('tagManager.existingTags') }}</h3>
                    <div class="tags-list">
                        <div v-for="tag in tags" :key="tag.id" class="tag-item">
                            <div v-if="editingTagId !== tag.id" class="tag-display">
                                <span class="tag-color" :style="{ background: tag.color }"></span>
                                <span class="tag-name">{{ tag.name }}</span>
                                <div class="tag-actions">
                                    <button @click="startEdit(tag)" class="edit-btn">{{ t('common.edit') }}</button>
                                    <button @click="confirmDelete(tag)" class="delete-btn">{{ t('common.delete') }}</button>
                                </div>
                            </div>
                            <div v-else class="tag-edit">
                                <input
                                    type="text"
                                    v-model="editTagName"
                                    class="tag-input small"
                                    @keyup.enter="saveEdit"
                                    @keyup.esc="cancelEdit"
                                />
                                <input
                                    type="color"
                                    v-model="editTagColor"
                                    class="color-picker small"
                                />
                                <button @click="saveEdit" class="save-btn">{{ t('common.save') }}</button>
                                <button @click="cancelEdit" class="cancel-btn">{{ t('common.cancel') }}</button>
                            </div>
                        </div>
                    </div>
                </div>

                <div v-else class="no-tags">
                    <p>{{ t('tagManager.noTagsYet') }}</p>
                </div>
                </div>

                <!-- Stats Tab -->
                <div v-show="activeTab === 'stats'">
                    <TagStats
                        :stats="tagStats"
                        :dark-theme="darkTheme"
                        :t="t"
                    />
                </div>
            </div>

            <!-- Delete Confirmation -->
            <div v-if="tagToDelete" class="delete-confirm-overlay" @click.self="tagToDelete = null">
                <div class="delete-confirm-modal">
                    <p>{{ t('tagManager.deleteConfirm', { name: tagToDelete.name }) }}</p>
                    <p class="delete-warning">{{ t('tagManager.deleteWarning') }}</p>
                    <div class="confirm-buttons">
                        <button @click="tagToDelete = null" class="cancel-btn">{{ t('common.cancel') }}</button>
                        <button @click="deleteTag" class="confirm-delete-btn">{{ t('common.delete') }}</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import TagStats from './TagStats.vue';

export default {
    name: 'TagManager',
    components: {
        TagStats
    },
    props: {
        tags: {
            type: Array,
            default: () => []
        },
        darkTheme: {
            type: Boolean,
            default: false
        },
        t: {
            type: Function,
            required: true
        }
    },
    emits: ['close', 'create', 'update', 'delete'],
    data() {
        return {
            activeTab: 'manage',
            newTagName: '',
            newTagColor: '#4A8B8B',
            editingTagId: null,
            editTagName: '',
            editTagColor: '',
            tagToDelete: null,
            error: '',
            tagStats: [],
            presetColors: [
                '#4A8B8B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FFEAA7',
                '#DDA0DD', '#F7DC6F', '#85C1E9'
            ]
        };
    },
    methods: {
        createTag() {
            if (!this.newTagName.trim()) return;

            this.error = '';
            this.$emit('create', {
                name: this.newTagName.trim(),
                color: this.newTagColor
            });
            this.newTagName = '';
        },
        startEdit(tag) {
            this.editingTagId = tag.id;
            this.editTagName = tag.name;
            this.editTagColor = tag.color;
        },
        saveEdit() {
            if (!this.editTagName.trim()) return;

            this.$emit('update', {
                id: this.editingTagId,
                name: this.editTagName.trim(),
                color: this.editTagColor
            });
            this.cancelEdit();
        },
        cancelEdit() {
            this.editingTagId = null;
            this.editTagName = '';
            this.editTagColor = '';
        },
        confirmDelete(tag) {
            this.tagToDelete = tag;
        },
        deleteTag() {
            this.$emit('delete', this.tagToDelete.id);
            this.tagToDelete = null;
        },
        setError(msg) {
            this.error = msg;
            setTimeout(() => { this.error = ''; }, 3000);
        },
        async loadStats() {
            try {
                this.tagStats = await invoke('get_tag_stats');
            } catch (error) {
                console.error('Failed to load tag stats:', error);
            }
        }
    }
}
</script>

<style lang="scss" scoped>
// Apple-style animation
@keyframes modalOverlayIn {
    from {
        opacity: 0;
        backdrop-filter: blur(0px);
    }
    to {
        opacity: 1;
        backdrop-filter: blur(20px);
    }
}

@keyframes modalScaleIn {
    from {
        opacity: 0;
        transform: scale(0.95) translateY(10px);
    }
    to {
        opacity: 1;
        transform: scale(1) translateY(0);
    }
}

.tag-manager-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1001;
    padding: 20px;
    padding-top: calc(20px + max(env(safe-area-inset-top, 0px), 28px));
    padding-bottom: calc(20px + max(env(safe-area-inset-bottom, 0px), 64px));
    box-sizing: border-box;
    animation: modalOverlayIn 0.35s cubic-bezier(0.32, 0.72, 0, 1);
}

.tag-manager-modal {
    width: calc(100% - 40px);
    max-width: 420px;
    max-height: calc(100vh - 120px - max(env(safe-area-inset-top, 0px), 28px) - max(env(safe-area-inset-bottom, 0px), 64px));
    background: #fff;
    border-radius: 20px;
    box-shadow: 0 24px 80px rgba(0, 0, 0, 0.2),
                0 0 0 1px rgba(0, 0, 0, 0.04);
    overflow: hidden;
    display: flex;
    flex-direction: column;
    animation: modalScaleIn 0.4s cubic-bezier(0.32, 0.72, 0, 1);

    .modal-tabs {
        display: flex;
        gap: 4px;
        padding: 8px 16px;
        background: rgba(118, 118, 128, 0.08);
        margin: 0 16px;
        margin-top: 8px;
        border-radius: 10px;

        button {
            flex: 1;
            padding: 8px 12px;
            background: transparent;
            border: none;
            border-radius: 8px;
            font-size: 14px;
            font-weight: 500;
            color: #666;
            cursor: pointer;
            transition: all 0.25s cubic-bezier(0.32, 0.72, 0, 1);

            &:active {
                transform: scale(0.97);
            }

            &.active {
                color: #1a1a1a;
                background: white;
                box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
            }
        }
    }

    &.dark-mode {
        background: #2e3440;
        color: #d8dee9;

        .modal-header {
            background: transparent;
            border-color: rgba(255, 255, 255, 0.1);

            .close-btn {
                background: rgba(255, 255, 255, 0.1);
                color: #999;

                &:hover {
                    background: rgba(255, 255, 255, 0.15);
                    color: #d8dee9;
                }
            }
        }

        .modal-tabs {
            background: rgba(255, 255, 255, 0.08);
            border-color: #4c566a;

            button {
                color: #999;

                &:hover {
                    color: #d8dee9;
                    background: rgba(255, 255, 255, 0.05);
                }

                &.active {
                    color: #88c0d0;
                    background: rgba(255, 255, 255, 0.1);
                    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
                }
            }
        }

        .tag-input {
            background: #3b4252;
            border-color: #4c566a;
            color: #d8dee9;

            &::placeholder {
                color: #888;
            }
        }

        h3 {
            color: #6FB5B5;
        }

        .tag-item {
            background: rgba(255, 255, 255, 0.05);
            border-color: rgba(255, 255, 255, 0.1);
        }

        .no-tags {
            color: #888;
        }

        .delete-confirm-modal {
            background: #3b4252;
            color: #d8dee9;
        }

        .edit-btn {
            background: rgba(111, 181, 181, 0.15);
            color: #6FB5B5;

            &:hover {
                background: rgba(111, 181, 181, 0.25);
            }
        }

        .save-btn {
            background: rgba(111, 181, 181, 0.2);
            color: #88c0d0;

            &:hover {
                background: rgba(111, 181, 181, 0.3);
            }
        }

        .cancel-btn {
            background: rgba(255, 255, 255, 0.08);
            color: #999;

            &:hover {
                background: rgba(255, 255, 255, 0.12);
            }
        }

        .add-btn {
            background: linear-gradient(135deg, #5a9b9b, #4A8B8B);

            &:hover:not(:disabled) {
                background: linear-gradient(135deg, #4A8B8B, #3D7A7A);
            }
        }
    }

    .modal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 16px 20px;
        background: transparent;
        border-bottom: 1px solid rgba(0, 0, 0, 0.08);

        h2 {
            margin: 0;
            font-size: 17px;
            font-weight: 600;
            letter-spacing: -0.2px;
        }

        .close-btn {
            width: 30px;
            height: 30px;
            background: rgba(0, 0, 0, 0.06);
            border: none;
            border-radius: 50%;
            font-size: 18px;
            cursor: pointer;
            color: #666;
            line-height: 1;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.2s;

            &:hover {
                background: rgba(0, 0, 0, 0.1);
                color: #333;
            }

            &:active {
                transform: scale(0.95);
            }
        }
    }

    .modal-content {
        padding: 1.5rem;
        overflow-y: auto;
        flex: 1;
    }

    h3 {
        font-size: 0.85rem;
        color: #666;
        margin: 0 0 0.75rem 0;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .add-tag-section {
        margin-bottom: 1rem;
    }

    .add-tag-form {
        display: flex;
        gap: 8px;
        align-items: center;
        width: 100%;
        box-sizing: border-box;
    }

    .tag-input {
        flex: 1;
        min-width: 0;
        padding: 0.6rem 0.8rem;
        border: 1px solid #ddd;
        border-radius: 10px;
        font-size: 0.9rem;
        outline: none;
        transition: border-color 0.2s;
        box-sizing: border-box;

        &:focus {
            border-color: #4A8B8B;
        }

        &.small {
            padding: 0.4rem 0.6rem;
            font-size: 0.85rem;
        }
    }

    .color-picker-wrapper {
        position: relative;
        flex-shrink: 0;
    }

    .color-picker {
        width: 36px;
        height: 36px;
        border: none;
        border-radius: 8px;
        cursor: pointer;
        padding: 0;

        &.small {
            width: 30px;
            height: 30px;
        }
    }

    .add-btn {
        padding: 0.6rem 1rem;
        background: linear-gradient(135deg, #4A8B8B, #3D7A7A);
        color: white;
        border: none;
        border-radius: 10px;
        cursor: pointer;
        font-weight: 500;
        transition: all 0.2s;
        flex-shrink: 0;
        white-space: nowrap;

        &:hover:not(:disabled) {
            transform: translateY(-1px);
            background: linear-gradient(135deg, #3D7A7A, #336969);
        }

        &:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
    }

    .error-msg {
        color: #d32f2f;
        font-size: 0.85rem;
        margin-top: 0.5rem;
    }

    .color-presets {
        display: flex;
        gap: 6px;
        margin-bottom: 1.5rem;
        flex-wrap: wrap;

        .preset-color {
            width: 24px;
            height: 24px;
            border-radius: 50%;
            border: 2px solid transparent;
            cursor: pointer;
            transition: all 0.2s;

            &:hover {
                transform: scale(1.1);
            }

            &.selected {
                border-color: #333;
                transform: scale(1.15);
            }
        }
    }

    .tags-list-section {
        margin-top: 1rem;
    }

    .tags-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .tag-item {
        padding: 0.75rem;
        background: rgba(0, 0, 0, 0.03);
        border: 1px solid rgba(0, 0, 0, 0.08);
        border-radius: 12px;
    }

    .tag-display {
        display: flex;
        align-items: center;
        gap: 0.75rem;

        .tag-color {
            width: 16px;
            height: 16px;
            border-radius: 50%;
            flex-shrink: 0;
        }

        .tag-name {
            flex: 1;
            font-weight: 500;
        }

        .tag-actions {
            display: flex;
            gap: 0.5rem;
        }
    }

    .tag-edit {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .edit-btn, .delete-btn, .save-btn, .cancel-btn {
        padding: 0.35rem 0.7rem;
        border: none;
        border-radius: 8px;
        font-size: 0.8rem;
        cursor: pointer;
        transition: all 0.2s;
    }

    .edit-btn {
        background: rgba(74, 139, 139, 0.12);
        color: #4A8B8B;

        &:hover {
            background: rgba(74, 139, 139, 0.2);
        }
    }

    .delete-btn {
        background: #ffebee;
        color: #d32f2f;

        &:hover {
            background: #ffcdd2;
        }
    }

    .save-btn {
        background: rgba(74, 139, 139, 0.15);
        color: #3D7A7A;

        &:hover {
            background: rgba(74, 139, 139, 0.25);
        }
    }

    .cancel-btn {
        background: #f5f5f5;
        color: #666;

        &:hover {
            background: #e0e0e0;
        }
    }

    .no-tags {
        text-align: center;
        color: #999;
        padding: 2rem;
    }
}

.delete-confirm-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 20px;
}

.delete-confirm-modal {
    background: white;
    padding: 1.5rem;
    border-radius: 16px;
    text-align: center;
    max-width: 280px;

    p {
        margin: 0 0 0.5rem 0;
    }

    .delete-warning {
        font-size: 0.85rem;
        color: #666;
        margin-bottom: 1rem;
    }

    .confirm-buttons {
        display: flex;
        gap: 0.75rem;
        justify-content: center;

        button {
            padding: 0.5rem 1.2rem;
            border: none;
            border-radius: 10px;
            cursor: pointer;
            font-weight: 500;
        }

        .confirm-delete-btn {
            background: #d32f2f;
            color: white;

            &:hover {
                background: #c62828;
            }
        }
    }
}
</style>
