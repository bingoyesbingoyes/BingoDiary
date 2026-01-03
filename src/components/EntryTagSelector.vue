<template>
    <div class="entry-tag-selector" :class="{ 'dark-mode': darkTheme }">
        <div class="tags-header">
            <div class="tags-display">
                <span
                    v-for="tag in selectedTags"
                    :key="tag.id"
                    class="tag-pill"
                    :style="{ background: tag.color }"
                >
                    {{ tag.name }}
                </span>
                <span v-if="selectedTags.length === 0" class="no-tags-text">{{ t('tags.noTags') }}</span>
            </div>
            <button class="add-tag-btn" @click="toggleDropdown" :title="showDropdown ? t('common.close') : t('tags.addTags')">
                <span :class="{ 'rotate': showDropdown }">+</span>
            </button>
        </div>

        <div v-if="showDropdown" class="dropdown" @click.stop>
            <div class="dropdown-content">
                <div v-if="availableTags.length === 0" class="no-tags-hint">
                    {{ t('tags.noTagsHint') }}
                </div>
                <label
                    v-for="tag in availableTags"
                    :key="tag.id"
                    class="tag-option"
                    :class="{ 'selected': isSelected(tag.id) }"
                >
                    <input
                        type="checkbox"
                        :checked="isSelected(tag.id)"
                        @change="toggleTag(tag.id)"
                    />
                    <span class="tag-color" :style="{ background: tag.color }"></span>
                    <span class="tag-name">{{ tag.name }}</span>
                    <span v-if="isSelected(tag.id)" class="check-icon">✓</span>
                </label>
            </div>
        </div>

        <div v-if="showDropdown" class="dropdown-backdrop" @click="closeDropdown"></div>
    </div>
</template>

<script>
export default {
    name: 'EntryTagSelector',
    props: {
        availableTags: {
            type: Array,
            default: () => []
        },
        selectedTags: {
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
    emits: ['update'],
    data() {
        return {
            showDropdown: false
        };
    },
    methods: {
        toggleDropdown() {
            this.showDropdown = !this.showDropdown;
        },
        closeDropdown() {
            this.showDropdown = false;
        },
        isSelected(tagId) {
            return this.selectedTags.some(t => t.id === tagId);
        },
        toggleTag(tagId) {
            const currentIds = this.selectedTags.map(t => t.id);
            let newIds;

            if (currentIds.includes(tagId)) {
                newIds = currentIds.filter(id => id !== tagId);
            } else {
                newIds = [...currentIds, tagId];
            }

            this.$emit('update', newIds);
        }
    }
}
</script>

<style lang="scss" scoped>
.entry-tag-selector {
    position: relative;

    .tags-header {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 0;
        background: transparent;
        min-height: auto;
    }

    .tags-display {
        display: flex;
        flex-wrap: wrap;
        gap: 6px;
        flex: 1;
        align-items: center;

        .tag-pill {
            padding: 3px 8px;
            border-radius: 10px;
            font-size: 11px;
            font-weight: 500;
            color: white;
            text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
            box-shadow: 0 1px 4px rgba(0, 0, 0, 0.12);
            transition: transform 0.15s, box-shadow 0.15s;

            &:hover {
                transform: translateY(-1px);
                box-shadow: 0 2px 6px rgba(0, 0, 0, 0.18);
            }
        }

        .no-tags-text {
            color: #aaa;
            font-size: 12px;
            font-style: italic;
        }
    }

    .add-tag-btn {
        width: 22px;
        height: 22px;
        border: none;
        background: rgba(0, 0, 0, 0.06);
        border-radius: 6px;
        cursor: pointer;
        font-size: 14px;
        font-weight: 300;
        color: #666;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: all 0.2s;
        flex-shrink: 0;

        &:hover {
            background: rgba(0, 0, 0, 0.1);
            color: #333;
        }

        span {
            transition: transform 0.2s;
            display: block;

            &.rotate {
                transform: rotate(45deg);
            }
        }
    }

    .dropdown-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        z-index: 99;
    }

    .dropdown {
        position: absolute;
        bottom: 100%;
        left: 0;
        min-width: 200px;
        margin-bottom: 8px;
        background: white;
        border-radius: 12px;
        box-shadow: 0 -4px 24px rgba(0, 0, 0, 0.15);
        z-index: 100;
        overflow: hidden;
        animation: dropdownUp 0.15s ease-out;
    }

    @keyframes dropdownUp {
        from {
            opacity: 0;
            transform: translateY(8px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .dropdown-content {
        max-height: 220px;
        overflow-y: auto;
        padding: 10px;
    }

    .no-tags-hint {
        padding: 16px;
        text-align: center;
        color: #999;
        font-size: 13px;
    }

    .tag-option {
        display: flex;
        align-items: center;
        gap: 10px;
        padding: 10px 12px;
        border-radius: 10px;
        cursor: pointer;
        transition: all 0.15s;
        position: relative;

        &:hover {
            background: rgba(0, 0, 0, 0.05);
        }

        &.selected {
            background: rgba(25, 118, 210, 0.08);
        }

        input[type="checkbox"] {
            width: 16px;
            height: 16px;
            cursor: pointer;
            accent-color: #1976D2;
        }

        .tag-color {
            width: 14px;
            height: 14px;
            border-radius: 50%;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
        }

        .tag-name {
            font-size: 13px;
            flex: 1;
        }

        .check-icon {
            font-size: 12px;
            color: #1976D2;
            font-weight: bold;
        }
    }

    // Dark mode
    &.dark-mode {
        .tags-header {
            background: rgba(255, 255, 255, 0.05);
        }

        .tags-display .no-tags-text {
            color: #666;
        }

        .add-tag-btn {
            background: rgba(255, 255, 255, 0.08);
            color: #999;

            &:hover {
                background: rgba(255, 255, 255, 0.12);
                color: #ccc;
            }
        }

        .dropdown {
            background: #3b4252;
        }

        .tag-option {
            color: #d8dee9;

            &:hover {
                background: rgba(255, 255, 255, 0.08);
            }

            &.selected {
                background: rgba(136, 192, 208, 0.15);
            }

            .check-icon {
                color: #88c0d0;
            }
        }

        .no-tags-hint {
            color: #888;
        }
    }
}
</style>
