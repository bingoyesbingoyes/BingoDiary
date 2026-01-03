<template>
    <div class="tag-bar" :class="{ 'dark-mode': darkTheme }">
        <div class="tag-pills-container">
            <button
                class="tag-pill all-tag"
                :class="{ active: !selectedTagId }"
                @click="$emit('select', null)"
            >
                All
            </button>
            <button
                v-for="tag in tags"
                :key="tag.id"
                class="tag-pill"
                :class="{ active: selectedTagId === tag.id }"
                :style="{ '--tag-color': tag.color }"
                @click="$emit('select', tag.id)"
            >
                {{ tag.name }}
            </button>
        </div>
        <button class="manage-tags-btn" @click="$emit('manage')" title="Manage Tags">
            +
        </button>
    </div>
</template>

<script>
export default {
    name: 'TagBar',
    props: {
        tags: {
            type: Array,
            default: () => []
        },
        selectedTagId: {
            type: String,
            default: null
        },
        darkTheme: {
            type: Boolean,
            default: false
        }
    },
    emits: ['select', 'manage']
}
</script>

<style lang="scss" scoped>
.tag-bar {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 16px 0;
    margin-bottom: 12px;

    .tag-pills-container {
        display: flex;
        flex-wrap: wrap;
        gap: 10px;
        flex: 1;
        padding: 0;
    }

    .tag-pill {
        padding: 4px 10px;
        border-radius: 12px;
        border: none;
        font-size: 11px;
        font-weight: 500;
        cursor: pointer;
        white-space: nowrap;
        transition: all 0.2s ease;
        background: var(--tag-color, #e0e0e0);
        color: white;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);

        &.all-tag {
            background: rgba(0, 0, 0, 0.08);
            color: #333;
            text-shadow: none;
        }

        &:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        }

        &.active {
            transform: scale(1.05);
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
            ring: 2px solid white;
        }
    }

    .manage-tags-btn {
        width: 24px;
        height: 24px;
        border-radius: 8px;
        border: none;
        background: rgba(0, 0, 0, 0.06);
        font-size: 14px;
        font-weight: 400;
        cursor: pointer;
        flex-shrink: 0;
        color: rgba(0, 0, 0, 0.5);
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        justify-content: center;

        &:hover {
            background: rgba(0, 0, 0, 0.1);
            color: rgba(0, 0, 0, 0.7);
        }
    }

    // Dark mode
    &.dark-mode {
        .tag-pill {
            &.all-tag {
                background: rgba(255, 255, 255, 0.1);
                color: #ddd;
            }
        }

        .manage-tags-btn {
            background: rgba(255, 255, 255, 0.08);
            color: rgba(255, 255, 255, 0.5);

            &:hover {
                background: rgba(255, 255, 255, 0.15);
                color: rgba(255, 255, 255, 0.8);
            }
        }
    }
}
</style>
